use crate::data::{ActionEnum, Frame, LuoshuDataEnum, LuoshuDataHandle};
use anyhow::Result;
use async_trait::async_trait;
use luoshu_configuration::ConfiguratorStore;
use luoshu_core::Store;
use luoshu_namespace::NamespaceStore;
use luoshu_registry::RegistryStore;
use luoshu_sled_storage::LuoshuSledStorage;
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::mpsc::UnboundedSender;

use super::ServiceReg;

/// 服务器端数据层Sled实现
pub struct LuoshuSledData {
    /// 配置中心存储器
    pub configuration_store: ConfiguratorStore<LuoshuSledStorage>,
    /// 命名空间存储器
    pub namespace_store: NamespaceStore<LuoshuSledStorage>,
    /// 注册中心存储器
    pub service_store: RegistryStore<LuoshuSledStorage>,
    config_subscribers: HashMap<String, Vec<UnboundedSender<Frame>>>,
    service_book: HashMap<SocketAddr, ServiceReg>,
}

impl LuoshuSledData {
    /// 服务器端数据层Sled实现实例化
    pub fn new() -> Self {
        let storage: LuoshuSledStorage = LuoshuSledStorage::default();
        let configuration_store = ConfiguratorStore::new(storage.clone());
        let namespace_store = NamespaceStore::new(storage.clone());
        let service_store = RegistryStore::new(storage);
        LuoshuSledData {
            configuration_store,
            namespace_store,
            service_store,
            config_subscribers: HashMap::new(),
            service_book: HashMap::new(),
        }
    }
}

impl Default for LuoshuSledData {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LuoshuDataHandle for LuoshuSledData {
    async fn append(&mut self, value: &LuoshuDataEnum, client: Option<SocketAddr>) -> Result<()> {
        match value {
            LuoshuDataEnum::Namespace(value) => self.namespace_store.append(value.into())?,
            LuoshuDataEnum::Configuration(value) => {
                match self.config_subscribers.get_mut(&value.name) {
                    None => {}
                    Some(subscribers) => {
                        let mut pre_delete_list = vec![];
                        for (index, subscriber) in subscribers.iter().enumerate() {
                            match subscriber.send(ActionEnum::Sync(value.clone().into()).into()) {
                                Ok(_) => {}
                                Err(_) => {
                                    pre_delete_list.push(index);
                                }
                            };
                        }
                        for delete_index in pre_delete_list {
                            subscribers.remove(delete_index);
                        }
                    }
                }
                self.configuration_store.append(value.into())?
            }
            LuoshuDataEnum::Service(value) => {
                if let Some(client) = client {
                    self.service_book.insert(client, value.clone());
                }
                self.service_store.append(value.into())?
            }
            _ => {}
        };
        Ok(())
    }
    async fn remove(&mut self, value: &LuoshuDataEnum) -> Result<()> {
        match value {
            LuoshuDataEnum::Namespace(value) => self.namespace_store.remove(value.into())?,
            LuoshuDataEnum::Configuration(value) => {
                self.configuration_store.remove(value.into())?
            }
            LuoshuDataEnum::Service(value) => self.service_store.remove(value.into())?,
            _ => {}
        };
        Ok(())
    }

    async fn sync(&mut self, value: &LuoshuDataEnum) -> Result<()> {
        let _ = value;
        Ok(())
    }

    async fn subscribe(&mut self, value: String, sender: UnboundedSender<Frame>) -> Result<()> {
        match self.config_subscribers.get_mut(value.as_str()) {
            None => {
                self.config_subscribers.insert(value, vec![sender]);
            }
            Some(subscribers) => {
                subscribers.push(sender);
            }
        };
        Ok(())
    }
    async fn broken(&mut self, client: SocketAddr) -> Result<()> {
        tracing::info!("连接断开: {}", client);
        if let Some(service) = self.service_book.remove(&client) {
            self.service_store.remove((&service).into())?;
        };
        Ok(())
    }
}
