use luoshu_configuration::{Configurator, ConfiguratorStore};
use luoshu_connection::Connector;
use luoshu_core::default_namespace;
use luoshu_namespace::{Namespace, NamespaceStore};
use luoshu_registry::{Registry, RegistryStore, Service};
use luoshu_sled_storage::LuoshuSledStorage;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{io::Cursor, sync::Arc};
use tokio::sync::RwLock;

use anyhow::Result;

#[derive(Deserialize, Serialize)]
pub(crate) struct ServiceReg {
    #[serde(default = "default_namespace")]
    namespace: String,
    name: String,
    #[serde(flatten)]
    service: Service,
}

impl From<&ServiceReg> for Registry {
    fn from(service_reg: &ServiceReg) -> Self {
        let mut registry = Registry::new(
            Some(service_reg.namespace.clone()),
            service_reg.name.clone(),
        );
        registry
            .register_service(service_reg.service.host.clone(), service_reg.service.port)
            .unwrap();
        registry
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ConfigurationReg {
    #[serde(default = "default_namespace")]
    namespace: String,
    name: String,
    config: Value,
}

impl From<&ConfigurationReg> for Configurator {
    fn from(configuration_reg: &ConfigurationReg) -> Self {
        let mut configuration = Configurator::new(Some(configuration_reg.namespace.clone()));
        configuration
            .set_configuration(
                configuration_reg.name.clone(),
                configuration_reg.config.clone(),
            )
            .unwrap();
        configuration
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct NamespaceReg {
    name: String,
}

impl From<&NamespaceReg> for Namespace {
    fn from(namespace_reg: &NamespaceReg) -> Self {
        Namespace::new(namespace_reg.name.clone())
    }
}

#[derive(Clone)]
pub(crate) struct LuoshuData {
    pub(crate) configuration_store: Arc<RwLock<ConfiguratorStore<LuoshuSledStorage, Connector>>>,
    pub(crate) namespace_store: Arc<RwLock<NamespaceStore<LuoshuSledStorage, Connector>>>,
    pub(crate) service_store: Arc<RwLock<RegistryStore<LuoshuSledStorage, Connector>>>,
}

impl LuoshuData {
    pub fn new() -> Self {
        let storage: LuoshuSledStorage = LuoshuSledStorage::default();
        let connection: Connector = Connector {};
        let configuration_store = Arc::new(RwLock::new(ConfiguratorStore::new(
            connection.clone(),
            storage.clone(),
        )));
        let namespace_store = Arc::new(RwLock::new(NamespaceStore::new(
            connection.clone(),
            storage.clone(),
        )));
        let service_store = Arc::new(RwLock::new(RegistryStore::new(connection, storage)));
        LuoshuData {
            configuration_store,
            namespace_store,
            service_store,
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize)]
pub(crate) enum ActionEnum {
    Up,
    Down,
    Sync,
}

#[derive(Serialize)]
pub(crate) struct Frame {
    action: ActionEnum,
    data: LuoshuDataEnum,
}

#[allow(dead_code)]
impl Frame {
    pub fn parse(_src: &mut Cursor<&[u8]>) -> Result<Frame> {
        Ok(Frame {
            action: ActionEnum::Up,
            data: NamespaceReg {
                name: "test".to_string(),
            }
            .into(),
        })
    }
    pub fn check(_src: &mut Cursor<&[u8]>) -> Result<()> {
        Ok(())
    }
}

#[derive(Serialize)]
pub(crate) enum LuoshuDataEnum {
    Namespace(NamespaceReg),
    Configuration(ConfigurationReg),
    Service(ServiceReg),
}

impl From<NamespaceReg> for LuoshuDataEnum {
    fn from(namespace: NamespaceReg) -> Self {
        Self::Namespace(namespace)
    }
}
impl From<ConfigurationReg> for LuoshuDataEnum {
    fn from(configuration: ConfigurationReg) -> Self {
        Self::Configuration(configuration)
    }
}
impl From<ServiceReg> for LuoshuDataEnum {
    fn from(service: ServiceReg) -> Self {
        Self::Service(service)
    }
}

impl LuoshuData {
    pub async fn append(&self, value: &LuoshuDataEnum) -> Result<()> {
        match value {
            LuoshuDataEnum::Namespace(value) => self
                .namespace_store
                .write()
                .await
                .append_namespace(value.into())?,
            LuoshuDataEnum::Configuration(value) => self
                .configuration_store
                .write()
                .await
                .append_configurator(value.into())?,
            LuoshuDataEnum::Service(value) => self
                .service_store
                .write()
                .await
                .append_registry(value.into())?,
        };
        Ok(())
    }
}
