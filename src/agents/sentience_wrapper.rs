use crate::api::handlers::{LONG_MEM, SHORT_MEM};
use sentience::SentienceAgent;

pub struct SentienceWrapper {
    pub inner: SentienceAgent,
}

impl SentienceWrapper {
    pub fn new() -> Self {
        let mut agent = SentienceAgent::new();

        if let Some(short) = SHORT_MEM.get() {
            if let Some(all) = short.all() {
                for (k, v) in all {
                    agent.set_short(&k, &v);
                }
            }
        }

        Self { inner: agent }
    }

    pub async fn load_program(&mut self, code: &str) -> Result<(), String> {
        if let Some(long) = LONG_MEM.get() {
            let data = long.all().await;
            for (k, v) in data {
                self.inner.set_long(&k, &v);
            }
        }

        let _ = self.inner.run_sentience(code)?;
        self.flush_back();
        Ok(())
    }

    pub fn handle_code(&mut self, code: &str) -> Option<String> {
        let result = self.inner.run_sentience(code);
        self.flush_back();
        match result {
            Ok(s) if !s.trim().is_empty() => Some(s),
            _ => None,
        }
    }

    fn flush_back(&self) {
        if let Some(short) = SHORT_MEM.get() {
            for (k, v) in self.inner.all_short() {
                short.set(k, v);
            }
        }
    }
}
