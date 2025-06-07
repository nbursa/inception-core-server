use crate::icore::llm;
use crate::icore::protocol::IcoreError;

pub async fn generate(prompt: &str) -> Result<String, IcoreError> {
    if prompt.trim().is_empty() {
        return Err(IcoreError::InvalidInput("prompt is empty".into()));
    }

    llm::generate_local(prompt)
        .await
        .map_err(|e| IcoreError::Model(e.to_string()))
}
