use std::{fs, path::PathBuf};

use axum::{
    Router,
    http::Request,
    routing::{get, post},
};
use clap::Parser;
use sqlx::PgPool;
use thiserror::Error;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
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
    #[arg(short, long)]
    conf: PathBuf,
}

#[derive(Clone, Debug)]
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
    logging::setup();

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
        .with_state(shared_state)
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
                .layer(
                    TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                        let request_id = request
                            .headers()
                            .get("x-request-id")
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or("unknown");

                        tracing::debug_span!(
                            "request",
                            method = %request.method(),
                            uri = %request.uri(),
                            request_id = %request_id,
                        )
                    }),
                )
                .layer(PropagateRequestIdLayer::x_request_id()),
        );

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
