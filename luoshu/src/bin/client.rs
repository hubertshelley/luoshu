use luoshu::data::{ActionEnum, Connection, Frame, NamespaceReg};
// use std::time::Duration;
use tokio::net::TcpStream;
// use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:19998".to_string();
    let stream = TcpStream::connect(addr.clone()).await.unwrap();
    let mut connection = Connection::new(stream, addr.parse()?);
    let frame = Frame {
        action: ActionEnum::Up,
        data: NamespaceReg {
            name: "test".into(),
        }
        .into(),
    };
    connection.write_frame(&frame).await?;

    loop {
        if let Some(_frame) = connection.read_frame().await? {
            // match frame.action {
            //     ActionEnum::Up => data.append(&frame.data).await?,
            //     ActionEnum::Down => data.remove(&frame.data).await?,
            //     ActionEnum::Sync => todo!(),
            // };
        }
    }
}
