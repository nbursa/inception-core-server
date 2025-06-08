use crate::agents::AGENT;
use crate::memory::latent::LatentMemory;
use crate::memory::long_term::LongTermMemory;
use crate::memory::short_term::ShortTermMemory;
use axum::debug_handler;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub static SHORT_MEM: OnceLock<ShortTermMemory> = OnceLock::new();
pub static LONG_MEM: OnceLock<LongTermMemory> = OnceLock::new();
pub static LATENT_MEM: OnceLock<LatentMemory> = OnceLock::new();

fn mem() -> &'static ShortTermMemory {
    SHORT_MEM.get_or_init(ShortTermMemory::new)
}

async fn long_mem() -> &'static LongTermMemory {
    LONG_MEM.get_or_init(|| panic!("LongTermMemory not initialized"))
}

pub async fn ping() -> &'static str {
    "pong"
}

pub async fn get_all_short_mem() -> impl IntoResponse {
    match mem().all() {
        Some(map) => Json(map).into_response(),
        None => (StatusCode::INTERNAL_SERVER_ERROR, "memory lock failed").into_response(),
    }
}

pub async fn get_short_mem(Path(key): Path<String>) -> impl IntoResponse {
    match mem().get(&key) {
        Some(val) => (StatusCode::OK, val),
        None => (StatusCode::NOT_FOUND, "key not found".to_string()),
    }
}

pub async fn get_long_mem(Path(key): Path<String>) -> impl IntoResponse {
    let mem = long_mem().await;
    match mem.get(&key).await {
        Some(val) => (StatusCode::OK, val),
        None => (StatusCode::NOT_FOUND, "key not found".to_string()),
    }
}

#[derive(Deserialize)]
pub struct SetValue {
    value: String,
}

pub async fn set_short_mem(
    Path(key): Path<String>,
    Json(payload): Json<SetValue>,
) -> impl IntoResponse {
    mem().set(key, payload.value);
    (StatusCode::OK, "stored")
}

pub async fn set_long_mem(
    Path(key): Path<String>,
    Json(payload): Json<SetValue>,
) -> impl IntoResponse {
    let mem = long_mem().await;
    mem.set(&key, &payload.value).await;
    (StatusCode::OK, "stored")
}

#[derive(Deserialize)]
pub struct EmbedPayload {
    id: String,
    content: String,
}

pub async fn embed_latent(Json(_payload): Json<EmbedPayload>) -> impl IntoResponse {
    let mem = LATENT_MEM.get().unwrap();
    let dummy_vec = vec![0.0; 1536];
    match mem.embed(&_payload.id, dummy_vec).await {
        Ok(_) => (StatusCode::OK, "embedded"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "error"),
    }
}

#[derive(Deserialize)]
pub struct QueryPayload {
    content: String,
}

pub async fn query_latent(
    Json(_payload): Json<QueryPayload>,
) -> Result<axum::Json<Vec<String>>, (StatusCode, String)> {
    let mem = LATENT_MEM.get().unwrap();
    let dummy_vec = vec![0.0; 1536];
    match mem.query(dummy_vec).await {
        Ok(ids) => Ok(axum::Json(ids)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

#[derive(Deserialize)]
pub struct ChatPayload {
    message: String,
}

#[debug_handler]
pub async fn chat(Json(payload): Json<ChatPayload>) -> axum::Json<String> {
    let agent_lock = AGENT.get().unwrap().clone();
    let mut agent = agent_lock.lock().await;

    let mut ctx = crate::icore::context::Context::new();
    let response = agent.handle(&payload.message, &mut ctx).await;

    axum::Json(response.unwrap_or_else(|| "No response.".to_string()))
}

#[derive(Deserialize)]
pub struct SentienceRequest {
    pub code: String,
}

#[derive(Serialize)]
pub struct SentienceResponse {
    pub output: String,
}

#[debug_handler]
pub async fn sentience_run_handler(
    Json(payload): Json<SentienceRequest>,
) -> axum::Json<SentienceResponse> {
    let agent_lock = AGENT.get().unwrap().clone();
    let mut agent = agent_lock.lock().await;

    let mut ctx = crate::icore::context::Context::new();
    if let Some(output) = agent.handle(&payload.code, &mut ctx).await {
        axum::Json(SentienceResponse { output })
    } else {
        axum::Json(SentienceResponse {
            output: "".to_string(),
        })
    }
}
