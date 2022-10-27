//! registry for luoshu
#![deny(missing_docs)]

extern crate core;

use luoshu_core::Storage;
use once_cell::sync::Lazy;

/// 全局存储文件配置
static SLED_DB: Lazy<sled::Db> = Lazy::new(|| sled::open("registry.db").unwrap());

/// 洛书数据持久化Sled实现
#[derive(Debug, Clone)]
pub struct LuoshuSledStorage {
    storage: sled::Db,
}

impl Default for LuoshuSledStorage {
    fn default() -> Self {
        Self {
            storage: SLED_DB.clone(),
        }
    }
}

impl LuoshuSledStorage {
    /// 创建存储
    pub fn new(storage_file: &str) -> Self {
        Self {
            storage: sled::open(format!("{}.db", storage_file)).unwrap(),
        }
    }
}

impl Storage for LuoshuSledStorage {
    fn save(&self, key: &str, values: &[u8]) -> anyhow::Result<()> {
        self.storage.insert(key, values)?;
        self.storage.flush()?;
        Ok(())
    }
    fn load(&mut self, key: &str) -> anyhow::Result<Vec<u8>> {
        let data = self.storage.get(key)?;
        match data {
            Some(data) => Ok(data.to_vec()),
            None => Ok("[]".as_bytes().to_vec()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use crate::LuoshuSledStorage;
    use luoshu_configuration::{Configurator, ConfiguratorStore};
    use luoshu_core::{Connection, Store};
    use luoshu_registry::{Registry, RegistryStore};

    #[test]
    fn registry_store_save_test() {
        let mut registry = Registry::new(None, "test_registry".into());
        registry.register_service("127.0.0.1".into(), 8000).unwrap();
        let connector = Connector {};
        let storage = LuoshuSledStorage::new("registry_store_save_test");
        let mut store = RegistryStore::new(Box::new(connector), &storage);
        store.append_registry(registry).unwrap();
        store.save().unwrap();
    }

    #[test]
    fn configurator_store_save_test() {
        let mut configurator = Configurator::new(None);
        configurator
            .set_configuration("test".into(), "{\"hello\": \"world\"}".to_string())
            .unwrap();
        let connector = Connector {};
        let storage = LuoshuSledStorage::new("configurator_store_save_test");
        let mut store = ConfiguratorStore::new(Box::new(connector), &storage);
        store.append_configurator(configurator).unwrap();
        store.save().unwrap();
    }

    #[test]
    fn configurator_store_load_test() {
        let connector = Connector {};
        let storage = LuoshuSledStorage::default();
        let mut store = ConfiguratorStore::new(Box::new(connector), &storage);
        store.load().unwrap();
        println!("{:#?}", store.get_values());
    }

    struct Connector {}

    impl Connection for Connector {
        fn send(&self) {
            todo!()
        }

        fn recv(&self) {
            todo!()
        }

        fn connected(&self) {
            todo!()
        }

        fn disconnected(&self) {
            todo!()
        }

        fn get_ipaddr(&self) -> SocketAddr {
            todo!()
        }
    }
}
