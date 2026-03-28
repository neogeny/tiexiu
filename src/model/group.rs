use crate::input::Cursor;
use crate::engine::{Cst, Ctx};
use super::model::{CanParse, ParseResult};

pub struct Group<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for Group<M> 
where
    M: CanParse<C>,
    C: Cursor
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        self.exp.parse(ctx)
    }
}


// #27
pub struct SkipGroup<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for SkipGroup<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        let (new_ctx, _) = self.exp.parse(ctx)?;
        Ok((new_ctx, Cst::Nil))
    }
}


