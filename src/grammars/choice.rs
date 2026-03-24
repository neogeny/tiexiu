use crate::model::ctx::{Ctx, ParseResult};
use crate::model::scope::StateScope;
use super::model::Model;

pub struct ChoiceOption {
    exp: Box<dyn Model>
}

pub(crate) struct Choice {
    pub options: Vec<ChoiceOption>,
}

impl Model for Choice {
    fn parse(&self, ctx: &mut Ctx) -> ParseResult {
        // Rust's version of with ctx.choice() as ch:
        let scope = StateScope::new(ctx);
        for option in &self.options {
            if let Ok(result) = option.parse(scope.ctx) {
                scope.merge();
                return Ok(result);
            }
        }
        Err("No option matched".to_string())
    }
}

impl ChoiceOption {
    pub fn new(exp: Box<dyn Model>) -> Self {
        Self { exp }
    }
}

impl Model for ChoiceOption {
    fn parse(&self, ctx: &mut Ctx) -> ParseResult {
        self.exp.parse(ctx)
    }
}