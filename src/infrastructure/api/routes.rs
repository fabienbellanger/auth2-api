//! Routes list

use crate::config::Config;
use crate::infrastructure::api::handlers;
use crate::infrastructure::api::layers::basic_auth::BasicAuthLayer;
use crate::infrastructure::api::layers::state::SharedState;
use axum::routing::{get, post};
use axum::Router;

/// Return web routes list
pub fn web(settings: &Config) -> Router<SharedState> {
    Router::new()
        .route("/health-check", get(handlers::web::health))
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
        // .route("/login", post(Users::login))
        // .route("/forgotten-password/:email", post(Users::forgotten_password))
        // .route("/update-password/:token", patch(Users::update_password))
        // .route("/refresh-token/:token", post(Users::refresh_token))
        // Private routes
        .nest("/", api_protected(state))
}

/// Protected API routes
fn api_protected(_state: SharedState) -> Router<SharedState> {
    Router::new().nest("/users", api_users())
}

/// Users API routes
fn api_users() -> Router<SharedState> {
    Router::new().route("/", post(handlers::user::create_user))
}

/// Scopes API routes
fn _api_scopes() -> Router<SharedState> {
    Router::new()
}