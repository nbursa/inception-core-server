use crate::icore::context::Context;
use sentience::{
    context::AgentContext, eval::eval, lexer::Lexer, parser::Parser, types::Statement,
};
use std::collections::HashMap;

pub struct BaseAgent {
    pub name: String,
    pub goal: String,
    ctx: AgentContext,
}

impl BaseAgent {
    pub fn new(name: String, goal: String) -> Self {
        BaseAgent {
            name,
            goal,
            ctx: AgentContext::new(),
        }
    }

    pub async fn load(&mut self, code: &str) -> Result<(), String> {
        let mut lexer = Lexer::new(code.trim());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program();
        let mut output = Vec::new();
        for stmt in program.statements {
            eval(&stmt, "", "", &mut self.ctx, &mut output);
        }

        dbg!(&self.ctx.current_agent);

        Ok(())
    }

    pub async fn handle_core(&mut self, input: &str) -> Option<String> {
        self.ctx.set_mem("short", "msg", input);

        if let Some(Statement::AgentDeclaration { body, .. }) = self.ctx.current_agent.clone() {
            let mut output = Vec::new();
            for stmt in body {
                if let Statement::OnInput { body, .. } = stmt {
                    for inner in body {
                        eval(&inner, "", input, &mut self.ctx, &mut output);
                    }
                    return self.ctx.output.clone();
                }
            }
        }
        None
    }

    pub async fn handle(&mut self, input: &str, ctx: &mut Context) -> Option<String> {
        let output = self.handle_core(input).await;
        self.flush_to_global_short(ctx);
        self.flush_to_global_long(ctx).await;
        output
    }

    pub fn flush_to_global_short(&self, ctx: &mut Context) {
        for (k, v) in self.ctx.mem_short.iter() {
            ctx.mem_short.set(k.clone(), v.clone());
        }
    }

    pub async fn flush_to_global_long(&self, ctx: &Context) {
        for (k, v) in self.ctx.mem_long.iter() {
            ctx.mem_long.set(k, v).await;
        }
    }

    pub fn get_short(&self, key: &str) -> Option<String> {
        Some(self.ctx.get_mem("short", key))
    }

    pub fn get_long(&self, key: &str) -> Option<String> {
        Some(self.ctx.get_mem("long", key))
    }

    pub fn all_short(&self) -> Option<HashMap<String, String>> {
        Some(self.ctx.mem_short.clone())
    }

    pub fn all_long(&self) -> Option<HashMap<String, String>> {
        Some(self.ctx.mem_long.clone())
    }
}
