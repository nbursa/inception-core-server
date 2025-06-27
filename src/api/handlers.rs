use crate::agents::AGENT;
use crate::icore::context::Context;
use crate::icore::embed::embed_text;
use crate::memory::semantic::latent_graph::SEMANTIC_GRAPH;
use crate::memory::semantic::object::{AffectScore, ObjectCluster};
use crate::memory::semantic::reflect::reflect;
use crate::memory::{latent::LatentMemory, long_term::LongTermMemory, short_term::ShortTermMemory};
use axum::{
    debug_handler,
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::sync::Mutex;

pub static SHORT_MEM: OnceLock<ShortTermMemory> = OnceLock::new();
pub static LONG_MEM: OnceLock<LongTermMemory> = OnceLock::new();
pub static LATENT_MEM: OnceLock<Arc<Mutex<LatentMemory>>> = OnceLock::new();

fn mem() -> &'static ShortTermMemory {
    SHORT_MEM.get().expect("Short-term memory not initialized")
}

async fn long_mem() -> &'static LongTermMemory {
    LONG_MEM.get().expect("Long-term memory not initialized")
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

pub async fn embed_latent(Json(payload): Json<EmbedPayload>) -> impl IntoResponse {
    let Ok(vec) = embed_text(&payload.content).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let mem = LATENT_MEM.get().unwrap();
    let lock = mem.lock().await;
    match lock.query(vec).await {
        Ok(ids) => axum::Json(json!({ "ids": ids })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
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
    let lock = mem.lock().await;
    match lock.query(dummy_vec).await {
        Ok(ids) => Ok(axum::Json(ids)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

#[derive(Deserialize)]
pub struct ChatPayload {
    message: String,
}

#[debug_handler]
pub async fn agent_get_short(Path(key): Path<String>) -> impl IntoResponse {
    let agent = AGENT.get().unwrap().lock().await;
    match agent.get_short(&key) {
        Some(val) => (StatusCode::OK, val),
        None => (StatusCode::NOT_FOUND, "key not found".to_string()),
    }
}

#[debug_handler]
pub async fn agent_get_long(Path(key): Path<String>) -> impl IntoResponse {
    let agent = AGENT.get().unwrap().lock().await;
    match agent.get_long(&key) {
        Some(val) => (StatusCode::OK, val),
        None => (StatusCode::NOT_FOUND, "key not found".to_string()),
    }
}

#[debug_handler]
pub async fn agent_all_short() -> impl IntoResponse {
    let agent = AGENT.get().unwrap().lock().await;
    Json(agent.all_short().unwrap_or_default())
}

#[debug_handler]
pub async fn agent_all_long() -> impl IntoResponse {
    let agent = AGENT.get().unwrap().lock().await;
    Json(agent.all_long().unwrap_or_default())
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

    let mut ctx = Context::new();
    let output = agent.handle(&payload.code, &mut ctx).await;
    agent.flush_to_global_short(&mut ctx);
    agent.flush_to_global_long(&ctx).await;

    axum::Json(SentienceResponse {
        output: output.unwrap_or_else(|| "".to_string()),
    })
}

#[derive(Deserialize)]
pub struct SemanticPayload {
    pub id: String,
    pub embedding: Vec<f32>,
}

#[axum::debug_handler]
pub async fn embed_semantic(Json(payload): Json<SemanticPayload>) -> impl IntoResponse {
    let mut graph = SEMANTIC_GRAPH.lock().unwrap();
    let cluster = ObjectCluster {
        name: payload.id.clone(),
        embedding: payload.embedding,
        tags: vec![],
        affect: AffectScore::from_value(0.0),
        known: false,
    };
    graph.add_cluster(payload.id.clone(), cluster);
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}

#[axum::debug_handler]
pub async fn reflect_semantic(Path(id): Path<String>) -> impl IntoResponse {
    let graph = SEMANTIC_GRAPH.lock().unwrap();
    let result = reflect(&graph, &id);
    (StatusCode::OK, Json(result))
}

#[debug_handler]
pub async fn chat(Json(payload): Json<ChatPayload>) -> axum::Json<String> {
    let input = payload.message.trim();
    let timestamp = chrono::Utc::now().timestamp_millis();
    let id = format!("chat_{}", timestamp);

    let agent_lock = AGENT.get().unwrap().clone();
    let mut agent = agent_lock.lock().await;
    let mut ctx = Context::new();

    // 1. Embed input (llama.cpp)
    let embed_vec = match embed_text(input).await {
        Ok(vec) => vec,
        Err(e) => {
            eprintln!("Embedding failed: {}", e);
            vec![0.0; 1536]
        }
    };

    let _ = ctx.embed_latent(&id, embed_vec.clone()).await;

    // 2. Semantic cluster
    {
        let mut graph = SEMANTIC_GRAPH.lock().unwrap();
        let cluster = ObjectCluster {
            name: id.clone(),
            embedding: embed_vec,
            tags: vec!["chat_input".to_string()],
            affect: AffectScore::from_value(0.0),
            known: true,
        };
        graph.add_cluster(id.clone(), cluster);
    }

    // 3. Handle input
    let response = agent.handle(input, &mut ctx).await;
    let output = response
        .clone()
        .unwrap_or_else(|| "No response.".to_string());

    // 4. Persist to memory
    ctx.set_short("last_input", input);
    ctx.set_short("last_output", &output);
    ctx.set_long(&format!("input_{}", timestamp), input).await;
    ctx.set_long(&format!("output_{}", timestamp), &output)
        .await;

    agent.flush_to_global_short(&mut ctx);
    agent.flush_to_global_long(&ctx).await;

    axum::Json(output)
}
