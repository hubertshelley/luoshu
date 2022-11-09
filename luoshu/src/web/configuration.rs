use crate::data::{ConfigurationReg, LuoshuDataHandle, LuoshuSledData};
use luoshu_core::Store;
use salvo::{handler, writer::Json, Depot, Request, Response, Router};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::web::error::WebResult;
use crate::web::resp::Resp;

// use crate::web::LUOSHU_DATA;

pub fn get_routers() -> Router {
    Router::with_path("configuration").post(append).get(list)
    // .push(Router::with_path("delete").post(delete))
}

#[handler]
async fn append(req: &mut Request, res: &mut Response, depot: &mut Depot) -> WebResult<()> {
    let value = req.parse_body::<ConfigurationReg>().await?;
    let data = depot.obtain::<Arc<RwLock<LuoshuSledData>>>().unwrap();
    data.write().await.append(&value.into(), None).await?;
    res.render(Json(Resp::success("ok")));
    Ok(())
}

#[handler]
async fn list(_: &mut Request, res: &mut Response, depot: &mut Depot) -> WebResult<()> {
    let data = depot.obtain::<Arc<RwLock<LuoshuSledData>>>().unwrap();
    res.render(Json(Resp::success(
        data.write().await.configuration_store.get_values(),
    )));
    Ok(())
}
