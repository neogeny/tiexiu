// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub use super::build;
use super::error::ParseError;
pub use super::lookahead;
use super::parser::{Fail, ParseResult, Parser, Succ};
use super::rule::RuleRef;
use crate::state::Ctx;
use crate::trees::Tree;
use std::fmt::Debug;
use std::ops::Deref;

pub type ERef = Box<Exp>;
pub type ERefArr = Box<[Exp]>;
pub type Str = Box<str>;
#[derive(Debug, Clone)]
pub struct Exp {
    pub kind: ExpKind,
    pub lookahead: Box<[Str]>,
}

#[derive(Debug, Clone)]
pub enum ExpKind {
    Nil,
    Cut,
    Void,
    Fail,
    Dot,
    Eof,

    Call { name: Str, rule: Option<RuleRef> },

    Token(Str),
    Pattern(Str),
    Constant(Str),
    Alert(Str, u8),

    Named(Str, ERef),
    NamedList(Str, ERef),
    Override(ERef),
    OverrideList(ERef),

    Group(ERef),
    SkipGroup(ERef),

    Lookahead(ERef),
    NegativeLookahead(ERef),
    SkipTo(ERef),

    Sequence(ERefArr),
    Choice(ERefArr),
    Alt(ERef),
    Optional(ERef),
    Closure(ERef),
    PositiveClosure(ERef),

    Join { exp: ERef, sep: ERef },
    PositiveJoin { exp: ERef, sep: ERef },
    Gather { exp: ERef, sep: ERef },
    PositiveGather { exp: ERef, sep: ERef },
    RuleInclude { name: Str, exp: Option<ERef> },
}

impl Exp {
    pub fn parse<C: Ctx>(&self, ctx: C) -> ParseResult<C> {
        <Self as Parser<C>>::parse(self, ctx)
    }

    pub fn link<L: super::linker::Linker + ?Sized>(&mut self, linker: &mut L) {
        linker.link(self);
    }
}

