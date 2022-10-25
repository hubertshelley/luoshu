use anyhow::Result;
use luoshu_configuration::Configurator;
use luoshu_core::Storage;

/// 配置中心存储sled实现
pub struct ConfiguratorStorage {
    db: sled::Db,
    key: String,
}

impl ConfiguratorStorage {
    /// 初始化配置中心存储
    pub fn new(db: sled::Db) -> Self {
        Self {
            db,
            key: "ConfiguratorStorage".to_string(),
        }
    }
}

impl Storage for ConfiguratorStorage {
    type Target = Configurator;

    fn save(&self, values: Vec<Self::Target>) -> Result<()> {
        self.db
            .insert(
                self.key.as_bytes(),
                serde_json::to_string(&values).unwrap().as_bytes(),
            )
            .expect("ConfiguratorStorage save error");
        Ok(())
    }

    fn load(&mut self) -> Result<Vec<Self::Target>> {
        let _data = self.db.get(self.key.as_bytes()).unwrap();
        match _data {
            None => Ok(vec![]),
            Some(_data) => {
                let _data: Vec<Configurator> = serde_json::from_slice(_data.to_vec().as_slice())?;
                Ok(_data)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ConfiguratorStorage;
    use anyhow::Result;
    use luoshu_configuration::{Configurator, ConfiguratorStore};
    use luoshu_core::Connection;
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
        let db: sled::Db = sled::open("test_db_configuration1").unwrap();
        let storage = ConfiguratorStorage::new(db);
        let connector = Connector {};
        let mut store = ConfiguratorStore::new(Box::new(connector), Box::new(storage));
        let mut configurator = Configurator::new(None);
        configurator.set_configuration("test".into(), "{\"hello\": \"world\"}".to_string())?;
        store.append_configurator(configurator)?;
        store.save()?;
        Ok(())
    }

    #[test]
    fn registry_store_load_test() -> Result<()> {
        let db: sled::Db = sled::open("test_db_configuration2").unwrap();
        let storage = ConfiguratorStorage::new(db);
        let connector = Connector {};
        let mut store = ConfiguratorStore::new(Box::new(connector), Box::new(storage));
        store.load()?;
        println!("{:#?}", store.configurators);
        println!("{}", serde_json::to_string(&store.configurators)?);
        Ok(())
    }
}
