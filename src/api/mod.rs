use askama::Template;
use axum::response::{Html, IntoResponse};
use reqwest::StatusCode;
use tracing::error;

use crate::AppError;

pub mod model;
pub mod routes;

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        error!("{}", self);
        let (code, body) = match self {
            AppError::DBError(_) => (StatusCode::INTERNAL_SERVER_ERROR, ""),
            AppError::RenderError(_) => (StatusCode::INTERNAL_SERVER_ERROR, ""),
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
