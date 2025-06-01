use crate::memory::latent::LatentMemory;
use crate::memory::long_term::LongTermMemory;
use crate::memory::short_term::ShortTermMemory;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
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

pub async fn ping() -> impl IntoResponse {
    (StatusCode::OK, "pong")
}

pub async fn get_short_mem(Path(key): Path<String>) -> impl IntoResponse {
    match mem().get(&key) {
        Some(val) => (StatusCode::OK, val).into_response(),
        None => (StatusCode::NOT_FOUND, "key not found").into_response(),
    }
}

pub async fn get_long_mem(Path(key): Path<String>) -> impl IntoResponse {
    let mem = long_mem().await;
    match mem.get(&key).await {
        Some(val) => (StatusCode::OK, val).into_response(),
        None => (StatusCode::NOT_FOUND, "key not found").into_response(),
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
    (StatusCode::OK, "stored").into_response()
}

pub async fn set_long_mem(
    Path(key): Path<String>,
    Json(payload): Json<SetValue>,
) -> impl IntoResponse {
    let mem = long_mem().await;
    mem.set(&key, &payload.value).await;
    (StatusCode::OK, "stored").into_response()
}

#[derive(Deserialize)]
pub struct EmbedPayload {
    id: String,
    content: String,
}

pub async fn embed_latent(Json(payload): Json<EmbedPayload>) -> impl IntoResponse {
    let mem = LATENT_MEM.get().unwrap();
    let dummy_vec = vec![0.0; 1536]; // stub
    match mem.embed(&payload.id, dummy_vec).await {
        Ok(_) => (StatusCode::OK, "embedded").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

#[derive(Deserialize)]
pub struct QueryPayload {
    content: String,
}

pub async fn query_latent(Json(payload): Json<QueryPayload>) -> impl IntoResponse {
    let mem = LATENT_MEM.get().unwrap();
    let dummy_vec = vec![0.0; 1536]; // stub
    match mem.query(dummy_vec).await {
        Ok(results) => Json(results).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}
