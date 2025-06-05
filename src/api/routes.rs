use crate::api::handlers;
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
        .route("/chat", post(handlers::chat))
        .route("/sentience/run", post(handlers::sentience_run_handler))
}
