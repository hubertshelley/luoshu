use crate::data::{ActionEnum, Frame, LuoshuDataEnum, LuoshuDataHandle};
use anyhow::Result;
use async_trait::async_trait;
use luoshu_configuration::ConfiguratorStore;
use luoshu_core::Store;
use luoshu_namespace::NamespaceStore;
use luoshu_registry::RegistryStore;
use luoshu_sled_storage::LuoshuSledStorage;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct LuoshuSledData {
    pub configuration_store: Arc<RwLock<ConfiguratorStore<LuoshuSledStorage>>>,
    pub namespace_store: Arc<RwLock<NamespaceStore<LuoshuSledStorage>>>,
    pub service_store: Arc<RwLock<RegistryStore<LuoshuSledStorage>>>,
    config_subscribers: Arc<RwLock<HashMap<String, Vec<UnboundedSender<Frame>>>>>,
}

impl LuoshuSledData {
    pub fn new() -> Self {
        let storage: LuoshuSledStorage = LuoshuSledStorage::default();
        let configuration_store = Arc::new(RwLock::new(ConfiguratorStore::new(storage.clone())));
        let namespace_store = Arc::new(RwLock::new(NamespaceStore::new(storage.clone())));
        let service_store = Arc::new(RwLock::new(RegistryStore::new(storage)));
        LuoshuSledData {
            configuration_store,
            namespace_store,
            service_store,
            config_subscribers: Arc::new(RwLock::new(HashMap::new())),
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
    async fn append(&self, value: &LuoshuDataEnum) -> Result<()> {
        match value {
            LuoshuDataEnum::Namespace(value) => {
                self.namespace_store.write().await.append(value.into())?
            }
            LuoshuDataEnum::Configuration(value) => {
                match self.config_subscribers.write().await.get_mut(&value.name) {
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
                self.configuration_store
                    .write()
                    .await
                    .append(value.into())?
            }
            LuoshuDataEnum::Service(value) => {
                self.service_store.write().await.append(value.into())?
            }
            _ => {}
        };
        Ok(())
    }
    async fn remove(&self, value: &LuoshuDataEnum) -> Result<()> {
        match value {
            LuoshuDataEnum::Namespace(value) => {
                self.namespace_store.write().await.remove(value.into())?
            }
            LuoshuDataEnum::Configuration(value) => self
                .configuration_store
                .write()
                .await
                .remove(value.into())?,
            LuoshuDataEnum::Service(value) => {
                self.service_store.write().await.remove(value.into())?
            }
            _ => {}
        };
        Ok(())
    }

    async fn sync(&self, value: &LuoshuDataEnum) -> Result<()> {
        let _ = value;
        Ok(())
    }

    async fn subscribe(&self, value: String, connection: UnboundedSender<Frame>) -> Result<()> {
        match self
            .config_subscribers
            .write()
            .await
            .get_mut(value.as_str())
        {
            None => {
                self.config_subscribers
                    .write()
                    .await
                    .insert(value, vec![connection]);
            }
            Some(subscribers) => {
                subscribers.push(connection.into());
            }
        };
        Ok(())
    }
}
