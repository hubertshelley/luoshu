//! registry for luoshu
#![deny(missing_docs)]

extern crate core;

mod configurator_storage;
mod namespace_storage;
mod registry_storage;

use once_cell::sync::Lazy;
use tokio::sync::mpsc::UnboundedSender;
use serde::{Deserialize, Serialize};

pub use configurator_storage::*;
pub use namespace_storage::*;
pub use registry_storage::*;

use luoshu_configuration::Configurator;
use luoshu_core::Storage;
use luoshu_namespace::Namespace;
use luoshu_registry::Registry;

/// 全局存储文件配置
pub static SLED_DB: Lazy<sled::Db> = Lazy::new(|| { sled::open("registry.db").unwrap() });
/// 全局存储文件配置
static SLED_TX: Lazy<UnboundedSender<StoreType>> = Lazy::new(|| {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<StoreType>();
    // let sled_db = sled::open("registry.db").unwrap();
    tokio::spawn(async move {
        loop {
            if let Some(store_data) = rx.recv().await {
                println!("store_data {:#?}", store_data);
                let db = SLED_DB.clone();
                match store_data.clone() {
                    StoreType::Configurator(value) => {
                        println!("Configurator {:#?}", value);
                        db
                            .insert(
                                &"ConfiguratorStorage",
                                serde_json::to_string(&store_data).unwrap().as_bytes(),
                            )
                            .expect("ConfiguratorStorage store error");
                    }
                    StoreType::Namespace(value) => {
                        println!("Namespace {:#?}", value);
                        db
                            .insert(
                                &"NamespaceStorage",
                                serde_json::to_string(&store_data).unwrap().as_bytes(),
                            )
                            .expect("Namespace store error");
                    }
                    StoreType::Registry(value) => {
                        println!("Registry {:#?}", value);
                        db
                            .insert(
                                &"RegistryStorage",
                                serde_json::to_string(&store_data).unwrap().as_bytes(),
                            )
                            .expect("Registry store error");
                    }
                    _ => {}
                }
                db.flush().unwrap();
            }
        }
    });
    tx
});

/// 存储枚举表
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StoreType {
    /// 配置中心
    Configurator(Vec<Configurator>),
    /// 命名空间
    Namespace(Vec<Namespace>),
    /// 注册中心
    Registry(Vec<Registry>),
    /// 注册中心
    Empty,
}

impl From<Vec<Configurator>> for StoreType {
    fn from(value: Vec<Configurator>) -> Self {
        StoreType::Configurator(value)
    }
}

impl From<Vec<Registry>> for StoreType {
    fn from(value: Vec<Registry>) -> Self {
        StoreType::Registry(value)
    }
}

impl From<Vec<Namespace>> for StoreType {
    fn from(value: Vec<Namespace>) -> Self {
        StoreType::Namespace(value)
    }
}

/// 洛书数据持久化Sled实现
pub struct LuoshuSledStorage;

impl Storage for LuoshuSledStorage {
    type Target = StoreType;

    fn save(&self, values: Self::Target) -> anyhow::Result<()> {
        SLED_TX.send(values)?;
        Ok(())
    }

    fn load(&mut self) -> anyhow::Result<Self::Target> {
        Ok(StoreType::Empty)
    }

    fn load_value(&mut self, key: &str) -> anyhow::Result<Self::Target> {
        match SLED_DB.get(key.as_bytes())? {
            None => { Ok(StoreType::Empty) }
            Some(data) => {
                println!("{:#?}", data);
                let data: Vec<Namespace> = serde_json::from_slice(data.to_vec().as_slice())?;
                let data = data.into();
                Ok(data)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{LuoshuSledStorage, SLED_DB, SLED_TX, StoreType};
    use luoshu_configuration::Configurator;
    use luoshu_core::Storage;
    use luoshu_registry::Registry;

    #[tokio::test]
    async fn registry_store_save_test() {
        let mut registry = Registry::new(None, "test_registry".into());
        registry.register_service("127.0.0.1".into(), 8000).unwrap();
        SLED_TX.clone().send(vec![registry].into()).unwrap();
    }

    #[tokio::test]
    async fn configurator_store_save_test() {
        let mut configurator = Configurator::new(None);
        configurator
            .set_configuration("test".into(), "{\"hello\": \"world\"}".to_string())
            .unwrap();
        match SLED_TX.clone().send(vec![configurator].into()) {
            Ok(ok) => { println!("{:#?}", ok); }
            Err(e) => { println!("{:#?}", e); }
        };
    }

    #[tokio::test]
    async fn store_load_test1() {
        match SLED_DB.get(&"Configurator").unwrap() {
            None => { println!("not read"); }
            Some(data) => {
                let data: StoreType = serde_json::from_slice(data.to_vec().as_slice()).unwrap();
                println!("{:#?}", data);
            }
        }
    }

    #[tokio::test]
    async fn store_load_test() {
        let mut storage = LuoshuSledStorage {};
        // let load_data = storage.load_value("ConfiguratorStorage").unwrap();
        // println!("Configurator: {:#?}", load_data);
        let load_data = storage.load_value("NamespaceStorage").unwrap();
        println!("Namespace: {:#?}", load_data);
        let load_data = storage.load_value("RegistryStorage").unwrap();
        println!("Registry: {:#?}", load_data);
    }
}
