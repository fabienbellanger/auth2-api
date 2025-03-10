//! Custom Axum extractors

use crate::domain::entities::access_token::AccessToken;
use crate::domain::services::security::jwt::Jwt;
use crate::domain::services::security::payload::{Payload, PayloadError, PayloadExtractor};
use crate::domain::value_objects::datetime::UtcDateTime;
use crate::infrastructure::api::response::ApiError;
use axum::extract::FromRequestParts;
use axum::extract::path::ErrorKind;
use axum::extract::rejection::PathRejection;
use axum::http::{HeaderMap, StatusCode, header, header::HeaderValue, request::Parts};
use serde::de::DeserializeOwned;

/// Request ID extractor from HTTP headers
pub struct ExtractRequestId(pub HeaderValue);

impl<S> FromRequestParts<S> for ExtractRequestId
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match parts.headers.get("x-request-id") {
            Some(id) => Ok(ExtractRequestId(id.clone())),
            _ => Ok(ExtractRequestId(HeaderValue::from_static(""))),
        }
    }
}

impl PayloadExtractor<HeaderMap> for Payload {
    fn try_from_headers(headers: &HeaderMap, jwt: &Jwt) -> Result<Payload, PayloadError> {
        let result = headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| {
                let words = h.split("Bearer").collect::<Vec<&str>>();
                words.get(1).map(|w| w.trim())
            })
            .map(|token| AccessToken::new(token.to_string(), UtcDateTime::now()))
            .map(|token| jwt.parse(&token));

        match result {
            Some(Ok(payload)) => Ok(payload),
            Some(Err(err)) => Err(PayloadError::ParseTokenError(err.to_string())),
            None => Err(PayloadError::MissingToken),
        }
    }
}

// /// JWT extractor from HTTP headers
// pub struct ExtractJWT(pub Option<String>);
//
//
// impl<S> FromRequestParts<S> for ExtractJWT
// where
//     S: Send + Sync,
// {
//     type Rejection = ();
//
//     async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
//         match parts.headers.get(header::AUTHORIZATION) {
//             Some(token) => {
//                 let token = token
//                     .to_str()
//                     .unwrap_or_default()
//                     .to_string()
//                     .strip_prefix("Bearer ")
//                     .map(|s| s.to_string());
//                 Ok(Self(token))
//             }
//             _ => Ok(Self(None)),
//         }
//     }
// }

// We define our own `Path` extractor that customizes the error from `axum::extract::Path`
pub struct Path<T>(pub T);

impl<S, T> FromRequestParts<S> for Path<T>
where
    // these trait bounds are copied from `impl FromRequest for axum::extract::path::Path`
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = (StatusCode, ApiError);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let (status, body) = match rejection {
                    PathRejection::FailedToDeserializePathParams(inner) => {
                        let mut status = StatusCode::BAD_REQUEST;

                        let kind = inner.into_kind();
                        let body = match &kind {
                            ErrorKind::WrongNumberOfParameters { .. } => ApiError::BadRequest(kind.to_string()),

                            ErrorKind::ParseErrorAtKey { .. } => ApiError::BadRequest(kind.to_string()),

                            ErrorKind::ParseErrorAtIndex { .. } => ApiError::BadRequest(kind.to_string()),

                            ErrorKind::ParseError { .. } => ApiError::BadRequest(kind.to_string()),

                            ErrorKind::InvalidUtf8InPathParam { .. } => ApiError::BadRequest(kind.to_string()),

                            ErrorKind::UnsupportedType { .. } => {
                                // this error is caused by the programmer using an unsupported type
                                // (such as nested maps) so respond with `500` instead
                                status = StatusCode::INTERNAL_SERVER_ERROR;
                                ApiError::InternalServerError(kind.to_string())
                            }

                            ErrorKind::Message(msg) => ApiError::BadRequest(msg.clone()),

                            _ => ApiError::BadRequest(format!("Unhandled deserialization error: {kind}")),
                        };

                        (status, body)
                    }
                    PathRejection::MissingPathParams(error) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ApiError::InternalServerError(error.to_string()),
                    ),
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ApiError::InternalServerError(format!("Unhandled path rejection: {rejection}")),
                    ),
                };

                Err((status, body))
            }
        }
    }
}

// We define our own `Query` extractor that customizes the error from `axum::extract::Query`
pub struct Query<T>(pub T);

impl<T, S> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = (StatusCode, ApiError);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap_or_default();
        let value = serde_urlencoded::from_str(query)
            .map_err(|err| (StatusCode::BAD_REQUEST, ApiError::BadRequest(err.to_string())))?;

        Ok(Query(value))
    }
}
