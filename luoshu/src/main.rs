use clap::Parser;
use luoshu_core::Store;
use luoshu_namespace::Namespace;

mod data;
mod web;

use crate::data::LuoshuData;
use crate::web::run_server;

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
            .append_namespace(Namespace::new("default".into()))?;
        data.namespace_store.write().await.save()?;
    }

    if args.web {
        run_server("0.0.0.0:19999", data).await;
    }
    Ok(())
}
