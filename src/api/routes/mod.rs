pub mod recipes;

use axum::http::StatusCode;

pub(crate) async fn is_alive() -> StatusCode {
    StatusCode::OK
}
