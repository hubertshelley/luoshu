use clap::Parser;
use luoshu_core::Store;
use std::sync::Arc;
use tokio::net::TcpListener;

use anyhow::Result;
use luoshu::data::{Connection, LuoshuSledData};
use luoshu::web::run_server;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// run with web
    #[arg(long, default_value_t = false)]
    web: bool,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

/// 全局存储文件配置
static DB: Lazy<Arc<RwLock<LuoshuSledData>>> =
    Lazy::new(|| Arc::new(RwLock::new(LuoshuSledData::new())));

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    tracing_subscriber::fmt().init();

    let data = DB.clone();

    data.write().await.configuration_store.load()?;
    data.write().await.namespace_store.load()?;
    if data.write().await.namespace_store.values.is_empty() {
        data.write()
            .await
            .namespace_store
            .append("default".into())?;
        data.write().await.namespace_store.save()?;
    }
    let _data = data.clone();
    tokio::task::spawn(async move {
        if args.web {
            run_server("0.0.0.0:19999", _data).await;
        };
    });
    let listener = TcpListener::bind("0.0.0.0:19998").await?;
    tracing::info!("Luoshu listening on: http://0.0.0.0:19998");
    loop {
        let (stream, client) = listener.accept().await?;
        let _data = data.clone();
        tokio::task::spawn(async move {
            let mut connection = Connection::new(stream, client);
            match connection.process(_data).await {
                Ok(_) => {}
                Err(_) => {
                    tracing::info!("{} left", connection.client)
                }
            };
        });
    }
}
