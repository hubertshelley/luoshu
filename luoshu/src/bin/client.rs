use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::sleep;
use luoshu::data::{ActionEnum, Connection, Frame, NamespaceReg};

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
    loop {
        connection.write_frame(&frame).await?;
        sleep(Duration::from_secs(10)).await;
    }
}