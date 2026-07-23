use askama::Template;
use axum::response::{Html, IntoResponse};
use reqwest::StatusCode;
use tracing::error;

use crate::AppError;

pub mod routes;

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        error!("{}", self);
        let (code, body) = match self {
            AppError::DB(_) => (StatusCode::INTERNAL_SERVER_ERROR, ""),
            AppError::Render(_) => (StatusCode::INTERNAL_SERVER_ERROR, ""),
            AppError::MultiPart(_) => (StatusCode::INTERNAL_SERVER_ERROR, ""),
            AppError::Image(_) => (StatusCode::INTERNAL_SERVER_ERROR, ""),
            AppError::IO(_) => (StatusCode::INTERNAL_SERVER_ERROR, ""),
        };
        (code, body).into_response()
    }
}

#[derive(Template)]
#[template(path = "pages/404.html")]
struct NotFoundPage {}

pub fn not_found_page() -> Result<Html<String>, AppError> {
    Ok(Html(NotFoundPage {}.render()?))
}
