use luoshu_rust_client::LuoshuClient;
use serde::{Deserialize, Serialize};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    test: String,
    test2: Vec<usize>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = LuoshuClient::new(15666, "test_rust_server".to_string(), None).await;
    client
        .subscribe_config(
            "test_config2".to_string(),
            |config: Config| println!("config changed:{:#?}", config),
            None,
        )
        .await?;
    tokio::spawn(async move {
        client.registry().await.expect("TODO: panic message");
    });
    loop {
        println!("sleep");
        sleep(Duration::from_secs(10))
    }
}
