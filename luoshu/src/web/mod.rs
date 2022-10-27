use luoshu_connection::Connector;
use once_cell::sync::Lazy;
use salvo::prelude::*;

mod configuration;
mod namespace;
mod service;

use luoshu_configuration::ConfiguratorStore;
use luoshu_namespace::NamespaceStore;
use luoshu_registry::RegistryStore;
use luoshu_sled_storage::LuoshuSledStorage;

use service::get_routers as get_service_routers;

static LuoshuData: Lazy<LuoshuData> = Lazy::new(|| LuoshuData::new());

struct LuoshuData<'a> {
    configuration_store: &'a ConfiguratorStore<'a, LuoshuSledStorage>,
    namespace_store: &'a NamespaceStore<'a, LuoshuSledStorage>,
    service_store: &'a RegistryStore<'a, LuoshuSledStorage>,
}
impl LuoshuData<'_> {
    pub fn new() -> Self {
        let storage = LuoshuSledStorage::default();
        let connection = &Box::new(Connector {});
        let configuration_store = &ConfiguratorStore::new(connection, &storage);
        let namespace_store = &NamespaceStore::new(connection, &storage);
        let service_store = &RegistryStore::new(connection, &storage);
        LuoshuData {
            configuration_store,
            namespace_store,
            service_store,
        }
    }
}

pub async fn run_server(addr: &str) {
    let router = Router::new()
        .get(index)
        .push(get_service_routers())
        .push(
            Router::with_path("namespace")
                .post(append)
                .get(list)
                .push(Router::with_path("delete").post(delete)),
        )
        .push(
            Router::with_path("configuration")
                .post(append)
                .get(list)
                .push(Router::with_path("delete").post(delete)),
        );
    Server::new(TcpListener::bind(addr)).serve(router).await;
}

#[handler]
async fn index(res: &mut Response) {
    res.render(Text::Html(INDEX_HTML));
}

static INDEX_HTML: &str = include_str!("./templates/index.html");
