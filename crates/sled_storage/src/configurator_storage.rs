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
    type Target = Vec<Configurator>;

    fn save(&self, values: Self::Target) -> Result<()> {
        self.db
            .insert(
                self.key.as_bytes(),
                serde_json::to_string(&values).unwrap().as_bytes(),
            )
            .expect("ConfiguratorStorage save error");
        Ok(())
    }

    fn load(&mut self) -> Result<Self::Target> {
        let _data = self.db.get(self.key.as_bytes()).unwrap();
        match _data {
            None => Ok(vec![]),
            Some(_data) => {
                let _data: Vec<Configurator> = serde_json::from_slice(_data.to_vec().as_slice())?;
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
    use crate::{ConfiguratorStorage, SLED_DB};
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
    fn registry_store_save_test() {
        let storage = ConfiguratorStorage::new(SLED_DB.clone());
        let connector = Connector {};
        let mut store = ConfiguratorStore::new(Box::new(connector), Box::new(storage));
        let mut configurator = Configurator::new(None);
        configurator
            .set_configuration("test".into(), "{\"hello\": \"world\"}".to_string())
            .unwrap();
        store.append_configurator(configurator).unwrap();
        store.save().unwrap();
    }

    #[test]
    fn registry_store_load_test() -> Result<()> {
        let storage = ConfiguratorStorage::new(SLED_DB.clone());
        let connector = Connector {};
        let mut store = ConfiguratorStore::new(Box::new(connector), Box::new(storage));
        store.load().unwrap();
        println!("{:#?}", store.configurators);
        println!("{}", serde_json::to_string(&store.configurators)?);
        Ok(())
    }
}
