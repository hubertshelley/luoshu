use anyhow::Result;
use luoshu_configuration::{Configurator, ConfiguratorStore};
use luoshu_core::Store;
use luoshu_namespace::{Namespace, NamespaceStore};
use luoshu_registry::{Registry, RegistryStore};
use luoshu_sled_storage::LuoshuSledStorage;

fn main() -> Result<()> {
    let storage = LuoshuSledStorage::new("tests");

    let mut namespace_store = NamespaceStore::new(storage.clone());
    namespace_store.append(Namespace::new("hello".into()))?;
    namespace_store.save()?;
    println!("{:#?}", namespace_store.get_values());

    let mut configurator = Configurator::new(None);
    configurator
        .set_configuration("test".into(), "{\"hello\": \"world\"}".into())
        .unwrap();
    let mut configurator_store = ConfiguratorStore::new(storage.clone());
    configurator_store.append(configurator)?;
    configurator_store.save()?;
    println!("{:#?}", configurator_store.get_values());

    let mut registry = Registry::new(None, "hello".into());
    registry.register_service(("127.0.0.1", 7890).into()).unwrap();
    let mut registry_store = RegistryStore::new(storage);
    registry_store.append(registry)?;
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
