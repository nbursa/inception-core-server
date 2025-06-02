use crate::mcp::context::Context;

#[derive(Debug)]
pub struct BaseAgent {
    context: Context,
}

impl BaseAgent {
    pub fn new() -> Self {
        BaseAgent {
            context: Context::new(),
        }
    }

    pub async fn handle(&self, input: &str) -> Option<String> {
        let trimmed = input.trim();

        if let Some(rest) = trimmed.strip_prefix("remember ") {
            let parts: Vec<&str> = rest.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim();
                self.context.set(key, value);
                return Some(format!("Okay, remembered {} = {}", key, value));
            }
        }

        if let Some(rest) = trimmed.strip_prefix("recall ") {
            let key = rest.trim();
            if let Some(val) = self.context.get(key) {
                return Some(format!("{} = {}", key, val));
            } else {
                return Some(format!("I don't remember '{}'", key));
            }
        }

        None
    }
}
