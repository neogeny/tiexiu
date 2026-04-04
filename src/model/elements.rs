// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::parser::{ParseResult, Parser, S};
use super::repeat::{add_exp, repeat, repeat_with_pre};
use crate::model::F;
use crate::state::Ctx;
use crate::trees::Tree;
use std::fmt::Debug;
use std::ops::Deref;

pub type ERef = Box<Element>;
pub type ERefArr = Box<[Element]>;
pub type Str = Box<str>;

pub use super::build;

#[derive(Debug, Clone)]
pub enum Element {
    Cut,
    Void,
    Fail,
    Dot,
    Eof,

    Call(Str),

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
    Optional(ERef),
    Closure(ERef),
    PositiveClosure(ERef),

    Join { exp: ERef, sep: ERef },
    PositiveJoin { exp: ERef, sep: ERef },
    Gather { exp: ERef, sep: ERef },
    PositiveGather { exp: ERef, sep: ERef },
}

impl Element {
    pub fn parse<C: Ctx>(&self, ctx: C) -> ParseResult<C> {
        <Self as Parser<C>>::parse(self, ctx)
    }
}

impl<C> Parser<C> for Element
where
    C: Ctx,
{
    fn parse(&self, mut ctx: C) -> ParseResult<C> {
        match self {
            Self::Call(name) => match ctx.call(name) {
                Ok(S(mut ctx, tree)) => {
                    ctx.uncut();
                    Ok(S(ctx, tree))
                }
                Err(mut f) => {
                    f.uncut();
                    Err(f)
                }
            },
            Self::Cut => {
                ctx.cut();
                Ok(S(ctx, Tree::Nil))
            }
            Self::Void => Ok(S(ctx, Tree::Stump)),
            Self::Fail => Err(ctx.failure("Fail")),
            Self::Dot => {
                if ctx.next().is_some() {
                    Ok(S(ctx, Tree::Nil))
                } else {
                    Err(ctx.failure("No more input"))
                }
            }
            Self::Eof => {
                if ctx.eof_check() {
                    Ok(S(ctx, Tree::Nil))
                } else {
                    Err(ctx.failure("Expecting EOF/EOT"))
                }
            }

            Self::Token(token) => {
                if ctx.token(token) {
                    Ok(S(ctx, Tree::Leaf(token.deref().into())))
                } else {
                    Err(ctx.failure(&format!("Expecting '{}'", token.deref())))
                }
            }
            Self::Pattern(pattern) => {
                if let Some(matched) = ctx.pattern(pattern) {
                    Ok(S(ctx, Tree::Leaf(matched.into())))
                } else {
                    Err(ctx.failure(&format!("Expecting '{}'", pattern.deref())))
                }
            }
            Self::Constant(literal) => Ok(S(ctx, Tree::Leaf(literal.deref().into()))),
            Self::Alert(literal, _) => Ok(S(ctx, Tree::Leaf(literal.deref().into()))),

            Self::Named(name, exp) => match exp.parse(ctx) {
                Ok(S(ctx, tree)) => Ok(S(ctx, Tree::named(name, tree))),
                err => err,
            },
            Self::NamedList(name, exp) => match exp.parse(ctx) {
                Ok(S(ctx, tree)) => Ok(S(ctx, Tree::named_list(name, tree))),
                err => err,
            },
            Self::Override(exp) => match exp.parse(ctx) {
                Ok(S(ctx, tree)) => Ok(S(ctx, Tree::RootLeaf(Box::new(tree)))),
                err => err,
            },
            Self::OverrideList(exp) => match exp.parse(ctx) {
                Ok(S(ctx, tree)) => Ok(S(ctx, Tree::RootNode(Box::new(tree)))),
                err => err,
            },
            Self::Group(exp) => exp.parse(ctx),
            Self::SkipGroup(exp) => {
                let S(new_ctx, _) = exp.parse(ctx)?;
                Ok(S(new_ctx, Tree::Nil))
            }
            Self::Lookahead(exp) => {
                let _ = exp.parse(ctx.clone())?;
                Ok(S(ctx, Tree::Nil))
            }
            Self::NegativeLookahead(exp) => {
                if let Ok(S(_, _)) = exp.parse(ctx.clone()) {
                    Err(ctx.failure("Not expecting: ???"))
                } else {
                    Ok(S(ctx, Tree::Nil))
                }
            }
            Self::SkipTo(exp) => loop {
                match exp.parse(ctx.clone()) {
                    Err(f) => {
                        if !ctx.dot() {
                            return Err(f);
                        }
                    }
                    ok => break ok,
                }
            },

            Self::Sequence(sequence) => {
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
            Self::Choice(options) => {
                let mut furthest: Option<F> = None;

                for option in options.iter() {
                    match option.parse(ctx.clone()) {
                        Ok(S(mut new_ctx, tree)) => {
                            new_ctx.uncut();
                            return Ok(S(new_ctx, tree));
                        }
                        Err(mut f) => {
                            if f.cut_seen() {
                                f.uncut();
                                return Err(f);
                            }

                            match furthest {
                                Some(ref f) if f.mark() < f.mark() => {}
                                _ => furthest = Some(f),
                            }
                        }
                    }
                }
                Err(furthest.unwrap_or(ctx.failure("Expecting opions: X y z")))
            }

            Self::Optional(exp) => match exp.parse(ctx.clone()) {
                Ok(S(new_ctx, tree)) => Ok(S(new_ctx, tree)),
                Err(mut f) => {
                    // If the expression committed with a cut, we cannot be optional.
                    if f.cut_seen() {
                        f.uncut();
                        return Err(f);
                    }
                    // Otherwise, we forgive the failure and return the original ctx.
                    ctx.uncut();
                    Ok(S(ctx, Tree::Nil))
                }
            },

            Self::Closure(exp) => {
                let mut res = Vec::new();
                let new_ctx = repeat(exp.deref(), ctx, &mut res);
                Ok(S(new_ctx, Tree::from(res)))
            }
            Self::PositiveClosure(exp) => {
                let mut res: Vec<Tree> = Vec::new();
                match exp.parse(ctx) {
                    Ok(S(new_ctx, tree)) => {
                        ctx = new_ctx;
                        res.push(tree);
                    }
                    err => return err,
                };

                let new_ctx = repeat(exp.deref(), ctx, &mut res);
                Ok(S(new_ctx, Tree::from(res)))
            }
            Self::Join { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();

                match add_exp(exp.deref(), ctx, &mut res) {
                    Ok(new_ctx) => {
                        let ctx =
                            repeat_with_pre(exp.deref(), sep.deref(), new_ctx, &mut res, true);
                        Ok(S(ctx, Tree::from(res)))
                    }
                    Err(err_ctx) => Ok(S(err_ctx, Tree::from(res))),
                }
            }
            Self::PositiveJoin { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();

                match exp.parse(ctx) {
                    Ok(S(new_ctx, tree)) => {
                        res.push(tree);
                        ctx = new_ctx;
                    }
                    err => return err,
                };

                let new_ctx = repeat_with_pre(exp.deref(), sep.deref(), ctx, &mut res, true);
                Ok(S(new_ctx, Tree::from(res)))
            }
            Self::Gather { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();
                match add_exp(exp.deref(), ctx, &mut res) {
                    Ok(new_ctx) => {
                        let ctx =
                            repeat_with_pre(exp.deref(), sep.deref(), new_ctx, &mut res, false);
                        Ok(S(ctx, Tree::from(res)))
                    }
                    Err(err_ctx) => Ok(S(err_ctx, Tree::from(res))),
                }
            }
            Self::PositiveGather { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();

                match exp.parse(ctx) {
                    Ok(S(new_ctx, tree)) => {
                        ctx = new_ctx;
                        res.push(tree);
                    }
                    err => return err,
                };

                let new_ctx = repeat_with_pre(exp.deref(), sep.deref(), ctx, &mut res, false);
                Ok(S(new_ctx, Tree::from(res)))
            }
        }
    }
}
