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
