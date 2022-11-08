use crate::data::{Frame, LuoshuDataEnum, LuoshuDataHandle};
use anyhow::Result;
use async_trait::async_trait;
use luoshu_configuration::ConfiguratorStore;
use luoshu_core::Store;
use luoshu_mem_storage::LuoshuMemStorage;
use luoshu_namespace::NamespaceStore;
use luoshu_registry::RegistryStore;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct LuoshuMemData {
    pub configuration_store: Arc<RwLock<ConfiguratorStore<LuoshuMemStorage>>>,
    pub namespace_store: Arc<RwLock<NamespaceStore<LuoshuMemStorage>>>,
    pub service_store: Arc<RwLock<RegistryStore<LuoshuMemStorage>>>,
}

impl LuoshuMemData {
    pub fn new() -> Self {
        let storage: LuoshuMemStorage = LuoshuMemStorage::default();
        let configuration_store = Arc::new(RwLock::new(ConfiguratorStore::new(storage.clone())));
        let namespace_store = Arc::new(RwLock::new(NamespaceStore::new(storage.clone())));
        let service_store = Arc::new(RwLock::new(RegistryStore::new(storage)));
        LuoshuMemData {
            configuration_store,
            namespace_store,
            service_store,
        }
    }
}

impl Default for LuoshuMemData {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LuoshuDataHandle for LuoshuMemData {
    async fn append(&self, value: &LuoshuDataEnum) -> Result<()> {
        match value {
            LuoshuDataEnum::Namespace(value) => {
                self.namespace_store.write().await.append(value.into())?
            }
            LuoshuDataEnum::Configuration(value) => self
                .configuration_store
                .write()
                .await
                .append(value.into())?,
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
        match value {
            LuoshuDataEnum::Namespace(_) => {}
            LuoshuDataEnum::Configuration(config) => {
                println!("{}", config.name);
            }
            LuoshuDataEnum::Service(_) => {}
            LuoshuDataEnum::Subscribe(_) => {}
        };
        Ok(())
    }

    async fn subscribe(&self, value: String, connection: UnboundedSender<Frame>) -> Result<()> {
        let (_, _) = (value, connection);
        Ok(())
    }
}
