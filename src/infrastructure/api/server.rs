//! Server

use super::layers::{
    MakeRequestUuid, REQUEST_ID_HEADER,
    state::{SharedState, State},
};
use super::{layers, logger, routes};
use crate::adapters::database::GenericDb;
use crate::adapters::database::mysql::Db;
use crate::adapters::email::EmailAdapter;
use crate::config::Config;
use crate::domain::entities::email::EmailConfig;
use crate::infrastructure::api::errors::timeout_error;
use crate::infrastructure::api::response::ApiError;
use crate::infrastructure::api::use_cases::AppUseCases;
use axum::{Extension, Router, error_handling::HandleErrorLayer, middleware};
use std::time::Duration;
use tera::Tera;
use tokio::net::TcpListener;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::request_id::{PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::services::ServeDir;

/// Starts API server
pub async fn start_server() -> Result<(), ApiError> {
    // Load configuration
    let settings = Config::from_env()?;

    // Get router
    let app = get_app(&settings).await?;

    // Start server
    let addr = format!("{}:{}", settings.server_url, settings.server_port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Starting server on {}...", &addr);

    let server = axum::serve(listener, app);

    // Graceful shutdown only in production environment
    if settings.environment != "production" {
        server
            .await
            .map_err(|err| ApiError::InternalServerError(err.to_string()))
    } else {
        server
            .with_graceful_shutdown(shutdown_signal())
            .await
            .map_err(|err| ApiError::InternalServerError(err.to_string()))
    }
}

/// Initialize router
async fn get_app(settings: &Config) -> Result<Router, ApiError> {
    // Tracing
    logger::init(&settings.environment, &settings.logs_path, &settings.logs_file)?;

    // CORS
    let cors = layers::cors(settings);

    // Layers
    let layers = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(REQUEST_ID_HEADER.clone(), MakeRequestUuid))
        .layer(layers::logger::LoggerLayer)
        .layer(HandleErrorLayer::new(timeout_error))
        .timeout(Duration::from_secs(settings.request_timeout))
        .layer(PropagateRequestIdLayer::new(REQUEST_ID_HEADER.clone()));

    // Global state
    let global_state = SharedState::new(State::init(settings)?);

    // Routing - API
    let mut app = Router::new()
        .nest("/api/v1", routes::api(global_state.clone()))
        .layer(cors);

    // Routing - Web
    app = app.merge(routes::web(settings));

    // Templates
    let mut tera = Tera::new("templates/**/*")
        .map_err(|err| ApiError::InternalServerError(format!("error during template render: {}", err)))?;
    tera.autoescape_on(vec![".html", ".txt"]);
    app = app.layer(Extension(tera));

    // Database
    let db = Db::new(settings).await?;

    // Email service
    let email_service = EmailAdapter::new(EmailConfig::from(settings.clone()));

    app = app
        .fallback_service(ServeDir::new("assets").append_index_html_on_directories(true)) // FIXME: static_file_error not work this Axum 0.6.9!
        .layer(middleware::from_fn_with_state(
            global_state.clone(),
            layers::override_http_errors,
        ))
        .layer(layers)
        .layer(Extension(AppUseCases::new(db, email_service).await?));

    // State
    let app = app.with_state(global_state);

    Ok(app)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}
