use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct LatentMemory {
    pub chroma_url: String,
    pub collection_id: String,
    client: Client,
}

impl LatentMemory {
    pub async fn new(chroma_url: String) -> Self {
        let client = Client::new();
        let path = ".chroma";

        let collection_id = if let Ok(env_id) = env::var("CHROMA_COLLECTION_ID") {
            env_id
        } else if Path::new(path).exists() {
            fs::read_to_string(path)
                .expect("failed to read .chroma")
                .trim()
                .to_string()
        } else {
            let payload = serde_json::json!({
                "name": "mem",
                "embedding_function": null,
                "dimension": 1536
            });

            let res = client
                .post(format!(
                    "{}/api/v2/tenants/default_tenant/databases/default_database/collections",
                    chroma_url
                ))
                .json(&payload)
                .send()
                .await
                .expect("failed to create collection");

            if res.status().is_success() {
                let text = res.text().await.expect("failed to get response text");
                let value: serde_json::Value =
                    serde_json::from_str(&text).expect("invalid json response");
                let id = value["id"]
                    .as_str()
                    .expect("missing id field in response")
                    .to_string();

                fs::write(path, &id).expect("failed to write .chroma");
                id
            } else {
                let err = res.text().await.unwrap_or_else(|_| "unknown error".into());
                panic!("failed to create collection: {}", err);
            }
        };

        Self {
            chroma_url,
            collection_id,
            client,
        }
    }

    pub async fn embed(&self, id: &str, embedding: Vec<f32>) -> Result<(), String> {
        let payload = serde_json::json!({
            "ids": [id],
            "embeddings": [embedding],
            "metadatas": [{"source": "stub"}]
        });

        let url = format!(
            "{}/api/v2/tenants/default_tenant/databases/default_database/collections/{}/add",
            self.chroma_url, self.collection_id
        );

        let resp = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_else(|_| "unknown error".into());

        if status.is_success() {
            Ok(())
        } else {
            Err(format!("Chroma embed error: {} - {}", status, text))
        }
    }

    pub async fn query(&self, embedding: Vec<f32>) -> Result<Vec<String>, String> {
        let payload = serde_json::json!({
            "query_embeddings": [embedding],
            "n_results": 3
        });

        let url = format!(
            "{}/api/v2/tenants/default_tenant/databases/default_database/collections/{}/query",
            self.chroma_url, self.collection_id
        );

        let resp = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_else(|_| "unknown error".into());

        if status.is_success() {
            let parsed: QueryResponse = serde_json::from_str(&text).map_err(|e| e.to_string())?;
            Ok(parsed.ids.into_iter().flatten().collect())
        } else {
            Err(format!("Chroma query error: {} - {}", status, text))
        }
    }
}

#[derive(Debug, Deserialize)]
struct QueryResponse {
    ids: Vec<Vec<String>>,
}
