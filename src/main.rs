use axum::{Router, routing::get, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, fmt};

mod api;
mod config;

#[tokio::main]
async fn main() {
    // Load .env variables
    dotenvy::dotenv().ok();

    // Init logger
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    // Load settings
    let settings = config::settings::Settings::new();
    tracing::info!("Starting MCP server in {} mode", settings.env);

    // Define routes
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", api::routes::routes());

    // Start server
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on http://{}", addr);
    serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "MCP server is healthy."
}
