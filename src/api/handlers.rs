use crate::memory::short_term::ShortTermMemory;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use std::sync::OnceLock;

static SHORT_MEM: OnceLock<ShortTermMemory> = OnceLock::new();

fn mem() -> &'static ShortTermMemory {
    SHORT_MEM.get_or_init(ShortTermMemory::new)
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
