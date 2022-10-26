use anyhow::Result;
use luoshu_configuration::{Configurator, ConfiguratorStore};
use luoshu_core::{Connection, Store};
use luoshu_namespace::{Namespace, NamespaceStore};
use luoshu_sled_storage::LuoshuSledStorage;
use std::net::SocketAddr;
use luoshu_registry::{Registry, RegistryStore};

#[derive(Clone)]
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
    let connector = Connector {};

    let mut namespace_store = NamespaceStore::new(Box::new(connector.clone()), &storage);
    namespace_store.append_namespace(Namespace::new("hello".into()))?;
    namespace_store.save()?;
    println!("{:#?}", namespace_store.get_values());

    let mut configurator = Configurator::new(None);
    configurator
        .set_configuration("test".into(), "{\"hello\": \"world\"}".to_string())
        .unwrap();
    let mut configurator_store = ConfiguratorStore::new(Box::new(connector.clone()), &storage);
    configurator_store.append_configurator(configurator)?;
    configurator_store.save()?;
    println!("{:#?}", configurator_store.get_values());

    let mut registry = Registry::new(None, "hello".into());
    registry
        .register_service("127.0.0.1".into(), 7890)
        .unwrap();
    let mut registry_store = RegistryStore::new(Box::new(connector), &storage);
    registry_store.append_registry(registry)?;
    registry_store.save()?;
    println!("{:#?}", registry_store.get_values());

    configurator_store.load()?;
    println!("{:#?}", configurator_store.get_values());

    registry_store.load()?;
    println!("{:#?}", registry_store.get_values());

    namespace_store.load()?;
    println!("{:#?}", namespace_store.get_values());
    Ok(())
}
