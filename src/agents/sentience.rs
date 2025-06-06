use crate::icore::context::Context;
use crate::memory::latent::LatentMemory;
use crate::memory::long_term::LongTermMemory;
use crate::memory::short_term::ShortTermMemory;
use sentience::ast::AST;
use sentience::error::SentienceError;
use sentience::evaluator::Evaluator;
use sentience::parser::Parser;

pub struct SentienceAgent {
    evaluator: Evaluator,
}

impl SentienceAgent {
    pub fn new() -> Self {
        let mut evaluator = Evaluator::new();
        Self { evaluator }
    }

    pub async fn run_sentience(
        &mut self,
        code: &str,
        context: &mut Context,
    ) -> Result<String, SentienceError> {
        let ast = Parser::parse(code)?;

        self.evaluator.attach_short_mem(context.mem_short.clone());
        self.evaluator.attach_long_mem(context.mem_long.clone());
        self.evaluator.attach_latent_mem(context.mem_latent.clone());

        let output = self.evaluator.evaluate(&ast).await?;

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::icore::context::Context;
    use crate::memory::latent::LatentMemory;
    use crate::memory::long_term::LongTermMemory;
    use crate::memory::short_term::ShortTermMemory;

    #[tokio::test]
    async fn test_sentience_basic() {
        let mut context = Context {
            mem_short: ShortTermMemory::new(),
            mem_long: LongTermMemory::new(),
            mem_latent: LatentMemory::new("dummy-collection-id"),
        };

        let mut agent = SentienceAgent::new();
        let code = r#"
            mem.short["foo"] = "bar";
            reflect { mem.short["foo"] }
        "#;

        let result = agent.run_sentience(code, &mut context).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "bar".to_string());
    }
}
