use anyhow::Result;
use luoshu_core::Storage;
use luoshu_registry::Registry;

/// 注册中心存储sled实现
pub struct RegistryStorage {
    db: sled::Db,
    key: String,
}

impl RegistryStorage {
    fn new(db: sled::Db) -> Self {
        Self {
            db,
            key: "RegistryStorage".to_string(),
        }
    }
}

impl Storage for RegistryStorage {
    type Target = Vec<Registry>;

    fn save(&self, values: Self::Target) -> Result<()> {
        self.db
            .insert(
                self.key.as_bytes(),
                serde_json::to_string(&values).unwrap().as_bytes(),
            )
            .expect("RegistryStorage save error");
        Ok(())
    }

    fn load(&mut self) -> Result<Self::Target> {
        let _data = self.db.get(self.key.as_bytes()).unwrap();
        match _data {
            None => Ok(vec![]),
            Some(_data) => {
                let _data: Vec<Registry> = serde_json::from_slice(_data.to_vec().as_slice())?;
                Ok(_data)
            }
        }
    }

    fn load_value(&mut self, key: &str) -> Result<Self::Target> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{RegistryStorage, SLED_DB};
    use anyhow::Result;
    use luoshu_core::Connection;
    use luoshu_registry::{Registry, RegistryStore};
    use std::net::SocketAddr;

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

    #[test]
    fn registry_store_save_test() -> Result<()> {
        let storage = RegistryStorage::new(SLED_DB.clone());
        let connector = Connector {};
        let mut store = RegistryStore::new(Box::new(connector), Box::new(storage));
        let mut registry = Registry::new(None, "test_registry".into());
        registry.register_service("127.0.0.1".into(), 8000)?;
        store.append_registry(registry)?;
        store.save()?;
        Ok(())
    }

    #[test]
    fn registry_store_load_test() -> Result<()> {
        let storage = RegistryStorage::new(SLED_DB.clone());
        let connector = Connector {};
        let mut store = RegistryStore::new(Box::new(connector), Box::new(storage));
        store.load()?;
        println!("{:#?}", store.registries);
        Ok(())
    }
}
