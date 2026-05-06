use std::{fs, path::PathBuf};

use axum::{
    Router,
    routing::{get, post},
};
use clap::Parser;
use sqlx::PgPool;
use thiserror::Error;
use tracing::info;

use crate::conf::AppConf;

mod api;
mod app;
mod conf;
mod db;
mod logging;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, action = clap::ArgAction::Count)]
    verbose: u8,
    #[arg(short, long)]
    conf: PathBuf,
}

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}

#[derive(Error, Debug)]
enum AppError {
    #[error("Database error")]
    DBError(#[from] sqlx::Error),
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

    let conf = AppConf::from_file(args.conf)?;

    let db_pool = db::init(&conf.db).await?;
    let shared_state = AppState { db_pool };

    let app = Router::new()
        .route("/isalive", get(api::routes::is_alive))
        .route("/recipes", post(api::routes::recipes::post))
        .route("/recipes/{recipe_id}", get(api::routes::recipes::get))
        .with_state(shared_state);

    let addr = format!("{}:{}", conf.http.ip, conf.http.port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // For integration tests, we need to know which port to call
    if let Ok(port_file) = std::env::var("TEST_LISTENING_PORT_FILE") {
        let port = listener.local_addr()?.port();
        fs::write(port_file, port.to_string())?;
    }

    axum::serve(listener, app).await?;

    Ok(())
}
