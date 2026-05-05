use axum::response::IntoResponse;
use reqwest::StatusCode;

use crate::AppError;

pub mod model;
pub mod routes;

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (code, body) = match self {
            AppError::DBError(_) => (StatusCode::INTERNAL_SERVER_ERROR, ""),
        };
        (code, body).into_response()
    }
}
