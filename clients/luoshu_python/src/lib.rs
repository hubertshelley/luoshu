use luoshu::data::{
    ActionEnum, Connection, LuoshuDataHandle, LuoshuMemData, ServiceReg, Subscribe,
};
use luoshu_registry::Service;
use pyo3::exceptions;
use pyo3::prelude::*;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

async fn process(
    namespace: String,
    name: String,
    host: String,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = Arc::new(RwLock::new(LuoshuMemData::new()));
    let addr = "127.0.0.1:19998".to_string();
    let stream = TcpStream::connect(addr.clone()).await.unwrap();
    let mut connection = Connection::new(stream, addr.parse()?);
    // 生成服务数据
    let frame =
        ActionEnum::Up(ServiceReg::new(namespace, name, Service::new(host, port)).into()).into();
    connection.write_frame(&frame).await?;
    connection
        .write_frame(
            &ActionEnum::Subscribe(Subscribe::new("default".to_string(), "test".to_string()))
                .into(),
        )
        .await?;
    let time_sleep = || async {
        tokio::time::sleep(Duration::from_secs(5)).await;
        true
    };
    loop {
        tokio::select! {
            Ok(Some(frame)) = connection.read_frame() => {
                match frame.data {
                    ActionEnum::Up(frame) => data.write().await.append(&frame, None).await?,
                    ActionEnum::Down(frame) => data.write().await.remove(&frame).await?,
                    ActionEnum::Sync(frame) => data.write().await.sync(&frame).await?,
                    _ => {}
                }
            }
            true = time_sleep()=>{
                connection.write_frame(&ActionEnum::Ping.into()).await?;
            }
        }
    }
}

#[pyfunction]
fn sleep(py: Python, namespace: String, name: String, host: String, port: u16) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        process(namespace, name, host, port)
            .await
            .map_err(|e| exceptions::PyException::new_err(e.to_string()))?;
        Ok(Python::with_gil(|py| py.None()))
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn luoshu_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sleep, m)?)?;
    Ok(())
}
