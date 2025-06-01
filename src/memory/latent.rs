use reqwest::Client;
use serde::{Deserialize, Serialize};

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

    // 🧠 Dummy embed: koristi f32 vektor kao ulaz
    pub async fn embed(&self, id: &str, embedding: Vec<f32>) -> Result<(), String> {
        let collection_id = "1414cedf-3081-4235-ab29-656549bdff1a";
        let payload = serde_json::json!({
            "ids": [id],
            "embeddings": [embedding],
            "metadatas": [{"source": "stub"}]
        });

        let res = self
            .client
            .post(format!(
                "{}/api/v2/tenants/default_tenant/databases/default_database/collections/{}/add",
                self.chroma_url, collection_id
            ))
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(format!("Chroma embed error: {}", res.status()))
        }
    }

    pub async fn query(&self, embedding: Vec<f32>) -> Result<Vec<String>, String> {
        let collection_id = "1414cedf-3081-4235-ab29-656549bdff1a";
        let payload = serde_json::json!({
            "query_embeddings": [embedding],
            "n_results": 3
        });

        let res = self
            .client
            .post(format!(
                "{}/api/v2/tenants/default_tenant/databases/default_database/collections/{}/query",
                self.chroma_url, collection_id
            ))
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
