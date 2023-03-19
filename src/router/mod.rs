use crate::handler::{health::health_handler, send_email::send_email_handler};
use axum::{routing, Router};

use self::cors::get_cors_layers;

mod cors;

fn public_router() -> Router {
    Router::new()
        .route("/health", routing::get(health_handler))
        .route("/send_email", routing::post(send_email_handler))
}

pub fn app_router() -> Router {
    Router::new()
        .nest("/api", public_router())
        .layer(get_cors_layers())
}
