//! Routes list

use crate::auth;
use crate::config::Config;
use crate::infrastructure::api::handlers;
use crate::infrastructure::api::layers::auth::JwtLayer;
use crate::infrastructure::api::layers::basic_auth::BasicAuthLayer;
use crate::infrastructure::api::layers::state::SharedState;
use axum::Router;
use axum::routing::{delete, get, patch, post};

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
        .route("/refresh-token/{token}", post(handlers::user::refresh_token))
        .route("/forgotten-password/{email}", post(handlers::user::forgotten_password))
        .route("/update-password", patch(handlers::user::update_password_from_token))
        // Private routes
        .merge(api_protected(state))
}

/// Protected API routes
fn api_protected(state: SharedState) -> Router<SharedState> {
    Router::new()
        .nest("/users", api_users().layer(auth!(state.clone())))
        .nest("/applications", api_applications().layer(auth!(state.clone())))
        .nest("/scopes", api_scopes().layer(auth!(state.clone())))
        .nest("/external-links", api_external_links().layer(auth!(state.clone())))
}

/// Users API routes
fn api_users() -> Router<SharedState> {
    Router::new()
        .route("/", post(handlers::user::create))
        .route("/", get(handlers::user::get_all))
        .route("/deleted", get(handlers::user::get_all_deleted))
        .route("/{user_id}", get(handlers::user::get_by_id))
        .route("/{user_id}", delete(handlers::user::delete))
        .route("/{user_id}/restore", patch(handlers::user::restore))
}

/// Applications API routes
fn api_applications() -> Router<SharedState> {
    Router::new()
        .route("/", post(handlers::application::create))
        .route("/", get(handlers::application::get_all))
        .route("/deleted", get(handlers::application::get_all_deleted))
        .route("/{application_id}", get(handlers::application::get_by_id))
        .route("/{application_id}", patch(handlers::application::update))
        .route("/{application_id}", delete(handlers::application::delete))
        .route("/{application_id}/restore", patch(handlers::application::restore))
        .route("/{application_id}/scopes", post(handlers::scope::create))
}

/// Scopes API routes
fn api_scopes() -> Router<SharedState> {
    Router::new()
        .route("/", get(handlers::scope::get_all))
        .route("/deleted", get(handlers::scope::get_all_deleted))
        .route("/{scope_id}", delete(handlers::scope::delete))
        .route("/{scope_id}/restore", patch(handlers::scope::restore))
}

/// External links API routes
fn api_external_links() -> Router<SharedState> {
    Router::new()
        .route("/", post(handlers::external_link::create))
        .route("/", get(handlers::external_link::get_all))
        .route("/deleted", get(handlers::external_link::get_all_deleted))
        .route("/{external_link_id}", get(handlers::external_link::get_by_id))
        .route("/{external_link_id}", patch(handlers::external_link::update))
        .route("/{external_link_id}", delete(handlers::external_link::delete))
        .route("/{external_link_id}/restore", patch(handlers::external_link::restore))
}
