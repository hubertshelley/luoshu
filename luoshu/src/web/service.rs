use std::sync::Arc;

use salvo::{prelude::*, Error};
use tokio::sync::RwLock;

use crate::web::LuoshuData;
use luoshu_core::Store;
use luoshu_registry::Registry;

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

#[handler]
async fn append(req: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<(), Error> {
    let value = req.parse_body::<Registry>().await?;
    let data = depot.obtain::<Arc<RwLock<LuoshuData>>>().unwrap();
    match data.write().await.service_store.append_registry(value) {
        Ok(_) => {
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
    match data.write().await.service_store.load() {
        Ok(_) => {
            res.set_status_code(StatusCode::OK);
        }
        Err(_) => {
            res.set_status_code(StatusCode::BAD_REQUEST);
        }
    }
    res.render(Json(data.write().await.service_store.get_values()));
    Ok(())
}
