// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub use super::build;
use super::parser::{F, ParseResult, Parser, S};
use crate::state::Ctx;
use crate::trees::Tree;
use std::fmt::Debug;
use std::ops::Deref;

pub type ERef = Box<Exp>;
pub type ERefArr = Box<[Exp]>;
pub type Str = Box<str>;

#[derive(Debug, Clone)]
pub struct Exp {
    pub(super) exp: ParserExp,
}

#[derive(Debug, Clone)]
pub enum ParserExp {
    Nil,
    Cut,
    Void,
    Fail,
    Dot,
    Eof,

    Call(Str, ERef),

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
    RuleInclude { name: Str, exp: ERef },
}

impl Exp {
    pub fn parse<C: Ctx>(&self, ctx: C) -> ParseResult<C> {
        <Self as Parser<C>>::parse(self, ctx)
    }
}

impl ParserExp {
    pub fn parse<C: Ctx>(&self, ctx: C) -> ParseResult<C> {
        <Self as Parser<C>>::parse(self, ctx)
    }
}

impl<C> Parser<C> for Exp
where
    C: Ctx,
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        self.exp.parse(ctx)
    }
}

impl<C> Parser<C> for ParserExp
where
    C: Ctx,
{
    fn parse(&self, mut ctx: C) -> ParseResult<C> {
        match self {
            Self::Nil => Ok(S(ctx, Tree::Nil)),
            Self::RuleInclude { .. } => Ok(S(ctx, Tree::Nil)),
            Self::Call(name, _exp) => match ctx.call(name) {
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
                    Err(ctx.failure(&format!("Expecting '{}'", token)))
                }
            }
            Self::Pattern(pattern) => {
                if let Some(matched) = ctx.pattern(pattern) {
                    Ok(S(ctx, Tree::Leaf(matched.into())))
                } else {
                    Err(ctx.failure(&format!("Expecting '{}'", pattern)))
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
            Self::Alt(exp) => exp.parse(ctx),
            Self::Choice(options) => {
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

                            match furthest {
                                Some(ref prev) if f.mark >= prev.mark => {}
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
                    if f.cut {
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
                let new_ctx = Self::repeat(ctx, exp, &mut res);
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

                let new_ctx = Self::repeat(ctx, exp, &mut res);
                Ok(S(new_ctx, Tree::from(res)))
            }
            Self::Join { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();

                match Self::add_exp(ctx, exp, &mut res) {
                    Ok(new_ctx) => {
                        let ctx = Self::repeat_with_pre(new_ctx, exp, sep, &mut res, true);
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

                let new_ctx = Self::repeat_with_pre(ctx, exp, sep, &mut res, true);
                Ok(S(new_ctx, Tree::from(res)))
            }
            Self::Gather { exp, sep } => {
                let mut res: Vec<Tree> = Vec::new();
                match Self::add_exp(ctx, exp, &mut res) {
                    Ok(new_ctx) => {
                        let ctx = Self::repeat_with_pre(new_ctx, exp, sep, &mut res, false);
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

                let new_ctx = Self::repeat_with_pre(ctx, exp, sep, &mut res, false);
                Ok(S(new_ctx, Tree::from(res)))
            }
        }
    }
}
