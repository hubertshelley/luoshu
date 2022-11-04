use crate::data::ConfigurationReg;
use luoshu_core::Store;
use salvo::{handler, writer::Json, Depot, Request, Response, Router};

use crate::web::error::WebResult;
use crate::web::resp::Resp;
use crate::LuoshuData;

// use crate::web::LUOSHU_DATA;

pub fn get_routers() -> Router {
    Router::with_path("configuration").post(append).get(list)
    // .push(Router::with_path("delete").post(delete))
}

#[handler]
async fn append(req: &mut Request, res: &mut Response, depot: &mut Depot) -> WebResult<()> {
    let value = req.parse_body::<ConfigurationReg>().await?;
    let data = depot.obtain::<LuoshuData>().unwrap();
    data.append(&value.into()).await?;
    res.render(Json(Resp::success("ok")));
    Ok(())
}

#[handler]
async fn list(_: &mut Request, res: &mut Response, depot: &mut Depot) -> WebResult<()> {
    let data = depot.obtain::<LuoshuData>().unwrap();
    res.render(Json(Resp::success(
        data.configuration_store.write().await.get_values(),
    )));
    Ok(())
}
