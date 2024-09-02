//! API response module

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

/// API response success
#[derive(Debug, Clone)]
pub struct ApiSuccess<T: Serialize + PartialEq>(StatusCode, Json<T>);

impl<T> PartialEq for ApiSuccess<T>
where
    T: Serialize + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 .0 == other.1 .0
    }
}

impl<T: Serialize + PartialEq> ApiSuccess<T> {
    pub fn new(status: StatusCode, data: T) -> Self {
        ApiSuccess(status, Json(data))
    }
}

impl<T: Serialize + PartialEq> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

/// The response data format for all error responses.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ApiErrorData {
    pub message: String,
}

/// Generic response structure shared by all API responses.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ApiResponseBody<T: Serialize + PartialEq> {
    code: u16,
    data: T,
}

impl<T: Serialize + PartialEq> ApiResponseBody<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self {
            code: status_code.as_u16(),
            data,
        }
    }
}

impl ApiResponseBody<ApiErrorData> {
    pub fn new_error(status_code: StatusCode, message: String) -> Self {
        Self {
            code: status_code.as_u16(),
            data: ApiErrorData { message },
        }
    }
}

/// API error
#[derive(Debug, Clone, PartialEq)]
pub enum ApiError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    UnprocessableEntity(String),
    InternalServerError(String),
    Timeout,
    TooManyRequests,
    MethodNotAllowed,
    PayloadTooLarge,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Timeout => (
                StatusCode::REQUEST_TIMEOUT,
                Json(ApiResponseBody::new_error(
                    StatusCode::REQUEST_TIMEOUT,
                    "Request timeout".to_string(),
                )),
            )
                .into_response(),
            ApiError::TooManyRequests => (
                StatusCode::TOO_MANY_REQUESTS,
                Json(ApiResponseBody::new_error(
                    StatusCode::TOO_MANY_REQUESTS,
                    "Too many requests".to_string(),
                )),
            )
                .into_response(),
            ApiError::MethodNotAllowed => (
                StatusCode::METHOD_NOT_ALLOWED,
                Json(ApiResponseBody::new_error(
                    StatusCode::METHOD_NOT_ALLOWED,
                    "Method not allowed".to_string(),
                )),
            )
                .into_response(),
            ApiError::PayloadTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                Json(ApiResponseBody::new_error(
                    StatusCode::PAYLOAD_TOO_LARGE,
                    "Payload too large".to_string(),
                )),
            )
                .into_response(),
            ApiError::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                Json(ApiResponseBody::new_error(StatusCode::BAD_REQUEST, message)),
            )
                .into_response(),
            ApiError::Unauthorized(message) => (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponseBody::new_error(StatusCode::UNAUTHORIZED, message)),
            )
                .into_response(),
            ApiError::Forbidden(message) => (
                StatusCode::FORBIDDEN,
                Json(ApiResponseBody::new_error(StatusCode::FORBIDDEN, message)),
            )
                .into_response(),
            ApiError::NotFound(message) => (
                StatusCode::NOT_FOUND,
                Json(ApiResponseBody::new_error(StatusCode::NOT_FOUND, message)),
            )
                .into_response(),
            ApiError::UnprocessableEntity(message) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponseBody::new_error(StatusCode::UNPROCESSABLE_ENTITY, message)),
            )
                .into_response(),
            ApiError::InternalServerError(message) => {
                // Log the error
                error!("{}", message);

                // Return the response
                (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiErrorData { message })).into_response()
            }
        }
    }
}
