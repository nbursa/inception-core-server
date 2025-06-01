use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct LatentMemory {
    pub chroma_url: String,
    client: Client,
}

impl LatentMemory {
    pub fn new(chroma_url: String) -> Self {
        Self {
            chroma_url,
            client: Client::new(),
        }
    }

    pub async fn embed(&self, id: &str, content: &str) -> Result<(), String> {
        let payload = serde_json::json!({
            "ids": [id],
            "documents": [content],
        });

        let res = self
            .client
            .post(format!("{}/collections/mem/docs", self.chroma_url))
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(format!("Chroma error: {}", res.status()))
        }
    }

    pub async fn query(&self, content: &str) -> Result<Vec<String>, String> {
        let payload = serde_json::json!({
            "query_texts": [content],
            "n_results": 3
        });

        let res = self
            .client
            .post(format!("{}/collections/mem/query", self.chroma_url))
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            let parsed: QueryResponse = res.json().await.map_err(|e| e.to_string())?;
            Ok(parsed.ids.into_iter().flatten().collect())
        } else {
            Err(format!("Chroma query error: {}", res.status()))
        }
    }
}

#[derive(Debug, Deserialize)]
struct QueryResponse {
    ids: Vec<Vec<String>>,
}
