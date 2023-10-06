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

use file_service::*;
pub mod file_service {
    tonic::include_proto!("file_service");
}


#[derive(Default)]
pub struct FileService {
    pub root_path: PathBuf,
    suffixed_root_path: String,
}

impl FileService {
    pub fn new(root_path: PathBuf) -> Self {
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

    pub fn find_all_files(&self) -> impl Stream<Item = io::Result<ShardFile>> + Send + '_ {
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
