use std::sync::Arc;

use salvo::{handler, prelude::StatusCode, writer::Json, Depot, Error, Request, Response, Router};
use serde::Deserialize;
use tokio::sync::RwLock;

use crate::web::LuoshuData;

use luoshu_registry::{Registry, Service};

pub fn get_routers() -> Router {
    Router::with_path("service").post(append).get(list)
    // .push(Router::with_path("delete").post(delete))
}

// #[handler]
// async fn delete(req: &mut Request, res: &mut Response) -> Result<(), Error> {
//     res.set_status_code(StatusCode::OK);
//     res.render("删除成功");
//     Ok(())
// }

#[derive(Deserialize)]
struct ServiceReg {
    #[serde(default = "default_namespace")]
    namespace: String,
    name: String,
    #[serde(flatten)]
    service: Service,
}

fn default_namespace() -> String {
    "default".to_string()
}

impl From<ServiceReg> for Registry {
    fn from(service_reg: ServiceReg) -> Self {
        let mut registry = Registry::new(Some(service_reg.namespace), service_reg.name);
        registry
            .register_service(service_reg.service.host, service_reg.service.port)
            .unwrap();
        registry
    }
}

#[handler]
async fn append(req: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<(), Error> {
    let value = req.parse_body::<ServiceReg>().await?;
    let data = depot.obtain::<Arc<RwLock<LuoshuData>>>().unwrap();
    match data
        .write()
        .await
        .service_store
        .append_registry(value.into())
    {
        Ok(_) => {
            res.render("ok");
            res.set_status_code(StatusCode::OK);
        }
        Err(_) => {
            res.set_status_code(StatusCode::BAD_REQUEST);
        }
    }
    Ok(())
}

#[handler]
async fn list(_: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<(), Error> {
    let data = depot.obtain::<Arc<RwLock<LuoshuData>>>().unwrap();
    // match data.write().await.service_store.load() {
    //     Ok(_) => {
    //         res.set_status_code(StatusCode::OK);
    //     }
    //     Err(_) => {
    //         res.set_status_code(StatusCode::BAD_REQUEST);
    //     }
    // }
    let values: Vec<Registry> = data
        .write()
        .await
        .service_store
        .values
        .values()
        .cloned()
        .collect();
    res.render(Json(values));
    Ok(())
}
