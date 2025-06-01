use axum::{http::StatusCode, response::IntoResponse};

pub async fn ping() -> impl IntoResponse {
    (StatusCode::OK, "pong")
}
