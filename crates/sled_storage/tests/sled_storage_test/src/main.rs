use anyhow::Result;
use luoshu_configuration::{Configurator, ConfiguratorStore};
use luoshu_core::Connection;
use luoshu_namespace::NamespaceStore;
use luoshu_sled_storage::{ConfiguratorStorage, NamespaceStorage, SLED_DB};
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

#[tokio::main]
async fn main() -> Result<()> {
    let db = SLED_DB.read().await.clone();
    let storage = ConfiguratorStorage::new(db);
    let connector = Connector {};
    let mut store = ConfiguratorStore::new(Box::new(connector), Box::new(storage));
    let mut configurator = Configurator::new(None);
    configurator.set_configuration("test".into(), "{\"hello\": \"world\"}".to_string())?;
    store.append_configurator(configurator)?;
    store.save()?;

    let db = SLED_DB.read().await.clone();
    let storage = NamespaceStorage::new(db);
    let mut store = NamespaceStore::new(Box::new(storage));
    store.load()?;
    println!("{:#?}", store.namespaces);
    Ok(())
}
