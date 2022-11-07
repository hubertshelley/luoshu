use luoshu_core::Store;
use salvo::{handler, writer::Json, Depot, Request, Response, Router};

use crate::data::{LuoshuData, ServiceReg};
use crate::web::error::WebResult;
use crate::web::resp::Resp;

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

#[handler]
async fn append(req: &mut Request, res: &mut Response, depot: &mut Depot) -> WebResult<()> {
    let value = req.parse_body::<ServiceReg>().await?;
    let data = depot.obtain::<LuoshuData>().unwrap();
    data.append(&value.into()).await?;
    res.render(Json(Resp::success("ok")));
    Ok(())
}

#[handler]
async fn list(_: &mut Request, res: &mut Response, depot: &mut Depot) -> WebResult<()> {
    let data = depot.obtain::<LuoshuData>().unwrap();
    res.render(Json(Resp::success(
        data.service_store.write().await.get_values(),
    )));
    Ok(())
}