impl<C> Parser<C> for Exp
where
    C: Ctx,
{
    fn parse(&self, mut ctx: C) -> ParseResult<C> {
        let start = ctx.mark();
        match &self.kind {
            ExpKind::Nil => Ok(Succ(ctx, Tree::Nil)),
            ExpKind::RuleInclude { name, exp } => match exp {
                None => Err(ctx.failure(start, ParseError::RuleNotLinked(name.clone()))),
                Some(exp) => exp.parse(ctx),
            },
            ExpKind::Call { name, rule } => match rule {
                None => Err(ctx.failure(start, ParseError::RuleNotLinked(name.clone()))),
                Some(rule) => match ctx.call_rule(name, rule.as_ref()) {
                    Ok(Succ(mut ctx, tree)) => {
                        ctx.uncut();
                        Ok(Succ(ctx, tree))
                    }
                    Err(mut f) => {
                        f.uncut();
                        Err(f)
                    }
                },
            },
            ExpKind::Cut => {
                // TODO: self.tracer.trace_cut(self.cursor)
                ctx.cut();
                Ok(Succ(ctx, Tree::Nil))
            }
            ExpKind::Void => Ok(Succ(ctx, Tree::Nil)),
            ExpKind::Fail => Err(ctx.failure(start, ParseError::Fail)),
            ExpKind::Dot => {
                if ctx.next().is_some() {
                    // TODO: self.tracer.trace_match(self.cursor, c)
                    Ok(Succ(ctx, Tree::Nil))
                } else {
                    // TODO: self.tracer.trace_match(self.cursor, c, failed=True)
                    Err(ctx.failure(start, ParseError::NoMoreInput))
                }
            }
            ExpKind::Eof => {
                if ctx.eof_check() {
                    Ok(Succ(ctx, Tree::Nil))
                } else {
                    Err(ctx.failure(start, ParseError::ExpectingEof))
                }
            }

            ExpKind::Token(token) => {
                if ctx.token(token) {
                    // TODO: self.tracer.trace_match(self.cursor, token)
                    Ok(Succ(ctx, Tree::Text(token.deref().into())))
                } else {
                    // TODO: self.tracer.trace_match(self.cursor, token, failed=True)
                    Err(ctx.failure(start, ParseError::ExpectedToken(token.deref().into())))
                }
            }
            ExpKind::Pattern(pattern) => {
                if let Some(matched) = ctx.pattern(pattern) {
                    // TODO: self.tracer.trace_match(self.cursor, token, pattern)
                    Ok(Succ(ctx, Tree::Text(matched.into())))
                } else {
                    // TODO: self.tracer.trace_match(self.cursor, '', pattern, failed=True)
                    Err(ctx.failure(start, ParseError::ExpectedPattern(pattern.deref().into())))
                }
            }
            ExpKind::Constant(literal) => Ok(Succ(ctx, Tree::Text(literal.deref().into()))),
            ExpKind::Alert(literal, _) => Ok(Succ(ctx, Tree::Text(literal.deref().into()))),

            ExpKind::Named(name, exp) => match exp.parse(ctx) {
                Ok(Succ(ctx, tree)) => Ok(Succ(ctx, Tree::named(name, tree))),
                err => err,
            },
            ExpKind::NamedList(name, exp) => match exp.parse(ctx) {
                Ok(Succ(ctx, tree)) => Ok(Succ(ctx, Tree::named_as_list(name, tree))),
                err => err,
            },
            ExpKind::Override(exp) => match exp.parse(ctx) {
                Ok(Succ(ctx, tree)) => Ok(Succ(ctx, Tree::override_with(tree))),
                err => err,
            },
            ExpKind::OverrideList(exp) => match exp.parse(ctx) {
                Ok(Succ(ctx, tree)) => Ok(Succ(ctx, Tree::override_as_list(tree))),
                err => err,
            },
            ExpKind::Group(exp) => exp.parse(ctx),
            ExpKind::SkipGroup(exp) => {
                let Succ(new_ctx, _) = exp.parse(ctx)?;
                Ok(Succ(new_ctx, Tree::Nil))
            }
            ExpKind::Lookahead(exp) => {
                let _ = exp.parse(ctx.clone())?;
                Ok(Succ(ctx, Tree::Nil))
            }
            ExpKind::NegativeLookahead(exp) => {
                if let Ok(Succ(_, _)) = exp.parse(ctx.clone()) {
                    Err(ctx.failure(start, ParseError::UnexpectedLookahead))
                } else {
                    Ok(Succ(ctx, Tree::Nil))
                }
            }
            ExpKind::SkipTo(exp) => loop {
                match exp.parse(ctx.clone()) {
                    Err(f) => {
                        if !ctx.dot() {
                            return Err(f);
                        }
                    }
                    ok => break ok,
                }
            },

            ExpKind::Sequence(sequence) => {
                let mut results = Vec::new();
                for exp in sequence.iter() {
                    match exp.parse(ctx) {
                        Ok(Succ(new_ctx, tree)) => {
                            results.push(tree);
                            ctx = new_ctx;
                        }
                        err => return err,
                    }
                }
                Ok(Succ(ctx, Tree::from(results)))
            }
            ExpKind::Alt(exp) => exp.parse(ctx),
            ExpKind::Choice(options) => {
                let mut furthest: Option<Fail> = None;

                for option in options.iter() {
                    match option.parse(ctx.clone()) {
                        Ok(Succ(mut new_ctx, tree)) => {
                            new_ctx.uncut();
                            return Ok(Succ(new_ctx, tree));
                        }
                        Err(mut f) => {
                            if f.cut {
                                f.uncut();
                                return Err(f);
                            }

                            if furthest.as_ref().is_none_or(|prev| f.mark > prev.mark) {
                                furthest = Some(f);
                            }
                        }
                    }
                }
                Err(furthest.unwrap_or(
                    ctx.failure(start, ParseError::NoViableOption(self.lookahead.clone())),
                ))
            }

            ExpKind::Optional(exp) => match exp.parse(ctx.clone()) {
                Ok(Succ(new_ctx, tree)) => Ok(Succ(new_ctx, tree)),
                Err(mut f) => {
                    // If the expression committed with a cut, we cannot be optional.
                    if f.cut {
                        f.uncut();
                        return Err(f);
                    }
                    // Otherwise, we forgive the failure and return the original ctx.
                    ctx.uncut();
                    Ok(Succ(ctx, Tree::Nil))
                }
            },

            ExpKind::Closure(exp) => {
                let mut res = Vec::new();
                match Self::repeat(ctx, exp, &mut res) {
                    Ok(Succ(new_ctx, _)) => Ok(Succ(new_ctx, Tree::from(res))),
                    err => err,
                }
            }
            ExpKind::PositiveClosure(exp) => {
                let mut res: Vec<Tree> = Vec::new();
                match exp.parse(ctx) {
                    Ok(Succ(new_ctx, tree)) => {
                        ctx = new_ctx;
                        res.push(tree);
                    }
                    err => return err,
                };

                match Self::repeat(ctx, exp, &mut res) {
                    Ok(Succ(new_ctx, _)) => Ok(Succ(new_ctx, Tree::from(res))),
                    err => err,
                }
            }
            ExpKind::Join { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();

                match Self::add_exp(ctx, exp, &mut res) {
                    Ok(new_ctx) => match Self::repeat_with_pre(new_ctx, exp, sep, &mut res, true) {
                        Ok(Succ(new_ctx, _)) => Ok(Succ(new_ctx, Tree::from(res))),
                        err => err,
                    },
                    Err((_actx, f)) => Err(f),
                }
            }
            ExpKind::PositiveJoin { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();

                match exp.parse(ctx) {
                    Ok(Succ(new_ctx, tree)) => {
                        res.push(tree);
                        ctx = new_ctx;
                    }
                    err => return err,
                };

                match Self::repeat_with_pre(ctx, exp, sep, &mut res, true) {
                    Ok(Succ(new_ctx, _)) => Ok(Succ(new_ctx, Tree::from(res))),
                    err => err,
                }
            }
            ExpKind::Gather { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();
                match Self::add_exp(ctx, exp, &mut res) {
                    Ok(new_ctx) => {
                        match Self::repeat_with_pre(new_ctx, exp, sep, &mut res, false) {
                            Ok(Succ(new_ctx, _)) => Ok(Succ(new_ctx, Tree::from(res))),
                            err => err,
                        }
                    }
                    Err((_actx, f)) => Err(f),
                }
            }
            ExpKind::PositiveGather { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();

                match exp.parse(ctx) {
                    Ok(Succ(new_ctx, tree)) => {
                        ctx = new_ctx;
                        res.push(tree);
                    }
                    err => return err,
                };

                match Self::repeat_with_pre(ctx, exp, sep, &mut res, false) {
                    Ok(Succ(new_ctx, _)) => Ok(Succ(new_ctx, Tree::from(res))),
                    err => err,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::StrCursor;
    use crate::peg::Rule;
    use crate::state::strctx::StrCtx;

    #[test]
    fn choice_keeps_furthest_failure() {
        let grammar = crate::peg::Grammar::new(
            "test",
            &[Rule::new(
                "start",
                &[],
                Exp::choice(vec![
                    Exp::sequence(vec![Exp::token("a"), Exp::token("b")]),
                    Exp::sequence(vec![Exp::token("a"), Exp::token("c"), Exp::token("d")]),
                ]),
            )],
        );
        let _ = grammar;
        let ctx = StrCtx::new(StrCursor::new("acx"));

        let err = grammar.parse(ctx).unwrap_err();

        assert_eq!(err.mark, 2);
        assert_eq!(err.error, ParseError::ExpectedToken("d".into()));
    }
}
