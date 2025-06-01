use sqlx::{Pool, Row, Sqlite, sqlite::SqlitePoolOptions};

#[derive(Debug, Clone)]
pub struct LongTermMemory {
    pool: Pool<Sqlite>,
}

impl LongTermMemory {
    pub async fn new(database_url: &str) -> Self {
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(database_url)
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .connect_with(options)
            .await
            .expect("Failed to connect to SQLite");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS memories (
                id TEXT PRIMARY KEY,
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );",
        )
        .execute(&pool)
        .await
        .expect("Failed to create table");

        Self { pool }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        sqlx::query("SELECT value FROM memories WHERE key = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await
            .ok()?
            .map(|row| row.get("value"))
    }

    pub async fn set(&self, key: &str, value: &str) {
        sqlx::query(
            "INSERT INTO memories (id, key, value) VALUES (?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET value = excluded.value",
        )
        .bind(key)
        .bind(key)
        .bind(value)
        .execute(&self.pool)
        .await
        .ok();
    }

    pub async fn delete(&self, key: &str) {
        sqlx::query("DELETE FROM memories WHERE key = ?")
            .bind(key)
            .execute(&self.pool)
            .await
            .ok();
    }
}
