use salvo::{handler, writer::Json, Depot, Request, Response, Router};
use serde::Deserialize;

use crate::web::error::WebResult;
use crate::web::resp::Resp;
use crate::LuoshuData;
use luoshu_registry::{Registry, Service};

pub fn get_routers() -> Router {
    Router::with_path("service").post(append).get(list)
    // .push(Router::with_path("delete").post(delete))
}

// #[handler]
// async fn delete(req: &mut Request, res: &mut Response) -> WebResult<()> {
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
async fn append(req: &mut Request, res: &mut Response, depot: &mut Depot) -> WebResult<()> {
    let value = req.parse_body::<ServiceReg>().await?;
    let data = depot.obtain::<LuoshuData>().unwrap();
    data.service_store
        .write()
        .await
        .append_registry(value.into())?;
    res.render(Json(Resp::success("ok")));
    Ok(())
}

#[handler]
async fn list(_: &mut Request, res: &mut Response, depot: &mut Depot) -> WebResult<()> {
    let data = depot.obtain::<LuoshuData>().unwrap();
    let values: Vec<Registry> = data
        .service_store
        .write()
        .await
        .values
        .values()
        .cloned()
        .collect();
    res.render(Json(Resp::success(values)));
    Ok(())
}
