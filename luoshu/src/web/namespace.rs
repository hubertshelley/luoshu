use luoshu_core::Store;
use salvo::{handler, writer::Json, Depot, Request, Response, Router};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::data::{LuoshuDataHandle, LuoshuSledData, NamespaceReg};
use crate::web::error::WebResult;
use crate::web::resp::Resp;
// use crate::web::LUOSHU_DATA;

pub fn get_routers() -> Router {
    Router::with_path("namespace").post(append).get(list)
    // .push(Router::with_path("delete").post(delete))
}

// #[handler]
// async fn delete(req: &mut Request, res: &mut Response) -> WebResult<()> {
//     res.set_status_code(StatusCode::OK);
//     res.render("删除成功");
//     Ok(())
// }

#[handler]
async fn append(req: &mut Request, res: &mut Response, depot: &mut Depot) -> WebResult<()> {
    let value = req.parse_body::<NamespaceReg>().await?;
    let data = depot.obtain::<Arc<RwLock<LuoshuSledData>>>().unwrap();
    data.write().await.append(&value.into()).await?;
    res.render(Json(Resp::success("ok")));
    data.write().await.namespace_store.save()?;
    Ok(())
}

#[handler]
async fn list(_: &mut Request, res: &mut Response, depot: &mut Depot) -> WebResult<()> {
    let data = depot.obtain::<Arc<RwLock<LuoshuSledData>>>().unwrap();
    res.render(Json(Resp::success(
        data.write().await.namespace_store.get_values(),
    )));
    Ok(())
}
