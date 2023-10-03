use futures::{stream, Stream, StreamExt}; // 0.3.1
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::{io, path::PathBuf};
use tokio::fs; // 0.2.4
use tokio::fs::File as TokioFile;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

use file_service::shard_file_service_server::{ShardFileService, ShardFileServiceServer};
use file_service::*;

pub mod file_service {
    tonic::include_proto!("file_service");
}

#[derive(Default)]
pub struct FileService {
    root_path: PathBuf,
    suffixed_root_path: String,
}

impl FileService {
    fn new(root_path: PathBuf) -> Self {
        let suffixed = root_path.to_str().unwrap().to_owned() + "/";
        FileService {
            root_path,
            suffixed_root_path: suffixed.to_owned(),
        }
    }

    async fn one_level(
        &self,
        path: PathBuf,
        to_visit: &mut Vec<PathBuf>,
    ) -> io::Result<Vec<ShardFile>> {
        let mut dir = fs::read_dir(path).await?;
        let mut files = Vec::new();

        while let Some(child) = dir.next_entry().await? {
            if child.metadata().await?.is_dir() {
                to_visit.push(child.path());
            } else if let Ok(metadata) = child.metadata().await {
                let path = child.path().into_os_string().into_string().unwrap();
                if let Some(path) = path.strip_prefix(&self.suffixed_root_path) {
                    files.push(ShardFile {
                        path: path.to_string(),
                        size: metadata.len(),
                    });
                }
            }
        }
        Ok(files)
    }

    fn find_all_files(&self) -> impl Stream<Item = io::Result<ShardFile>> + Send + '_ {
        stream::unfold(vec![self.root_path.clone().into()], |mut to_visit| async {
            let path = to_visit.pop()?;
            let file_stream = match self.one_level(path, &mut to_visit).await {
                Ok(files) => stream::iter(files).map(Ok).left_stream(),
                Err(e) => stream::once(async { Err(e) }).right_stream(),
            };

            Some((file_stream, to_visit))
        })
        .flatten()
    }
}

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

        let file = TokioFile::open(&path)
            .await
            .map_err(|_| Status::not_found("File not found"))?;

        let mut reader = BufReader::new(file);
        let mut buffer = vec![0; 1024 * 1024];

        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => break, // EOF
                Ok(bytes_read) => {
                    let chunk = DownloadShardFileResponse {
                        chunk_data: buffer[..bytes_read].to_vec(),
                    };
                    println!("Sending chunk for file {}", path.display());
                    tx.send(Ok(chunk)).await.expect("send failed");
                }
                Err(_) => {
                    tx.send(Err(Status::internal("Read error")))
                        .await
                        .expect("send failed");
                    break;
                }
            }
        }

        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as Self::DownloadShardFileStream
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let file_service = FileService::new("/Users/tarekziade/Dev/nucliadb/vectors_benchmark/datasets/large".into());
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(ShardFileServiceServer::new(file_service))
        .serve(addr)
        .await?;

    Ok(())
}
