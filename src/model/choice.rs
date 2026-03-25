use crate::engine::{Cst, Ctx};
use super::model::Model;


pub struct Choice {
    pub options: Vec<Box<dyn Model>>,
}

impl Model for Choice {
    fn parse(&self, ctx: Ctx) -> Result<(Ctx, Cst), String> {
        for option in &self.options {
            // Because Ctx is Copy/Value, 'ctx' here is the same for every iteration
            if let Ok((next_ctx, cst)) = option.parse(ctx) {
                return Ok((next_ctx, cst));
            }
        }
        Err("No option matched".to_string())
    }
}
