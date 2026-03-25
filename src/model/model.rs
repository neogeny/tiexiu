use crate::engine::{Cst, Ctx};

pub trait Model {
    fn parse(&self, mut _ctx: Ctx) -> Result<(Ctx, Cst), String> {
        unimplemented!()
    }
}

pub trait WithExp {
    fn exp(&self) -> &dyn Model;
}

impl<T: WithExp> Model for T {
    fn parse(&self, ctx: Ctx) -> Result<(Ctx, Cst), String> {
        self.exp().parse(ctx)
    }
}