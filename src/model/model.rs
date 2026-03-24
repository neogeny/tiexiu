use crate::engine::ctx::{Ctx, ParseResult};

pub trait Model {
    fn parse(&self, ctx: &mut Ctx) -> ParseResult;
}

pub trait WithExp {
    fn exp(&self) -> &dyn Model;
}

// A "Blanket Implementation": 
// "For any type T that is a Wrapper, also implement Model for it"
impl<T: WithExp> Model for T {
    fn parse(&self, ctx: &mut Ctx) -> ParseResult {
        self.exp().parse(ctx)
    }
}