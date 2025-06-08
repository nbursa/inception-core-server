use crate::icore::context::Context;
use sentience::evaluator::Evaluator;
use std::collections::HashMap;

#[derive(Clone)]
pub struct BaseAgent {
    pub name: String,
    pub goal: String,
    pub evaluator: Evaluator,
}

impl BaseAgent {
    pub fn new(name: String, goal: String) -> Self {
        BaseAgent {
            name,
            goal,
            evaluator: Evaluator::new(),
        }
    }

    pub fn load(&mut self, code: &str) -> Result<(), String> {
        self.evaluator
            .load_program(code)
            .map_err(|e| format!("Failed to load: {}", e))
    }

    pub async fn handle(&mut self, input: &str, ctx: &mut Context) -> Option<String> {
        self.evaluator.attach_short_mem(ctx.mem_short.clone());
        self.evaluator.attach_long_mem(ctx.mem_long.clone());
        self.evaluator.attach_latent_mem(ctx.mem_latent.clone());

        let result = self.evaluator.handle_input(input).await.ok()?;
        Some(result)
    }

    pub fn flush_to_global_short(&self, ctx: &mut Context) {
        if let Some(map) = self.evaluator.get_all_short() {
            for (k, v) in map {
                ctx.mem_short.set(k, v);
            }
        }
    }

    pub fn get_short(&self, key: &str) -> Option<String> {
        self.evaluator.get_short(key)
    }

    pub fn get_long(&self, key: &str) -> Option<String> {
        self.evaluator.get_long(key)
    }

    pub fn all_short(&self) -> Option<HashMap<String, String>> {
        self.evaluator.get_all_short()
    }

    pub fn all_long(&self) -> Option<HashMap<String, String>> {
        self.evaluator.get_all_long()
    }
}
