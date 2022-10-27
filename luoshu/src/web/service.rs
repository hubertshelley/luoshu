use salvo::{prelude::*, Error};

use luoshu_registry::{Registry, RegistryStore};

pub fn get_routers() -> Router {
    Router::with_path("service")
        .post(append)
        .get(list)
        .push(Router::with_path("delete").post(delete))
}

#[handler]
async fn delete(req: &mut Request, res: &mut Response) -> Result<(), Error> {
    res.set_status_code(StatusCode::OK);
    res.render("删除成功");
    Ok(())
}

#[handler]
async fn append(req: &mut Request, res: &mut Response) -> Result<(), Error> {
    let service = req.parse_body::<Registry>().await?;
    SERVICEBOOK
        .write()
        .await
        .join_service(
            service.name.to_string(),
            GatewayService {
                host: service.host,
                port: service.port,
                name: service.name,
            },
        )
        .unwrap();
    res.set_status_code(StatusCode::OK);
    Ok(())
}

#[handler]
async fn list(_: &mut Request, res: &mut Response) -> Result<(), Error> {
    let services = SERVICEBOOK.read().await.clone();
    let mut out_services: Vec<MyService> = vec![];
    for (_, v) in services.iter() {
        for service in v.iter() {
            let host = service.host.clone();
            let name = service.name.clone();
            out_services.push(MyService {
                host,
                port: service.port,
                name,
            });
        }
    }
    res.render(Json(out_services));
    Ok(())
}
