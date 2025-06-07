use std::env;

use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct LlamaRequest<'a> {
    prompt: &'a str,
    n_predict: u16,
    stream: bool,
}

#[derive(Deserialize)]
struct LlamaResponse {
    #[serde(alias = "response", alias = "content")]
    text: String,
}

pub async fn generate_local(prompt: &str) -> Result<String> {
    if prompt.trim().is_empty() {
        return Err(anyhow!("prompt is empty"));
    }

    let req = LlamaRequest {
        prompt,
        n_predict: 128,
        stream: false,
    };

    let url = env::var("LLM_URL").map_err(|_| anyhow!("LLM_URL not set"))?;

    let cli = Client::new();
    let res = cli
        .post(&url)
        .json(&req)
        .send()
        .await
        .map_err(|e| anyhow!("HTTP error: {}", e))?
        .error_for_status()
        .map_err(|e| anyhow!("LLM server returned {}", e))?;

    let parsed: LlamaResponse = res.json().await.map_err(|e| anyhow!("bad JSON: {}", e))?;
    Ok(parsed.text.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn generate_returns_text() {
        let out = generate_local("Explain Rust ownership model")
            .await
            .unwrap();
        assert!(!out.is_empty());
    }
}
