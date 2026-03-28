use crate::input::Cursor;
use crate::engine::{Cst, Ctx};
use super::model::CanParse;

pub struct Sequence<M> {
    pub exps: Vec<Box<M>>,
}

impl<M, C> CanParse<C> for Sequence<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> Result<(Ctx<C>, Cst), (Ctx<C>, String)> {
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