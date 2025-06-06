use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub env: String,
    pub chromadb_url: String,
    pub database_url: String,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            env: env::var("ICORE_ENV").unwrap_or_else(|_| "development".into()),
            chromadb_url: env::var("CHROMADB_URL").expect("CHROMADB_URL is not set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL is not set"),
        }
    }
}
