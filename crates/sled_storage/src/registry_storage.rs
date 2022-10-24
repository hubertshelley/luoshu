use anyhow::Result;
use core::Storage;
use registry::Registry;

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
    type Target = Registry;

    fn save(&self, values: Vec<Self::Target>) -> Result<()> {
        self.db
            .insert(
                self.key.as_bytes(),
                serde_json::to_string(&values).unwrap().as_bytes(),
            )
            .expect("RegistryStorage save error");
        Ok(())
    }

    fn load(&mut self) -> Result<Vec<Self::Target>> {
        let _data = self.db.get(self.key.as_bytes()).unwrap();
        match _data {
            None => Ok(vec![]),
            Some(_data) => {
                let _data: Vec<Registry> = serde_json::from_slice(_data.to_vec().as_slice())?;
                Ok(_data)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::net::SocketAddr;
    use crate::RegistryStorage;
    use anyhow::Result;
    use registry::{Registry, RegistryStore};
    use core::Connection;

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
        let db: sled::Db = sled::open("test_db_registry1").unwrap();
        let storage = RegistryStorage::new(db);
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
        let db: sled::Db = sled::open("test_db_registry2").unwrap();
        let storage = RegistryStorage::new(db);
        let connector = Connector {};
        let mut store = RegistryStore::new(Box::new(connector), Box::new(storage));
        store.load()?;
        println!("{:#?}", store.registries);
        Ok(())
    }
}
