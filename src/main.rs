use futures::{stream, Stream, StreamExt}; // 0.3.1
use std::sync::{Arc, Mutex};
use std::{io, path::PathBuf};
use tokio::fs; // 0.2.4
use tonic::{transport::Server, Request, Response, Status, Streaming};

use file_service::shard_file_service_server::{ShardFileService, ShardFileServiceServer};
use file_service::*;

pub mod file_service {
    tonic::include_proto!("file_service");
}

fn find_all_files(
    path: impl Into<PathBuf>,
) -> impl Stream<Item = io::Result<ShardFile>> + Send + 'static {
    async fn one_level(path: PathBuf, to_visit: &mut Vec<PathBuf>) -> io::Result<Vec<ShardFile>> {
        let mut dir = fs::read_dir(path).await?;
        let mut files = Vec::new();

        while let Some(child) = dir.next_entry().await? {
            if child.metadata().await?.is_dir() {
                to_visit.push(child.path());
            } else {
                if let Ok(metadata) = child.metadata().await {
                    let path = child.path().into_os_string().into_string().unwrap();
                    files.push(ShardFile {
                        path: path,
                        size: metadata.len(),
                    });
                }
            }
        }
        Ok(files)
    }

    stream::unfold(vec![path.into()], |mut to_visit| async {
        let path = to_visit.pop()?;
        let file_stream = match one_level(path, &mut to_visit).await {
            Ok(files) => stream::iter(files).map(Ok).left_stream(),
            Err(e) => stream::once(async { Err(e) }).right_stream(),
        };

        Some((file_stream, to_visit))
    })
    .flatten()
}

#[derive(Default)]
pub struct MyFileService {
    // ..
}

#[tonic::async_trait]
impl ShardFileService for MyFileService {
    async fn get_shard_files(
        &self,
        request: Request<GetShardFilesRequest>,
    ) -> Result<Response<GetShardFilesResponse>, Status> {
        let request = request.into_inner();

        let root_path = "/tmp";
        let paths = find_all_files(root_path);
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let file_service = MyFileService::default();
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(ShardFileServiceServer::new(file_service))
        .serve(addr)
        .await?;

    Ok(())
}
