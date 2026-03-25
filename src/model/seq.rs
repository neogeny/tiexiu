use crate::engine::{Cst, Ctx};
use super::model::Model;

pub struct Sequence {
    pub exps: Vec<Box<dyn Model>>,
}

impl Sequence {
    pub fn new(exps: Vec<Box<dyn Model>>) -> Self {
        Self { exps }
    }
    
}

impl Model for Sequence {
    fn parse(&self, mut ctx: Ctx) -> Result<(Ctx, Cst), String> {
        let mut results = Vec::new();
        for exp in &self.exps {
            let (new_ctx, cst) = exp.parse(ctx)?;
            ctx = new_ctx;
            results.push(cst)
        }
        Ok(
            (ctx, Cst::from(results))
        )
    }
}