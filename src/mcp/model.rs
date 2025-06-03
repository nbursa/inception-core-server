use crate::mcp::llm;
use crate::mcp::protocol::MCPError;

pub async fn generate(prompt: &str) -> Result<String, MCPError> {
    if prompt.trim().is_empty() {
        return Err(MCPError::InvalidInput("prompt is empty".into()));
    }

    llm::generate_local(prompt)
        .await
        .map_err(|e| MCPError::Model(e.to_string()))
}
