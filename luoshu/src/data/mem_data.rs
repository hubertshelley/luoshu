use crate::data::{
    Frame, LuoshuDataEnum, LuoshuDataHandle, LuoshuDataServiceHandle, LuoshuSyncDataEnum, Subscribe,
};
use anyhow::Result;
use async_trait::async_trait;
use luoshu_configuration::ConfiguratorStore;
use luoshu_core::Store;
use luoshu_mem_storage::LuoshuMemStorage;
use luoshu_namespace::NamespaceStore;
use luoshu_registry::{RegistryStore, Service};
use std::net::SocketAddr;
use tokio::sync::mpsc::UnboundedSender;

/// 客户端数据层内存实现
pub struct LuoshuMemData {
    /// 配置中心存储器
    pub configuration_store: ConfiguratorStore<LuoshuMemStorage>,
    /// 命名空间存储器
    pub namespace_store: NamespaceStore<LuoshuMemStorage>,
    /// 注册中心存储器
    pub service_store: RegistryStore<LuoshuMemStorage>,
}

impl LuoshuMemData {
    /// 服务器端数据层内存实现实例化
    pub fn new() -> Self {
        let storage: LuoshuMemStorage = LuoshuMemStorage::default();
        let configuration_store = ConfiguratorStore::new(storage.clone());
        let namespace_store = NamespaceStore::new(storage.clone());
        let service_store = RegistryStore::new(storage);
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
    async fn append(
        &mut self,
        value: &LuoshuDataEnum,
        client: Option<SocketAddr>,
        sender: Option<&UnboundedSender<Frame>>,
    ) -> Result<()> {
        let _ = client;
        let _ = sender;
        match value {
            LuoshuDataEnum::Namespace(value) => self.namespace_store.append(value.into())?,
            LuoshuDataEnum::Configuration(value) => {
                self.configuration_store.append(value.into())?
            }
            LuoshuDataEnum::Service(value) => self.service_store.append(value.into())?,
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

    async fn sync(&mut self, value: &LuoshuSyncDataEnum) -> Result<()> {
        match value {
            LuoshuSyncDataEnum::Namespace(_) => {}
            LuoshuSyncDataEnum::Configuration(_) => {}
            LuoshuSyncDataEnum::Registry(registries) => {
                print!("{:#?}", registries);
                self.service_store.set_values(registries.clone())
            }
            LuoshuSyncDataEnum::LuoshuData(_) => {}
        };
        Ok(())
    }

    async fn subscribe(
        &mut self,
        subscribe: Subscribe,
        subscriber_sender: &UnboundedSender<Frame>,
    ) -> Result<()> {
        let (_, _) = (subscribe, subscriber_sender);
        Ok(())
    }

    async fn broken(&mut self, client: SocketAddr) -> Result<()> {
        let _ = client;
        Ok(())
    }
}

impl LuoshuDataServiceHandle for LuoshuMemData {
    fn get_service(&mut self, name: String, namespace: Option<String>) -> Vec<Service> {
        match self.service_store.get_service(name, namespace) {
            None => {
                vec![]
            }
            Some(registry) => registry.get_service(),
        }
    }
}
