use crate::agents::{AGENT, BaseAgent};
use crate::api::handlers::{LATENT_MEM, LONG_MEM, SHORT_MEM};
use crate::api::routes::routes;
use crate::memory::latent::LatentMemory;
use crate::memory::long_term::LongTermMemory;
use crate::memory::short_term::ShortTermMemory;
use axum::{Router, http::Method, routing::get, serve};
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
use tracing_subscriber::{EnvFilter, fmt};

mod agents;
mod api;
mod config;
mod icore;
pub mod memory;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let settings = config::settings::Settings::new();
    tracing::info!("Starting ICORE server in {} mode", settings.env);

    SHORT_MEM.set(ShortTermMemory::new()).unwrap();
    LONG_MEM
        .set(LongTermMemory::new(&settings.database_url).await)
        .unwrap();
    LATENT_MEM
        .set(LatentMemory::new(settings.chromadb_url.clone()).await)
        .unwrap();

    let mut base_agent = BaseAgent::new();
    match fs::read_to_string("agent.sent") {
        Ok(sent_code) => {
            base_agent
                .load_sentience(&sent_code)
                .await
                .expect("Sentience load failed");
            println!("Loaded Sentience DSL from agent.sent");
        }
        Err(_) => {
            println!("Warning: 'agent.sent' not found, running w/o Sentience DSL");
        }
    }

    let agent_arc = Arc::new(base_agent);
    if AGENT.set(agent_arc).is_err() {
        panic!("AGENT was already set");
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(AllowHeaders::mirror_request());

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", routes())
        .layer(cors);

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on http://{}", addr);
    serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "ICORE server is healthy."
}
