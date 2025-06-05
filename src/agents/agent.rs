use crate::agents::sentience_wrapper::SentienceWrapper;
use crate::ICORE::context::Context;
use std::sync::Mutex;

pub struct BaseAgent {
    context: Context,
    sentience: Option<Mutex<SentienceWrapper>>,
}

impl BaseAgent {
    pub fn new() -> Self {
        BaseAgent {
            context: Context::new(),
            sentience: None,
        }
    }

    pub fn load_sentience(&mut self, code: &str) -> Result<(), String> {
        let mut wrapper = SentienceWrapper::new();
        wrapper.load_program(code)?;
        self.sentience = Some(Mutex::new(wrapper));
        Ok(())
    }

    pub async fn handle(&self, input: &str) -> Option<String> {
        let trimmed = input.trim();

        if let Some(wrapper_mutex) = &self.sentience {
            let mut guard = wrapper_mutex.lock().unwrap();
            let code = format!("on input({}) {{ }}", trimmed);
            let _ = guard.handle_code(&code);
            let resp = guard.inner.get_short("response");
            if !resp.is_empty() {
                return Some(resp);
            }
        }

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

        if let Some(rest) = trimmed.strip_prefix("if context includes ") {
            let keyword = rest.trim();
            for (_k, val) in self.context.all() {
                if val.contains(keyword) {
                    return Some(format!("Context includes '{}'", keyword));
                }
            }
            return Some(format!("Context does not include '{}'", keyword));
        }

        None
    }
}
