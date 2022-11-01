use luoshu_configuration::{Configurator, ConfiguratorStore};
use luoshu_connection::Connector;
use luoshu_core::default_namespace;
use luoshu_namespace::NamespaceStore;
use luoshu_registry::{Registry, RegistryStore, Service};
use luoshu_sled_storage::LuoshuSledStorage;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Deserialize)]
pub(crate) struct ServiceReg {
    #[serde(default = "default_namespace")]
    namespace: String,
    name: String,
    #[serde(flatten)]
    service: Service,
}

impl From<ServiceReg> for Registry {
    fn from(service_reg: ServiceReg) -> Self {
        let mut registry = Registry::new(Some(service_reg.namespace), service_reg.name);
        registry
            .register_service(service_reg.service.host, service_reg.service.port)
            .unwrap();
        registry
    }
}

#[derive(Deserialize)]
pub(crate) struct ConfigurationReg {
    #[serde(default = "default_namespace")]
    namespace: String,
    name: String,
    config: Value,
}

impl From<ConfigurationReg> for Configurator {
    fn from(configuration_reg: ConfigurationReg) -> Self {
        let mut configuration = Configurator::new(Some(configuration_reg.namespace));
        configuration
            .set_configuration(configuration_reg.name, configuration_reg.config)
            .unwrap();
        configuration
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
