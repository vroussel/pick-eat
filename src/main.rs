use std::{fs, path::PathBuf};

use axum::{
    Router,
    http::Request,
    routing::{get, post},
};
use clap::Parser;
use reqwest::StatusCode;
use sqlx::PgPool;
use thiserror::Error;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tower_livereload::LiveReloadLayer;
use tracing::info;

use crate::{conf::AppConf, images::ImageBank};

mod api;
mod app;
mod conf;
mod db;
mod images;
mod logging;
mod model;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    conf: PathBuf,
}

#[derive(Clone, Debug)]
struct AppState {
    db_pool: PgPool,
    image_bank: ImageBank,
}

#[derive(Error, Debug)]
enum AppError {
    #[error("Database error: {0}")]
    DB(#[from] sqlx::Error),
    #[error("Template rendering error: {0}")]
    Render(#[from] askama::Error),
    #[error("Form multipart error: {0}")]
    MultiPart(#[from] axum::extract::multipart::MultipartError),
    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
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
    let image_bank = ImageBank::new(&conf.images);
    let shared_state = AppState {
        db_pool,
        image_bank,
    };

    let app = Router::new()
        .route("/isalive", get(api::routes::is_alive))
        .route("/new-recipe", post(api::routes::recipes::post))
        .route("/recipes/{recipe_id}", get(api::routes::recipes::get))
        .route("/new-recipe", get(api::routes::recipes::new_recipe_form))
        .fallback(|| async { (StatusCode::NOT_FOUND, api::not_found_page()) })
        .with_state(shared_state)
        .layer(LiveReloadLayer::new())
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
    let listener = tokio::net::TcpListener::bind(addr).await?;

    // For integration tests, we need to know which port to call
    if let Ok(port_file) = std::env::var("TEST_LISTENING_PORT_FILE") {
        let port = listener.local_addr()?.port();
        fs::write(port_file, port.to_string())?;
    }

    axum::serve(listener, app).await?;

    Ok(())
}
