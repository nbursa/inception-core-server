use anyhow::{Result, anyhow};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

const LLM_ENDPOINT: &str = "http://127.0.0.1:11434/completion";

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

pub fn generate_local(prompt: &str) -> Result<String> {
    if prompt.trim().is_empty() {
        return Err(anyhow!("prompt is empty"));
    }

    let req = LlamaRequest {
        prompt,
        n_predict: 256,
        stream: false,
    };

    let cli = Client::new();
    let res = cli
        .post(LLM_ENDPOINT)
        .json(&req)
        .send()
        .map_err(|e| anyhow!("HTTP error: {}", e))?
        .error_for_status()
        .map_err(|e| anyhow!("LLM server returned {}", e))?;

    let parsed: LlamaResponse = res.json().map_err(|e| anyhow!("bad JSON: {}", e))?;
    Ok(parsed.text.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_returns_text() {
        let out = generate_local("Explain Rust ownership model").unwrap();
        assert!(!out.is_empty());
    }
}
