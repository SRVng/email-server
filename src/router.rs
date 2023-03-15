use crate::handler::health::health_handler;
use axum::{routing, Router};

pub fn public_router() -> Router {
    Router::new().route("/health", routing::get(health_handler))
}

pub fn app_router() -> Router {
    Router::new().nest("/api", public_router())
}
