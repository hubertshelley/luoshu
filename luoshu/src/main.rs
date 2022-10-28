use clap::Parser;

mod web;

use web::run_server;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// admin port
    #[arg(short, long, default_value_t = 19999)]
    admin_port: u16,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    run_server(format!("0.0.0.0:{}", args.admin_port).as_str()).await;
    Ok(())
}
