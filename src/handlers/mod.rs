use axum::{http::StatusCode, response::IntoResponse};

pub mod recipes;

pub(crate) async fn isalive() -> impl IntoResponse {
    StatusCode::OK
}
