use luoshu::data::{
    ActionEnum, ConfigurationReg, Connection, LuoshuDataEnum, LuoshuDataHandle, LuoshuMemData,
    ServiceReg, Subscribe,
};
use luoshu_registry::Service;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyFunction;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex as std_Mutex};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;

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
    subscribe_book: Arc<std_Mutex<HashMap<String, Sender<ConfigurationReg>>>>,
    subscribe_sender: UnboundedSender<Subscribe>,
    subscribe_receiver: Arc<Mutex<UnboundedReceiver<Subscribe>>>,
}

#[pymethods]
impl Luoshu {
    #[new]
    fn new(namespace: String, name: String, host: String, port: u16) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Subscribe>();
        Self {
            namespace,
            name,
            host,
            port,
            subscribe_book: Arc::new(std_Mutex::new(HashMap::new())),
            subscribe_sender: tx,
            subscribe_receiver: Arc::new(Mutex::new(rx)),
        }
    }

    pub fn config_subscribe(
        self_: PyRefMut<'_, Self>,
        py: Python<'_>,
        namespace: String,
        config_name: String,
        callback: PyObject,
    ) -> PyResult<()> {
        let func = callback.cast_as::<PyFunction>(py)?;
        if func.is_callable() {
            let (tx, rx) = channel::<ConfigurationReg>();
            let subscribe = Subscribe::new(namespace, config_name);
            self_
                .subscribe_book
                .lock()
                .unwrap()
                .insert(subscribe.to_string(), tx);
            self_
                .subscribe_sender
                .send(subscribe)
                .expect("callback error");
            loop {
                if let Ok(config) = rx.recv() {
                    let value = serde_json::to_string(&config).expect("callback error");
                    func.call((value,), None).expect("callback error");
                };
            }
        }
        Ok(())
    }

    pub fn process<'p>(self_: PyRef<'p, Self>, py: Python<'p>) -> PyResult<&'p PyAny> {
        let namespace = self_.namespace.clone();
        let name = self_.name.clone();
        let host = self_.host.clone();
        let port = self_.port;
        let subscribe_book = self_.subscribe_book.clone();
        let subscribe_receiver = self_.subscribe_receiver.clone();
        let callback = move |x: ConfigurationReg| {
            if let Some(sender) = subscribe_book.lock().unwrap().get(&x.get_subscribe_str()) {
                sender.send(x).expect("callback error");
            }
        };
        pyo3_asyncio::tokio::future_into_py(py, async move {
            process(namespace, name, host, port, callback, subscribe_receiver)
                .await
                .map_err(|e| exceptions::PyException::new_err(e.to_string()))?;
            Ok(Python::with_gil(|py| py.None()))
        })
    }
}

async fn process<T: Fn(ConfigurationReg)>(
    namespace: String,
    name: String,
    host: String,
    port: u16,
    callback: T,
    subscribe_receiver: Arc<Mutex<UnboundedReceiver<Subscribe>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = Arc::new(Mutex::new(LuoshuMemData::new()));
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
    let mut subscribe_receiver = subscribe_receiver.lock().await;
    loop {
        tokio::select! {
            Some(subscribe) = subscribe_receiver.recv()=>{
                connection.write_frame(&ActionEnum::Subscribe(subscribe).into()).await?;
            }
            Ok(Some(frame)) = connection.read_frame() => {
                match frame.data {
                    ActionEnum::Up(frame) => data.lock().await.append(&frame, None, None).await?,
                    ActionEnum::Down(frame) => data.lock().await.remove(&frame).await?,
                    ActionEnum::Sync(frame) => {
                        eprintln!("Sync {:#?}", frame);
                       match frame.clone() {
                            luoshu::data::LuoshuSyncDataEnum::LuoshuData(LuoshuDataEnum::Configuration(config))=>callback(config),
                           _ => todo!(),
                       };
                        data.lock().await.sync(&frame).await?;
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
