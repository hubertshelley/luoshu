use luoshu::data::{ActionEnum, Connection, LuoshuDataHandle, LuoshuMemData, ServiceReg};
use luoshu_registry::Service;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = Arc::new(RwLock::new(LuoshuMemData::new()));
    let addr = "127.0.0.1:19998".to_string();
    let stream = TcpStream::connect(addr.clone()).await.unwrap();
    let mut connection = Connection::new(stream, addr.parse()?);
    // 生成服务数据
    let frame = ActionEnum::Up(
        ServiceReg::new(
            "default".into(),
            "test".into(),
            Service::new("127.0.0.1".into(), 8000),
        )
        .into(),
    )
    .into();
    connection.write_frame(&frame).await?;
    connection
        .write_frame(&ActionEnum::Subscribe("test".to_string()).into())
        .await?;

    loop {
        if let Some(frame) = connection.read_frame().await? {
            match frame.data {
                ActionEnum::Up(frame) => data.write().await.append(&frame).await?,
                ActionEnum::Down(frame) => data.write().await.remove(&frame).await?,
                ActionEnum::Sync(frame) => data.write().await.sync(&frame).await?,
                _ => {}
            };
        }
    }
}
