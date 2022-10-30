use salvo::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;

mod configuration;
mod namespace;
mod service;
mod error;
mod resp;


use configuration::get_routers as get_configuration_routers;
use namespace::get_routers as get_namespace_routers;
use service::get_routers as get_service_routers;
use crate::LuoshuData;


pub(crate) async fn run_server(addr: &str, data: Arc<RwLock<LuoshuData>>) {
    let set_store = SetStore(data);

    let router = Router::with_hoop(set_store)
        .get(index)
        .push(get_service_routers())
        .push(get_namespace_routers())
        .push(get_configuration_routers());

    tracing::info!("admin listening on: http://{}", addr);

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
