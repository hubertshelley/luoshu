use std::sync::Arc;
use clap::Parser;
use tokio::sync::RwLock;
use luoshu_core::Store;

mod web;
mod data;

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

    let data = Arc::new(RwLock::new(LuoshuData::new()));

    data.write().await.configuration_store.load()?;
    data.write().await.namespace_store.load()?;

    if args.web {
        run_server("0.0.0.0:19999", data).await;
    }
    Ok(())
}
