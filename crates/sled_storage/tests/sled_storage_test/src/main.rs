use anyhow::Result;
use luoshu_configuration::{Configurator, ConfiguratorStore};
use luoshu_core::{Connection, Store};
use luoshu_namespace::NamespaceStore;
use luoshu_sled_storage::LuoshuSledStorage;
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

fn main() -> Result<()> {
    let storage = LuoshuSledStorage::new("tests");
    let mut configurator = Configurator::new(None);
    configurator
        .set_configuration("test".into(), "{\"hello\": \"world\"}".to_string())
        .unwrap();
    let connector = Connector {};
    let mut store = ConfiguratorStore::new(Box::new(connector), storage.clone());
    store.append_configurator(configurator)?;
    store.save()?;
    println!("{:#?}", store.get_values());

    let mut store = NamespaceStore::new(storage);
    store.load()?;
    println!("{:#?}", store.get_values());
    Ok(())
}
