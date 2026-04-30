use std::{fs, path::PathBuf, sync::Arc};

use axum::{
    Router,
    extract::FromRef,
    response::IntoResponse,
    routing::{get, post},
};
use clap::Parser;
use reqwest::StatusCode;
use sqlx::PgPool;
use tracing::info;

use crate::conf::AppConf;

mod api;
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

#[derive(FromRef, Clone)]
struct AppState {
    db_pool: PgPool,
}

#[derive(Debug)]
enum AppError {
    DBError(sqlx::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self::DBError(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (code, body) = match self {
            AppError::DBError(_) => (StatusCode::INTERNAL_SERVER_ERROR, ""),
        };
        (code, body).into_response()
    }
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

    let db_pool = db::get_pool(&conf.db).await?;
    let shared_state = AppState { db_pool };

    let app = Router::new()
        .route("/isalive", get(api::isalive))
        .route("/recipes", post(api::recipes::post))
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
