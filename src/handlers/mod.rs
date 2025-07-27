use axum::{http::StatusCode, response::IntoResponse};

pub(crate) async fn isalive() -> impl IntoResponse {
    StatusCode::OK
}
