use anyhow::Result;
use sled_storage::{ConfiguratorStorage, SLED_DB};
use core::Connection;
use std::net::SocketAddr;
use configuration::{Configurator, ConfiguratorStore};

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
    Ok(())
}
