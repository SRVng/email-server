use crate::handler::health::health_handler;
use axum::{routing, Router};
use lambda_http::Body;

pub fn public_router() -> Router<(), Body> {
    Router::new().route("/health", routing::get(health_handler))
}

pub fn app_router() -> Router<(), Body> {
    Router::new().nest("/api", public_router())
}
