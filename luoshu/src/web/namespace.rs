use luoshu_core::Store;
use luoshu_namespace::Namespace;
use salvo::{handler, writer::Json, Depot, Request, Response, Router};

use crate::web::error::WebResult;
use crate::web::resp::Resp;
use crate::LuoshuData;
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
    let value = req.parse_body::<Namespace>().await?;
    let data = depot.obtain::<LuoshuData>().unwrap();
    data.namespace_store.write().await.append_namespace(value)?;
    res.render(Json(Resp::success("ok")));
    data.namespace_store.write().await.save()?;
    Ok(())
}

#[handler]
async fn list(_: &mut Request, res: &mut Response, depot: &mut Depot) -> WebResult<()> {
    let data = depot.obtain::<LuoshuData>().unwrap();
    res.render(Json(Resp::success(
        data.namespace_store.write().await.get_values(),
    )));
    Ok(())
}
