use crate::input::Cursor;
use crate::engine::{Cst, Ctx};
use super::*;

pub type ParseResult<C> = Result<(Ctx<C>, Cst), Ctx<C>>;

pub trait CanParse<C: Cursor> {
    fn parse(&self, _ctx: Ctx<C>) -> ParseResult<C> {
        unimplemented!()
    }
}

pub enum Model<M>
{
    Cut(Cut),
    Void(Void),
    Fail(Fail),
    Dot(Dot),
    Eof(Eof),
    Token(Token),
    Constant(Constant),
    Alert(Alert),

    Named(Named<M>),
    NamedList(NamedList<M>),
    Override(Override<M>),
    OverrideList(OverrideList<M>),

    Group(Group<M>),
    SkipGroup(SkipGroup<M>),
    Sequence(Sequence<M>),
    Choice(Choice<M>),
    Optional(Optional<M>),
    Closure(Closure<M>),
    PositiveClosure(PositiveClosure<M>),
    Join(Join<M>),
    PositiveJoin(PositiveJoin<M>),
    Gather(Gather<M>),
    PositiveGather(PositiveGather<M>),

    Lookahead(Lookahead<M>),
    NegativeLookahead(NegativeLookahead<M>),
    SkipTo(SkipTo<M>)
}

impl<M, C> CanParse<C> for Model<M>
where
    M: CanParse<C>,
    C: Cursor
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        match self {
            Self::Cut(m) => m.parse(ctx),
            Self::Void(m) => m.parse(ctx),
            Self::Fail(m) => m.parse(ctx),
            Self::Dot(m) => m.parse(ctx),
            Self::Eof(m) => m.parse(ctx),
            Self::Token(m) => m.parse(ctx),
            Self::Constant(m) => m.parse(ctx),
            Self::Alert(m) => m.parse(ctx),
            
            Self::Named(m) => m.parse(ctx),
            Self::NamedList(m) => m.parse(ctx),
            Self::Override(m) => m.parse(ctx),
            Self::OverrideList(m) => m.parse(ctx),

            Self::Group(m) => m.parse(ctx),
            Self::SkipGroup(m) => m.parse(ctx),
            Self::Sequence(m) => m.parse(ctx),
            Self::Choice(m) => m.parse(ctx),
            Self::Optional(m) => m.parse(ctx),
            Self::Closure(m) => m.parse(ctx),
            Self::PositiveClosure(m) => m.parse(ctx),
            Self::Join(m) => m.parse(ctx),
            Self::PositiveJoin(m) => m.parse(ctx),
            Self::Gather(m) => m.parse(ctx),
            Self::PositiveGather(m) => m.parse(ctx),

            Self::Lookahead(m) => m.parse(ctx),
            Self::NegativeLookahead(m) => m.parse(ctx),
            Self::SkipTo(m) => m.parse(ctx),
        }
    }
}
