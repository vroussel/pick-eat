pub mod recipes;

use axum::http::StatusCode;

pub async fn is_alive() -> StatusCode {
    StatusCode::OK
}
