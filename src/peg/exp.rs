// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub use super::build;
use super::error::ParseError;
pub use super::lookahead;
use super::parser::{F, ParseResult, Parser, S};
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
    RuleInclude { name: Str, rule: Option<RuleRef> },
}

impl Exp {
    pub fn parse<C: Ctx>(&self, ctx: C) -> ParseResult<C> {
        <Self as Parser<C>>::parse(self, ctx)
    }
}

impl<C> Parser<C> for Exp
where
    C: Ctx,
{
    fn parse(&self, mut ctx: C) -> ParseResult<C> {
        match &self.kind {
            ExpKind::Nil => Ok(S(ctx, Tree::Nil)),
            ExpKind::RuleInclude { name, rule } => match if let Some(rule) = rule {
                rule.exp.parse(ctx)
            } else {
                Err(ctx.failure(ParseError::RuleNotFound(name.clone())))
            } {
                Ok(S(mut ctx, tree)) => {
                    ctx.uncut();
                    Ok(S(ctx, tree))
                }
                Err(mut f) => {
                    f.uncut();
                    Err(f)
                }
            },
            ExpKind::Call { name, rule } => match if let Some(rule) = rule {
                ctx.call_rule(name, rule.as_ref())
            } else {
                Err(ctx.failure(ParseError::RuleNotFound(name.clone())))
            } {
                Ok(S(mut ctx, tree)) => {
                    ctx.uncut();
                    Ok(S(ctx, tree))
                }
                Err(mut f) => {
                    f.uncut();
                    Err(f)
                }
            },
            ExpKind::Cut => {
                ctx.cut();
                Ok(S(ctx, Tree::Nil))
            }
            ExpKind::Void => Ok(S(ctx, Tree::Void)),
            ExpKind::Fail => Err(ctx.failure(ParseError::Fail)),
            ExpKind::Dot => {
                if ctx.next().is_some() {
                    Ok(S(ctx, Tree::Nil))
                } else {
                    Err(ctx.failure(ParseError::NoMoreInput))
                }
            }
            ExpKind::Eof => {
                if ctx.eof_check() {
                    Ok(S(ctx, Tree::Nil))
                } else {
                    Err(ctx.failure(ParseError::ExpectingEof))
                }
            }

            ExpKind::Token(token) => {
                if ctx.token(token) {
                    Ok(S(ctx, Tree::Text(token.deref().into())))
                } else {
                    Err(ctx.failure(ParseError::ExpectedToken(token.deref().into())))
                }
            }
            ExpKind::Pattern(pattern) => {
                if let Some(matched) = ctx.pattern(pattern) {
                    Ok(S(ctx, Tree::Text(matched.into())))
                } else {
                    Err(ctx.failure(ParseError::ExpectedPattern(pattern.deref().into())))
                }
            }
            ExpKind::Constant(literal) => Ok(S(ctx, Tree::Text(literal.deref().into()))),
            ExpKind::Alert(literal, _) => Ok(S(ctx, Tree::Text(literal.deref().into()))),

            ExpKind::Named(name, exp) => match exp.parse(ctx) {
                Ok(S(ctx, tree)) => Ok(S(ctx, Tree::named(name, tree))),
                err => err,
            },
            ExpKind::NamedList(name, exp) => match exp.parse(ctx) {
                Ok(S(ctx, tree)) => Ok(S(ctx, Tree::named_as_list(name, tree))),
                err => err,
            },
            ExpKind::Override(exp) => match exp.parse(ctx) {
                Ok(S(ctx, tree)) => Ok(S(ctx, Tree::override_with(tree))),
                err => err,
            },
            ExpKind::OverrideList(exp) => match exp.parse(ctx) {
                Ok(S(ctx, tree)) => Ok(S(ctx, Tree::override_as_list(tree))),
                err => err,
            },
            ExpKind::Group(exp) => exp.parse(ctx),
            ExpKind::SkipGroup(exp) => {
                let S(new_ctx, _) = exp.parse(ctx)?;
                Ok(S(new_ctx, Tree::Nil))
            }
            ExpKind::Lookahead(exp) => {
                let _ = exp.parse(ctx.clone())?;
                Ok(S(ctx, Tree::Nil))
            }
            ExpKind::NegativeLookahead(exp) => {
                if let Ok(S(_, _)) = exp.parse(ctx.clone()) {
                    Err(ctx.failure(ParseError::UnexpectedLookahead))
                } else {
                    Ok(S(ctx, Tree::Nil))
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
                        Ok(S(new_ctx, tree)) => {
                            results.push(tree);
                            ctx = new_ctx;
                        }
                        err => return err,
                    }
                }
                Ok(S(ctx, Tree::from(results)))
            }
            ExpKind::Alt(exp) => exp.parse(ctx),
            ExpKind::Choice(options) => {
                let mut furthest: Option<F> = None;

                for option in options.iter() {
                    match option.parse(ctx.clone()) {
                        Ok(S(mut new_ctx, tree)) => {
                            new_ctx.uncut();
                            return Ok(S(new_ctx, tree));
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
                Err(furthest
                    .unwrap_or(ctx.failure(ParseError::NoViableOption(self.lookahead.clone()))))
            }

            ExpKind::Optional(exp) => match exp.parse(ctx.clone()) {
                Ok(S(new_ctx, tree)) => Ok(S(new_ctx, tree)),
                Err(mut f) => {
                    // If the expression committed with a cut, we cannot be optional.
                    if f.cut {
                        f.uncut();
                        return Err(f);
                    }
                    // Otherwise, we forgive the failure and return the original ctx.
                    ctx.uncut();
                    Ok(S(ctx, Tree::Nil))
                }
            },

            ExpKind::Closure(exp) => {
                let mut res = Vec::new();
                let new_ctx = Self::repeat(ctx, exp, &mut res);
                Ok(S(new_ctx, Tree::from(res)))
            }
            ExpKind::PositiveClosure(exp) => {
                let mut res: Vec<Tree> = Vec::new();
                match exp.parse(ctx) {
                    Ok(S(new_ctx, tree)) => {
                        ctx = new_ctx;
                        res.push(tree);
                    }
                    err => return err,
                };

                let new_ctx = Self::repeat(ctx, exp, &mut res);
                Ok(S(new_ctx, Tree::from(res)))
            }
            ExpKind::Join { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();

                match Self::add_exp(ctx, exp, &mut res) {
                    Ok(new_ctx) => {
                        let ctx = Self::repeat_with_pre(new_ctx, exp, sep, &mut res, true);
                        Ok(S(ctx, Tree::from(res)))
                    }
                    Err(err_ctx) => Ok(S(err_ctx, Tree::from(res))),
                }
            }
            ExpKind::PositiveJoin { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();

                match exp.parse(ctx) {
                    Ok(S(new_ctx, tree)) => {
                        res.push(tree);
                        ctx = new_ctx;
                    }
                    err => return err,
                };

                let new_ctx = Self::repeat_with_pre(ctx, exp, sep, &mut res, true);
                Ok(S(new_ctx, Tree::from(res)))
            }
            ExpKind::Gather { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();
                match Self::add_exp(ctx, exp, &mut res) {
                    Ok(new_ctx) => {
                        let ctx = Self::repeat_with_pre(new_ctx, exp, sep, &mut res, false);
                        Ok(S(ctx, Tree::from(res)))
                    }
                    Err(err_ctx) => Ok(S(err_ctx, Tree::from(res))),
                }
            }
            ExpKind::PositiveGather { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();

                match exp.parse(ctx) {
                    Ok(S(new_ctx, tree)) => {
                        ctx = new_ctx;
                        res.push(tree);
                    }
                    err => return err,
                };

                let new_ctx = Self::repeat_with_pre(ctx, exp, sep, &mut res, false);
                Ok(S(new_ctx, Tree::from(res)))
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
