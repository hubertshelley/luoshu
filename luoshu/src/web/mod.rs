use salvo::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;

mod configuration;
mod namespace;
mod service;

use luoshu_configuration::ConfiguratorStore;
use luoshu_connection::Connector;
use luoshu_namespace::NamespaceStore;
use luoshu_registry::RegistryStore;
use luoshu_sled_storage::LuoshuSledStorage;

use configuration::get_routers as get_configuration_routers;
use namespace::get_routers as get_namespace_routers;
use service::get_routers as get_service_routers;

// static LUOSHU_DATA: Lazy<RwLock<LuoshuData>> = Lazy::new(|| RwLock::new(LuoshuData::new()));

struct LuoshuData {
    configuration_store: ConfiguratorStore<LuoshuSledStorage, Connector>,
    namespace_store: NamespaceStore<LuoshuSledStorage, Connector>,
    service_store: RegistryStore<LuoshuSledStorage, Connector>,
}

impl LuoshuData {
    pub fn new() -> Self {
        let storage: LuoshuSledStorage = LuoshuSledStorage::default();
        let connection: Connector = Connector {};
        let configuration_store = ConfiguratorStore::new(connection.clone(), storage.clone());
        let namespace_store = NamespaceStore::new(connection.clone(), storage.clone());
        let service_store = RegistryStore::new(connection, storage);
        LuoshuData {
            configuration_store,
            namespace_store,
            service_store,
        }
    }
}

pub async fn run_server(addr: &str) {
    let data = Arc::new(RwLock::new(LuoshuData::new()));

    let set_store = SetStore(data);

    let router = Router::with_hoop(set_store)
        .get(index)
        .push(get_service_routers())
        .push(get_namespace_routers())
        .push(get_configuration_routers());
    Server::new(TcpListener::bind(addr)).serve(router).await;
}

struct SetStore(Arc<RwLock<LuoshuData>>);

#[async_trait]
impl Handler for SetStore {
    async fn handle(
        &self,
        _req: &mut Request,
        _depot: &mut Depot,
        _res: &mut Response,
        _ctrl: &mut FlowCtrl,
    ) {
        _depot.inject(self.0.clone());
        _ctrl.call_next(_req, _depot, _res).await;
    }
}

#[handler]
async fn index(res: &mut Response) {
    res.render(Text::Html(INDEX_HTML));
}

static INDEX_HTML: &str = include_str!("./templates/index.html");
