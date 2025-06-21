use crate::api::handlers::{LATENT_MEM, LONG_MEM, SHORT_MEM};
use crate::memory::latent::LatentMemory;
use crate::memory::long_term::LongTermMemory;
use crate::memory::semantic::LatentGraph;
use crate::memory::short_term::ShortTermMemory;

#[derive(Clone)]
pub struct Context {
    pub mem_short: ShortTermMemory,
    pub mem_long: LongTermMemory,
    pub mem_latent: LatentMemory,
    pub mem_semantic: LatentGraph,
}

impl Context {
    pub fn new() -> Self {
        Self {
            mem_short: SHORT_MEM
                .get()
                .expect("short memory not initialized")
                .clone(),
            mem_long: LONG_MEM.get().expect("long memory not initialized").clone(),
            mem_latent: LATENT_MEM
                .get()
                .expect("latent memory not initialized")
                .clone(),
            mem_semantic: LatentGraph::default(),
        }
    }

    pub fn set_short(&self, key: &str, value: &str) {
        self.mem_short.set(key.to_string(), value.to_string());
    }

    pub fn get_short(&self, key: &str) -> Option<String> {
        self.mem_short.get(key)
    }

    pub fn all_short(&self) -> Vec<(String, String)> {
        self.mem_short
            .all()
            .unwrap_or_default()
            .into_iter()
            .collect()
    }

    pub async fn set_long(&self, key: &str, value: &str) {
        self.mem_long.set(key, value).await;
    }

    pub async fn get_long(&self, key: &str) -> Option<String> {
        self.mem_long.get(key).await
    }

    pub async fn embed_latent(&self, id: &str, vec: Vec<f32>) -> Result<(), String> {
        self.mem_latent.embed(id, vec).await
    }

    pub async fn query_latent(&self, vec: Vec<f32>) -> Result<Vec<String>, String> {
        self.mem_latent.query(vec).await
    }
}
