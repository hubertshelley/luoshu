use std::borrow::{BorrowMut, Cow};
use std::sync::Arc;
use once_cell::sync::Lazy;
use salvo::prelude::*;
use tokio::sync::RwLock;

mod configuration;
mod namespace;
mod service;

use luoshu_configuration::ConfiguratorStore;
use luoshu_namespace::NamespaceStore;
use luoshu_registry::RegistryStore;
use luoshu_sled_storage::LuoshuSledStorage;
use luoshu_connection::Connector;

use service::get_routers as get_service_routers;
use configuration::get_routers as get_configuration_routers;
use namespace::get_routers as get_namespace_routers;

// static LUOSHU_DATA: Lazy<RwLock<LuoshuData>> = Lazy::new(|| RwLock::new(LuoshuData::new()));

struct LuoshuData<'a> {
    configuration_store: &'a mut ConfiguratorStore<'a, LuoshuSledStorage, Connector>,
    namespace_store: &'a mut NamespaceStore<'a, LuoshuSledStorage, Connector>,
    service_store: &'a mut RegistryStore<'a, LuoshuSledStorage, Connector>,
}

impl LuoshuData<'_> {
    pub fn new() -> Self {
        let storage: LuoshuSledStorage = LuoshuSledStorage::default();
        let connection: Connector = Connector {};
        let configuration_store = &mut ConfiguratorStore::new(&connection, &storage);
        let namespace_store = &mut NamespaceStore::new(&connection, &storage);
        let service_store = &mut RegistryStore::new(&connection, &storage);
        LuoshuData {
            configuration_store,
            namespace_store,
            service_store,
        }
    }
}

pub async fn run_server(addr: &str) {
    let storage: LuoshuSledStorage = LuoshuSledStorage::default();
    let connection: Connector = Connector {};
    let configuration_store = &mut ConfiguratorStore::new(&connection, &storage);
    let namespace_store = &mut NamespaceStore::new(&connection, &storage);
    let service_store = &mut RegistryStore::new(&connection, &storage);

    let data = RwLock::new(LuoshuData {
        configuration_store,
        namespace_store,
        service_store,
    });

    let set_store = SetStore(data);

    let router = Router::with_hoop(set_store)
        .get(index)
        .push(get_service_routers())
        .push(get_namespace_routers())
        .push(get_configuration_routers());
    Server::new(TcpListener::bind(addr)).serve(router).await;
}

struct SetStore<'a>(RwLock<LuoshuData<'a>>);

#[async_trait]
impl<'a> Handler for SetStore<'static> {
    async fn handle(&self, _req: &mut Request, _depot: &mut Depot, _res: &mut Response, _ctrl: &mut FlowCtrl) {
        _depot.inject(&self.0);
        _ctrl.call_next(_req, _depot, _res).await;
    }
}

#[handler]
async fn index(res: &mut Response) {
    res.render(Text::Html(INDEX_HTML));
}

static INDEX_HTML: &str = include_str!("./templates/index.html");
