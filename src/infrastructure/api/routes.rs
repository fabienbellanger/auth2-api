//! Routes list

use crate::auth;
use crate::config::Config;
use crate::infrastructure::api::handlers;
use crate::infrastructure::api::layers::auth::JwtLayer;
use crate::infrastructure::api::layers::basic_auth::BasicAuthLayer;
use crate::infrastructure::api::layers::state::SharedState;
use axum::routing::{delete, get, patch, post};
use axum::Router;

/// Return web routes list
pub fn web(settings: &Config) -> Router<SharedState> {
    Router::new()
        .route("/health", get(handlers::web::health))
        // API documentation
        .nest(
            "/doc",
            Router::new()
                .route("/api-v1", get(handlers::web::doc_api_v1))
                .layer(BasicAuthLayer::new(
                    &settings.basic_auth_username,
                    &settings.basic_auth_password,
                )),
        )
}

/// Return API routes list
pub fn api(state: SharedState) -> Router<SharedState> {
    Router::new()
        // Public routes
        .route("/token", post(handlers::user::get_access_token))
        .route("/refresh-token/:token", post(handlers::user::refresh_token))
        .route("/forgotten-password/:email", post(handlers::user::forgotten_password))
        .route("/update-password", patch(handlers::user::update_password_from_token))
        // Private routes
        .nest("/", api_protected(state))
}

/// Protected API routes
fn api_protected(state: SharedState) -> Router<SharedState> {
    Router::new()
        .nest("/users", api_users().layer(auth!(state.clone())))
        .nest("/applications", api_applications().layer(auth!(state)))
}

/// Users API routes
fn api_users() -> Router<SharedState> {
    Router::new()
        .route("/", post(handlers::user::create))
        .route("/", get(handlers::user::get_all))
        .route("/:user_id", get(handlers::user::get_by_id))
        .route("/:user_id", delete(handlers::user::delete))
}

/// Applications API routes
fn api_applications() -> Router<SharedState> {
    Router::new()
        .route("/", post(handlers::application::create))
        .route("/", get(handlers::application::get_all))
        .route("/:application_id", get(handlers::application::get_by_id))
        .route("/:application_id", delete(handlers::application::delete))
        .route("/:application_id", patch(handlers::application::update))
}

/// Scopes API routes
fn _api_scopes() -> Router<SharedState> {
    Router::new()
}
