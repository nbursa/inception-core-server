use crate::agents::sentience_wrapper::SentienceWrapper;
use crate::icore::context::Context;
use tokio::sync::Mutex;

pub struct BaseAgent {
    pub context: Context,
    sentience: Option<Mutex<SentienceWrapper>>,
}

impl BaseAgent {
    pub fn new() -> Self {
        BaseAgent {
            context: Context::new(),
            sentience: None,
        }
    }

    pub async fn load_sentience(&mut self, code: &str) -> Result<(), String> {
        let mut wrapper = SentienceWrapper::new();
        wrapper.load_program(code).await?;
        self.sentience = Some(Mutex::new(wrapper));
        Ok(())
    }

    fn flush_to_global_short(&self) {
        if let Some(global) = crate::api::handlers::SHORT_MEM.get() {
            for (k, v) in self.context.all() {
                global.set(k.clone(), v.clone());
            }
        }
    }

    pub async fn handle(&self, input: &str) -> Option<String> {
        let trimmed = input.trim();

        if let Some(wrapper_mutex) = &self.sentience {
            let mut guard = wrapper_mutex.lock().await;
            if let Some(output) = guard.handle_code(trimmed) {
                return Some(output);
            }
        }

        if let Some(rest) = trimmed.strip_prefix("remember ") {
            let parts: Vec<&str> = rest.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim();
                self.context.set(key, value);
                self.flush_to_global_short();
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

        if let Some(rest) = trimmed.strip_prefix("if context includes ") {
            let keyword = rest.trim();
            for (_k, val) in self.context.all() {
                if val.contains(keyword) {
                    return Some(format!("Context includes '{}'", keyword));
                }
            }
            return Some(format!("Context does not include '{}'", keyword));
        }

        self.flush_to_global_short();
        None
    }
}
