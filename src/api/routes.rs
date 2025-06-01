use axum::{Router, routing::get};

use super::handlers;

pub fn routes() -> Router {
    Router::new().route("/ping", get(handlers::ping))
}
