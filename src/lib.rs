use axum::{
    Router,
    routing::{get, post},
};
use tokio::{net::TcpListener, task::JoinHandle};

mod handlers;

pub fn run(listener: TcpListener) -> JoinHandle<Result<(), std::io::Error>> {
    let app = Router::new()
        .route("/isalive", get(handlers::isalive))
        .route("/recipes", post(handlers::recipes::post));
    tokio::spawn(async move { axum::serve(listener, app).await })
}
