use crate::icore::llm;
use crate::icore::protocol::ICOREError;

pub async fn generate(prompt: &str) -> Result<String, ICOREError> {
    if prompt.trim().is_empty() {
        return Err(ICOREError::InvalidInput("prompt is empty".into()));
    }

    llm::generate_local(prompt).map_err(|e| ICOREError::Model(e.to_string()))
}
