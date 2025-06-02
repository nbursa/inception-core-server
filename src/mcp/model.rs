use crate::mcp::protocol::MCPError;

pub async fn generate(prompt: &str) -> Result<String, MCPError> {
    if prompt.trim().is_empty() {
        Err(MCPError::InvalidInput("prompt is empty".into()))
    } else {
        Ok(format!("(LLM stub) {}", prompt))
    }
}
