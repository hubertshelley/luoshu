use luoshu::data::{
    ActionEnum, ConfigurationReg, Connection, LuoshuDataEnum, LuoshuDataHandle, LuoshuMemData,
    ServiceReg,
};
use luoshu_registry::Service;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyFunction;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyclass]
struct Luoshu {
    namespace: String,
    name: String,
    host: String,
    port: u16,
    subscribe_book: HashMap<String, PyObject>,
}

#[pymethods]
impl Luoshu {
    #[new]
    fn new(namespace: String, name: String, host: String, port: u16) -> Self {
        Self {
            namespace,
            name,
            host,
            port,
            subscribe_book: HashMap::new(),
        }
    }

    pub fn config_subscribe(
        mut self_: PyRefMut<'_, Self>,
        py: Python<'_>,
        config_name: String,
        callback: PyObject,
    ) -> PyResult<()> {
        let func = callback.cast_as::<PyFunction>(py)?;
        if func.is_callable() {
            self_.subscribe_book.insert(config_name, callback);
        }
        Ok(())
    }

    pub fn process(self_: PyRef<'_, Self>, py: Python<'_>) -> PyResult<()> {
        let namespace = self_.namespace.clone();
        let name = self_.name.clone();
        let host = self_.host.clone();
        let port = self_.port;
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<ConfigurationReg>();
        // let subscribe_book = self_.subscribe_book.clone();
        let callback = move |x| {
            tx.send(x).expect("callback error");
        };
        pyo3_asyncio::tokio::future_into_py(py, async move {
            tokio::task::spawn(async move {
                loop {
                    if let Some(config) = rx.recv().await {
                        println!(
                            "{}",
                            serde_json::to_string(&config).expect("callback error")
                        );
                        // if let Some(c) = subscribe_book.get(config.get_subscribe_str().as_str()) {
                        //     let func = c.cast_as::<PyFunction>(py).expect("callback error");
                        //     let value = serde_json::to_string(&config).expect("callback error");
                        //     func.call((value, ), None).expect("callback error");
                        // }
                    }
                }
            });
            process(namespace, name, host, port, callback)
                .await
                .map_err(|e| exceptions::PyException::new_err(e.to_string()))?;
            Ok(())
        })?;
        Ok(())
    }
}

async fn process<T: Fn(ConfigurationReg)>(
    namespace: String,
    name: String,
    host: String,
    port: u16,
    callback: T,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = Arc::new(RwLock::new(LuoshuMemData::new()));
    let addr = "127.0.0.1:19998".to_string();
    let stream = TcpStream::connect(addr.clone()).await.unwrap();
    let mut connection = Connection::new(stream, addr.parse()?);
    // 生成服务数据
    let frame =
        ActionEnum::Up(ServiceReg::new(namespace, name, Service::new(host, port)).into()).into();
    connection.write_frame(&frame).await?;
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
                    ActionEnum::Sync(frame) => {
                       match frame.clone() {
                            LuoshuDataEnum::Configuration(config)=>callback(config),
                           _ => todo!(),
                       };
                        data.write().await.sync(&frame).await?;
                    },
                    _ => {}
                }
            }
            true = time_sleep()=>{
                connection.write_frame(&ActionEnum::Ping.into()).await?;
            }
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn luoshu_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Luoshu>()?;
    Ok(())
}
