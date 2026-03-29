// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::engine::{Cst, Ctx};

pub type ParseResult<'c> = Result<(Ctx<'c>, Cst), Ctx<'c>>;

pub trait CanParse {
    fn parse<'a>(&self, _ctx: Ctx<'a>) -> ParseResult<'a>;
}

// pub enum ModelImpl
// {
//     Cut(Cut),
//     Void(Void),
//     Fail(Fail),
//     Dot(Dot),
//     Eof(Eof),
//     Token(Token),
//     Constant(Constant),
//     Alert(Alert),
// 
//     Named(Named<ModelImpl>),
//     NamedList(NamedList<ModelImpl>),
//     Override(Override<ModelImpl>),
//     OverrideList(OverrideList<ModelImpl>),
// 
//     Group(Group<ModelImpl>),
//     SkipGroup(SkipGroup<ModelImpl>),
//     Sequence(Sequence<ModelImpl>),
//     Choice(Choice<ModelImpl>),
//     Optional(Optional<ModelImpl>),
//     Closure(Closure<ModelImpl>),
//     PositiveClosure(PositiveClosure<ModelImpl>),
//     Join(Join<ModelImpl>),
//     PositiveJoin(PositiveJoin<ModelImpl>),
//     Gather(Gather<ModelImpl>),
//     PositiveGather(PositiveGather<ModelImpl>),
// 
//     Lookahead(Lookahead<ModelImpl>),
//     NegativeLookahead(NegativeLookahead<ModelImpl>),
//     SkipTo(SkipTo<ModelImpl>)
// }
// 
// impl<C> CanParse< C> for ModelImpl
// where
//     C: Cursor
// {
//     fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
//         match self {
//             Self::Cut(m) => m.parse(ctx),
//             Self::Void(m) => m.parse(ctx),
//             Self::Fail(m) => m.parse(ctx),
//             Self::Dot(m) => m.parse(ctx),
//             Self::Eof(m) => m.parse(ctx),
//             Self::Token(m) => m.parse(ctx),
//             Self::Constant(m) => m.parse(ctx),
//             Self::Alert(m) => m.parse(ctx),
//             
//             Self::Named(m) => m.parse(ctx),
//             Self::NamedList(m) => m.parse(ctx),
//             Self::Override(m) => m.parse(ctx),
//             Self::OverrideList(m) => m.parse(ctx),
// 
//             Self::Group(m) => m.parse(ctx),
//             Self::SkipGroup(m) => m.parse(ctx),
//             Self::Sequence(m) => m.parse(ctx),
//             Self::Choice(m) => m.parse(ctx),
//             Self::Optional(m) => m.parse(ctx),
//             Self::Closure(m) => m.parse(ctx),
//             Self::PositiveClosure(m) => m.parse(ctx),
//             Self::Join(m) => m.parse(ctx),
//             Self::PositiveJoin(m) => m.parse(ctx),
//             Self::Gather(m) => m.parse(ctx),
//             Self::PositiveGather(m) => m.parse(ctx),
// 
//             Self::Lookahead(m) => m.parse(ctx),
//             Self::NegativeLookahead(m) => m.parse(ctx),
//             Self::SkipTo(m) => m.parse(ctx),
//         }
//     }
// }
