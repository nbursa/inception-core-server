#[derive(Debug, Clone)]
pub struct BaseAgent;

impl BaseAgent {
    pub fn new() -> Self {
        BaseAgent
    }

    pub async fn handle(&self, input: &str) -> Option<String> {
        let normalized = input.trim().to_lowercase();

        match normalized.as_str() {
            "hello" | "hi" | "hey" => Some("Hi!".to_string()),
            _ => None,
        }
    }
}
