use luoshu_configuration::ConfiguratorStore;
use luoshu_connection::Connector;
use luoshu_namespace::NamespaceStore;
use luoshu_registry::RegistryStore;
use luoshu_sled_storage::LuoshuSledStorage;

pub(crate) struct LuoshuData {
    pub(crate) configuration_store: ConfiguratorStore<LuoshuSledStorage, Connector>,
    pub(crate) namespace_store: NamespaceStore<LuoshuSledStorage, Connector>,
    pub(crate) service_store: RegistryStore<LuoshuSledStorage, Connector>,
}

impl LuoshuData {
    pub fn new() -> Self {
        let storage: LuoshuSledStorage = LuoshuSledStorage::default();
        let connection: Connector = Connector {};
        let configuration_store = ConfiguratorStore::new(connection.clone(), storage.clone());
        let namespace_store = NamespaceStore::new(connection.clone(), storage.clone());
        let service_store = RegistryStore::new(connection, storage);
        LuoshuData {
            configuration_store,
            namespace_store,
            service_store,
        }
    }
}
