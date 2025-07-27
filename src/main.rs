use axum::{Router, routing::get};
use clap::Parser;
use tracing::info;

mod handlers;
mod logging;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    logging::setup(args.verbose);

    info!(
        "Starting {} v{}",
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let app = Router::new().route("/isalive", get(handlers::isalive));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4242")
        .await
        .unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
