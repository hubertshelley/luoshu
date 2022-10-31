use luoshu_configuration::ConfiguratorStore;
use luoshu_connection::Connector;
use luoshu_namespace::NamespaceStore;
use luoshu_registry::RegistryStore;
use luoshu_sled_storage::LuoshuSledStorage;
use std::sync::Arc;
use tokio::sync::RwLock;

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
