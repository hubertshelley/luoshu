use clap::Parser;
use luoshu_core::Store;
use tokio::net::TcpListener;

use anyhow::Result;
use luoshu::data::{Connection, LuoshuData};
use luoshu::web::run_server;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    tracing_subscriber::fmt().init();

    let data = LuoshuData::new();

    data.configuration_store.write().await.load()?;
    data.namespace_store.write().await.load()?;
    if data.namespace_store.read().await.values.is_empty() {
        data.namespace_store
            .write()
            .await
            .append("default".into())?;
        data.namespace_store.write().await.save()?;
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
