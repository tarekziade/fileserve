use futures::{stream, Stream, StreamExt}; // 0.3.1
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::types::PyList;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::{io, path::PathBuf};
use tokio::fs; // 0.2.4
use tokio::fs::File as TokioFile;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

pub mod finder;

use file_service::*;
use file_service::shard_file_service_server::{ShardFileService, ShardFileServiceServer};

use finder::*;

#[tonic::async_trait]
impl ShardFileService for FileService {
    async fn get_shard_files(
        &self,
        request: Request<GetShardFilesRequest>,
    ) -> Result<Response<GetShardFilesResponse>, Status> {
        let _request = request.into_inner();

        let paths = self.find_all_files();
        let files = Arc::new(Mutex::new(vec![]));

        paths
            .for_each(|entry| {
                let files = Arc::clone(&files);
                async move {
                    if let Ok(shard_file) = entry {
                        let mut files = files.lock().unwrap();
                        files.push(shard_file);
                    }
                }
            })
            .await;

        let files = Arc::try_unwrap(files).unwrap().into_inner().unwrap();
        let resp = GetShardFilesResponse {
            total: files.len() as i32,
            files,
        };

        Ok(Response::new(resp))
    }

    type DownloadShardFileStream =
        Pin<Box<dyn Stream<Item = Result<DownloadShardFileResponse, Status>> + Send + Sync>>;

    async fn download_shard_file(
        &self,
        request: Request<DownloadShardFileRequest>,
    ) -> Result<Response<Self::DownloadShardFileStream>, Status> {
        let request = request.into_inner();
        let path = self.root_path.join(request.relative_path);
        let (tx, rx) = mpsc::channel(32);

        tokio::spawn(async move {
            let file = TokioFile::open(&path)
                .await
                .map_err(|_| Status::not_found("File not found"))
                .unwrap();

            let mut reader = BufReader::new(file);
            let mut buffer = vec![0; 1024 * 1024];

            let mut idx = 0;
            println!("Sending {}", path.display());
            loop {
                match reader.read(&mut buffer).await {
                    Ok(0) => {
                        break;
                    }
                    Ok(bytes_read) => {
                        let chunk = DownloadShardFileResponse {
                            chunk_data: buffer[..bytes_read].to_vec(),
                            chunk_index: idx,
                        };
                        println!("{} sending chunk {}", path.display(), idx);
                        tx.send(Ok(chunk)).await.expect("send failed");
                        println!("{} sent chunk {}", path.display(), idx);
                        idx += 1;
                    }
                    Err(_) => {
                        tx.send(Err(Status::internal("Read error")))
                            .await
                            .expect("send failed");
                        break;
                    }
                }
            }
        });

        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as Self::DownloadShardFileStream
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let file_service =
        FileService::new("/Users/tarekziade/Dev/nucliadb/vectors_benchmark/datasets/large".into());
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(ShardFileServiceServer::new(file_service))
        .serve(addr)
        .await?;

    Ok(())
}
