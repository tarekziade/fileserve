use futures::{stream, Stream, StreamExt}; // 0.3.1
use pyo3::exceptions::PyStopAsyncIteration;
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
use tokio::time::{sleep, Duration};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

pub mod finder;
use file_service::*;
use finder::*;

#[pyclass]
struct PyAsyncIter {
    service: Pin<Box<FileService>>,
    stream: Arc<Mutex<Pin<Box<dyn Stream<Item = io::Result<ShardFile>> + Send + 'static>>>>,
}

#[pymethods]
impl PyAsyncIter {
    #[new]
    pub fn new() -> Self {
        let service = FileService::new(
            "/Users/tarekziade/Dev/nucliadb/vectors_benchmark/datasets/large".into(),
        );
        let stream = Arc::new(Mutex::new(Box::pin(service.find_all_files())));

        PyAsyncIter {
            service: Box::pin(service),
            stream: stream,
        }
    }

    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__(slf: PyRefMut<'_, Self>) -> PyResult<Option<PyObject>> {
        let stream = Arc::clone(&slf.stream);

        let fut = pyo3_asyncio::tokio::future_into_py(slf.py(), async move {
            let mut stream = slf.stream.lock().unwrap();
            match stream.as_mut().next().await {
                Some(Ok(shard_file)) => Python::with_gil(|py| Ok(Some(shard_file.path))),
                Some(Err(_)) => Err(pyo3::exceptions::PyIOError::new_err("Stream error")),
                None => Err(PyStopAsyncIteration::new_err("Async iterator is exhausted")),
            }
        })?;
        Ok(Some(fut.into()))
    }
}

#[pyfunction]
fn get_shard_files(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let list = vec![1, 2, 3];
        println!("list");

        sleep(Duration::from_millis(100)).await;
        println!("sleep");
        //py_sleep.await?;
        sleep(Duration::from_millis(100)).await;
        println!("sleep");

        Ok(Python::with_gil(|py| list))
    })
}

#[pymodule]
fn downloader(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_shard_files, m)?)?;
    m.add_class::<PyAsyncIter>()?;
    Ok(())
}
