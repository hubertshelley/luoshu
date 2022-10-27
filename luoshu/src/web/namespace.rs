use salvo::{prelude::*, Error};
use tokio::sync::RwLock;

use luoshu_core::Store;
use luoshu_namespace::Namespace;
use crate::web::LuoshuData;
// use crate::web::LUOSHU_DATA;

pub fn get_routers() -> Router {
    Router::with_path("service")
        .post(append)
        .get(list)
    // .push(Router::with_path("delete").post(delete))
}

// #[handler]
// async fn delete(req: &mut Request, res: &mut Response) -> Result<(), Error> {
//     res.set_status_code(StatusCode::OK);
//     res.render("删除成功");
//     Ok(())
// }

#[handler]
async fn append(req: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<(), Error> {
    let value = req.parse_body::<Namespace>().await?;
    let data = depot.obtain::<RwLock<LuoshuData>>().unwrap();
    match data.write().await.namespace_store.append_namespace(value) {
        Ok(_) => { res.set_status_code(StatusCode::OK); }
        Err(_) => { res.set_status_code(StatusCode::BAD_REQUEST); }
    }
    Ok(())
}

#[handler]
async fn list(_: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<(), Error> {
    let data = depot.obtain::<RwLock<LuoshuData>>().unwrap();
    // match LUOSHU_DATA.write().await.namespace_store.load() {
    //     Ok(_) => { res.set_status_code(StatusCode::OK); }
    //     Err(_) => { res.set_status_code(StatusCode::BAD_REQUEST); }
    // }
    res.render(Json(data.write().await.namespace_store.get_values()));
    Ok(())
}
