//! Web handler

use crate::infrastructure::api::response::ApiError;
use crate::infrastructure::api::TEMPLATES;
use axum::response::Html;
use tera::Context;

/// Health check route: GET "/health"
pub async fn health<'a>() -> &'a str {
    "OK"
}

/// API documentation route: GET "/doc/api-v1"
pub async fn doc_api_v1() -> Result<Html<String>, ApiError> {
    let templates = TEMPLATES
        .as_ref()
        .map_err(|err| ApiError::InternalServerError(format!("error during template render: {}", err)))?;
    Ok(Html(
        templates
            .render("doc/api_v1.html", &Context::new())
            .map_err(|err| ApiError::InternalServerError(err.to_string()))?,
    ))
}
