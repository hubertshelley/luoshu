use std::sync::Arc;

use luoshu_configuration::Configurator;
use salvo::{prelude::*};
use tokio::sync::RwLock;


use crate::LuoshuData;
use crate::web::error::WebResult;
use crate::web::resp::Resp;

// use crate::web::LUOSHU_DATA;

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
    let value = req.parse_body::<Configurator>().await?;
    let data = depot.obtain::<Arc<RwLock<LuoshuData>>>().unwrap();
    data.write().await.configuration_store.append_configurator(value)?;
    res.render(Json(Resp::success("ok")));
    Ok(())
}

#[handler]
async fn list(_: &mut Request, res: &mut Response, depot: &mut Depot) -> WebResult<()> {
    let data = depot.obtain::<Arc<RwLock<LuoshuData>>>().unwrap();
    let values: Vec<Configurator> = data
        .write()
        .await
        .configuration_store
        .values
        .values()
        .cloned()
        .collect();
    res.render(Json(Resp::success(values)));
    Ok(())
}
