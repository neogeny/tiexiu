use crate::input::Cursor;
use crate::engine::{Cst, Ctx};
use super::*;

pub type ParseResult<C> = Result<(Ctx<C>, Cst), (Ctx<C>, String)>;

pub trait CanParse<C: Cursor> {
    fn parse(&self, _ctx: Ctx<C>) -> ParseResult<C> {
        unimplemented!()
    }
}

pub enum Model<M>
{
    Group(Group<M>),
    Sequence(Sequence<M>),
    Choice(Choice<M>),
    Optional(Optional<M>),
    Closure(Closure<M>),
    PositiveClosure(PositiveClosure<M>)
}

impl<M, C> CanParse<C> for Model<M>
where
    M: CanParse<C>,
    C: Cursor
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        match self {
            Self::Group(m) => m.parse(ctx),
            Self::Sequence(m) => m.parse(ctx),
            Self::Choice(m) => m.parse(ctx),
            Self::Optional(m) => m.parse(ctx),
            Self::Closure(m) => m.parse(ctx),
            Self::PositiveClosure(m) => m.parse(ctx),
            // Self::Token(m) => m.parse(ctx),
            // ...
        }
    }
}
