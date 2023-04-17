use clap::Parser;
use luoshu_rust_client::LuoshuClient;
use serde::{Deserialize, Serialize};
use std::thread::sleep;
use std::time::Duration;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// run with port
    #[arg(long, short)]
    port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    test: String,
    test2: Vec<usize>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut client = LuoshuClient::new(args.port, "test_rust_server".to_string(), None).await;
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
