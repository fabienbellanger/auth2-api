//! Web handler

use crate::infrastructure::api::response::ApiError;
use axum::response::Html;
use axum::Extension;
use tera::{Context, Tera};

/// Health check route: GET "/health"
pub async fn health<'a>() -> &'a str {
    "OK"
}

/// API documentation route: GET "/doc/api-v1"
pub async fn doc_api_v1(Extension(templates): Extension<Tera>) -> Result<Html<String>, ApiError> {
    Ok(Html(
        templates
            .render("doc/api_v1.html", &Context::new())
            .map_err(|err| ApiError::InternalServerError(err.to_string()))?,
    ))
}
