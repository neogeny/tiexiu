use crate::engine::ctx::{Ctx, ParseResult};
use crate::engine::cst::Cst;
use super::model::Model;

pub struct Sequence {
    pub exps: Vec<Box<dyn Model>>,
}

impl Sequence {
    /// Accepts a Vec of anything that can be turned into a Box<dyn Model>
    pub fn new(exps: Vec<Box<dyn Model>>) -> Self {
        Self { exps }
    }

    fn parse(&self, ctx: &mut Ctx) -> ParseResult {
        let mut results = Vec::new();
        for exp in &self.exps {
            results.push(exp.parse(ctx)?);
        }
        Ok(Cst::from(results))
    }
}