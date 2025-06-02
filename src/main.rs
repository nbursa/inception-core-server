use axum::{Router, routing::get, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, fmt};

use crate::agents::{AGENT, agent::BaseAgent};
use crate::api::handlers::{LATENT_MEM, LONG_MEM};
use crate::mcp::context::Context;
use crate::memory::latent::LatentMemory;
use crate::memory::long_term::LongTermMemory;

mod agents;
mod api;
mod config;
mod mcp;
pub mod memory;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let settings = config::settings::Settings::new();
    tracing::info!("Starting MCP server in {} mode", settings.env);

    let long = LongTermMemory::new("memory.db").await;
    let latent = LatentMemory::new(settings.chromadb_url.clone()).await;

    LONG_MEM.set(long).unwrap();
    LATENT_MEM.set(latent).unwrap();

    let agent = BaseAgent::new();
    AGENT.set(agent).unwrap();

    // Context test
    let ctx = Context::new();
    ctx.set("mood", "curious");

    if let Some(value) = ctx.get("mood") {
        println!("Context test passed: mood = {}", value);
    } else {
        println!("Context test failed");
    }

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", api::routes::routes());

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on http://{}", addr);
    serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "MCP server is healthy."
}
