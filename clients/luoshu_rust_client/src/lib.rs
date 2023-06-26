//! client for luoshu
#![deny(missing_docs)]

mod error;

use anyhow::anyhow;
use error::LuoshuClientResult;
use luoshu::data::{
    ActionEnum, ConfigurationReg, Connection, LuoshuDataEnum, LuoshuDataHandle, LuoshuMemData,
    LuoshuSyncDataEnum, Service, ServiceReg, Subscribe,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

/// 洛书客户端
///
/// 订阅配置信息，并注册服务到洛书
/// ```
/// use std::thread::sleep;
/// use luoshu_rust_client::LuoshuClient;
///
/// #[derive(Debug, Serialize, Deserialize, Clone)]
/// struct Config {
///     test1: String,
///     test2: Vec<usize>,
/// }
///
/// #[tokio::test]
/// async fn it_works() -> LuoshuClientResult<()> {
///     let mut client = LuoshuClient::new(15666, "test_rust_server".to_string(), None).await;
///     client
///         .subscribe_config(
///             "test_config2".to_string(),
///             |config: Config| println!("config changed:{:#?}", config),
///             None,
///         )
///         .await?;
///     tokio::spawn(async move {
///         client.registry().await.expect("TODO: panic message");
///     });
///     // loop {
///     //     println!("sleep");
///     //     sleep(Duration::from_secs(10))
///     // }
///     Ok(())
/// }
/// ```
pub struct LuoshuClient {
    namespace: String,
    name: String,
    port: u16,
    connection: Connection,
    subscribe_sender: UnboundedSender<Subscribe>,
    subscribe_receiver: UnboundedReceiver<Subscribe>,
    subscribe_book: HashMap<String, Vec<UnboundedSender<ConfigurationReg>>>,
    data: LuoshuMemData,
}

impl LuoshuClient {
    /// 创建洛书客户端
    #[inline]
    pub async fn new(port: u16, name: String, namespace: Option<String>) -> LuoshuClient {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 19998);
        let stream = TcpStream::connect(addr).await.unwrap();
        let connection = Connection::new(stream, addr);

        let (subscribe_sender, subscribe_receiver) = mpsc::unbounded_channel::<Subscribe>();
        Self {
            namespace: namespace.unwrap_or_else(|| String::from("default")),
            name,
            port,
            connection,
            subscribe_sender,
            subscribe_receiver,
            subscribe_book: HashMap::new(),
            data: LuoshuMemData::new(),
        }
    }
    /// 注册服务
    #[inline]
    pub async fn registry(&mut self) -> LuoshuClientResult<()> {
        // 生成服务数据
        let frame = ActionEnum::Up(
            ServiceReg::new(
                self.namespace.clone(),
                self.name.clone(),
                Service::new("127.0.0.1".to_string(), self.port),
            )
            .into(),
        )
        .into();
        self.connection.write_frame(&frame).await?;
        let time_sleep = || async {
            tokio::time::sleep(Duration::from_secs(5)).await;
            true
        };
        loop {
            tokio::select! {
                Some(subscribe) = self.subscribe_receiver.recv()=>{
                    self.connection.write_frame(&ActionEnum::Subscribe(subscribe).into()).await?;
                }
                Ok(Some(frame)) = self.connection.read_frame() => {
                    match frame.data {
                        ActionEnum::Up(frame) => self.data.append(&frame, None, None).await?,
                        ActionEnum::Down(frame) => self.data.remove(&frame).await?,
                        ActionEnum::Sync(frame) => {
                            if let LuoshuSyncDataEnum::LuoshuData(data) = frame.clone(){
                           if let LuoshuDataEnum::Configuration(config) = data.clone() {
                                if let Some(senders) = self.subscribe_book.get_mut(&config.get_subscribe_str()) {
                                        let mut pre_delete_list = vec![];
                                        for (index, sender) in senders.iter().enumerate() {
                                                match sender.send(config.clone()) {
                                                    Ok(_) =>{},
                                                Err(_) => { pre_delete_list.push(index);},
                                                }
                                    }
                                            for index in pre_delete_list {
                                                senders.remove(index);
                                            }
                                    }
                                }};
                            self.data.sync(&frame).await?;
                        },
                        _ => {}
                    }
                }
                true = time_sleep()=>{
                    self.connection.write_frame(&ActionEnum::Ping.into()).await?;
                }
            }
        }
    }
    /// 订阅配置信息
    pub async fn subscribe_config<F, C>(
        &mut self,
        name: String,
        callback: F,
        namespace: Option<String>,
    ) -> LuoshuClientResult<()>
    where
        F: Fn(C) + Send + 'static,
        C: Serialize + for<'a> Deserialize<'a>,
    {
        let subscribe = Subscribe::new(namespace.unwrap_or_else(|| String::from("default")), name);
        let subscribe_str = subscribe.to_string();
        let (subscribe_sender, mut subscribe_receiver) =
            mpsc::unbounded_channel::<ConfigurationReg>();
        match self.subscribe_book.get_mut(&subscribe_str) {
            None => {
                self.subscribe_book
                    .insert(subscribe_str, vec![subscribe_sender]);
                self.subscribe_sender
                    .send(subscribe)
                    .map_err(|e| anyhow!(e.to_string()))?;
            }
            Some(senders) => {
                senders.push(subscribe_sender);
            }
        };
        tokio::spawn(async move {
            loop {
                if let Some(config) = subscribe_receiver.recv().await {
                    callback(
                        serde_json::from_slice(&serde_json::to_vec(&config.config).unwrap())
                            .unwrap(),
                    );
                }
            }
        });
        Ok(())
    }
    /// 提交配置并订阅相关的配置信息
    #[inline]
    pub async fn submit_config_with_subscribe<F, C>(
        &mut self,
        name: String,
        config: C,
        callback: F,
        namespace: Option<String>,
    ) -> LuoshuClientResult<()>
    where
        F: Fn(C) + Send + 'static,
        C: Serialize + for<'a> Deserialize<'a>,
    {
        let namespace = namespace.unwrap_or_else(|| String::from("default"));
        self.connection
            .write_frame(
                &ActionEnum::Up(
                    ConfigurationReg::new(
                        namespace.clone(),
                        name.clone(),
                        serde_json::from_slice(&serde_json::to_vec(&config)?)?,
                    )
                    .into(),
                )
                .into(),
            )
            .await?;
        self.subscribe_config(name, callback, Some(namespace)).await
    }
    /// 提交配置
    #[inline]
    pub async fn submit_config<C>(
        &mut self,
        name: String,
        config: C,
        namespace: Option<String>,
    ) -> LuoshuClientResult<()>
    where
        C: Serialize + for<'a> Deserialize<'a>,
    {
        let namespace = namespace.unwrap_or_else(|| String::from("default"));
        self.connection
            .write_frame(
                &ActionEnum::Up(
                    ConfigurationReg::new(
                        namespace,
                        name,
                        serde_json::from_slice(&serde_json::to_vec(&config)?)?,
                    )
                    .into(),
                )
                .into(),
            )
            .await
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use std::thread::sleep;

    // #[derive(Debug, Serialize, Deserialize, Clone)]
    // struct Config {
    //     test1: String,
    //     test2: Vec<usize>,
    // }

    // #[tokio::test]
    // async fn it_works() -> LuoshuClientResult<()> {
    //     let mut client = LuoshuClient::new(15666, "test_rust_server".to_string(), None).await;
    //     client
    //         .subscribe_config(
    //             "test_config2".to_string(),
    //             |config: Config| println!("config changed:{:#?}", config),
    //             None,
    //         )
    //         .await?;
    //     tokio::spawn(async move {
    //         client.registry().await.expect("TODO: panic message");
    //     });
    //     // loop {
    //     //     println!("sleep");
    //     //     sleep(Duration::from_secs(10))
    //     // }
    //     Ok(())
    // }
}
