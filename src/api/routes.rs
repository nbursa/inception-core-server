use super::handlers;
use crate::api::handlers::chat;
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes() -> Router {
    Router::new()
        .route("/ping", get(handlers::ping))
        .route("/mem/short/:key", get(handlers::get_short_mem))
        .route("/mem/short/:key", post(handlers::set_short_mem))
        .route("/mem/long/:key", get(handlers::get_long_mem))
        .route("/mem/long/:key", post(handlers::set_long_mem))
        .route("/mem/latent/embed", post(handlers::embed_latent))
        .route("/mem/latent/query", post(handlers::query_latent))
        .route("/chat", post(chat))
}
