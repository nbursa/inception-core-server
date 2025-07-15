use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct EmbeddingRequest<'a> {
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    embedding: Vec<f32>,
}

pub async fn embed_text(prompt: &str) -> Result<Vec<f32>> {
    let client = Client::new();
    let res = client
        .post("http://127.0.0.1:11434/api/embeddings")
        .json(&EmbeddingRequest {
            model: "tinyllama",
            prompt,
        })
        .send()
        .await?;

    if res.status().is_success() {
        let result: EmbeddingResponse = res.json().await?;
        Ok(result.embedding)
    } else {
        Err(anyhow!("embedding failed: {}", res.status()))
    }
}
