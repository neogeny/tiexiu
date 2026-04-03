// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::parser::{ParseResult, Parser, S};
use super::repeat::{add_exp, repeat, repeat_with_pre};
use crate::astree::Cst;
use crate::context::Ctx;
use crate::model::F;
use std::fmt::Debug;
use std::ops::Deref;

pub type ERef = Box<E>;
pub type ERefArr = Box<[E]>;
pub type Str = Box<str>;

pub use super::build;

#[derive(Debug, Clone)]
pub enum E {
    Cut,
    Void,
    Fail,
    Dot,
    Eof,

    Call(Str),

    Token(Str),
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

impl E {
    pub fn parse<C: Ctx>(&self, ctx: C) -> ParseResult<C> {
        <Self as Parser<C>>::parse(self, ctx)
    }
}

impl<C> Parser<C> for E
where
    C: Ctx,
{
    fn parse(&self, mut ctx: C) -> ParseResult<C> {
        match self {
            Self::Call(name) => match ctx.call(name) {
                Ok(S(mut ctx, cst)) => {
                    ctx.uncut();
                    Ok(S(ctx, cst))
                }
                Err(mut f) => {
                    f.uncut();
                    Err(f)
                }
            },
            Self::Cut => {
                ctx.cut();
                Ok(S(ctx, Cst::Nil))
            }
            Self::Void => Ok(S(ctx, Cst::Void)),
            Self::Fail => Err(ctx.failure("Fail")),
            Self::Dot => {
                if ctx.next().is_some() {
                    Ok(S(ctx, Cst::Nil))
                } else {
                    Err(ctx.failure("No more input"))
                }
            }
            Self::Eof => {
                if ctx.eof_check() {
                    Ok(S(ctx, Cst::Nil))
                } else {
                    Err(ctx.failure("Expecting EOF/EOT"))
                }
            }

            Self::Token(token) => {
                if ctx.token(token) {
                    Ok(S(ctx, Cst::Token(token.deref().into())))
                } else {
                    Err(ctx.failure(&format!("Expecting '{}'", token.deref())))
                }
            }
            Self::Constant(literal) => Ok(S(ctx, Cst::Literal(literal.deref().into()))),
            Self::Alert(literal, _) => Ok(S(ctx, Cst::Literal(literal.deref().into()))),

            Self::Named(name, exp) => match exp.parse(ctx) {
                Ok(S(ctx, cst)) => Ok(S(ctx, Cst::named(name, cst))),
                err => err,
            },
            Self::NamedList(name, exp) => match exp.parse(ctx) {
                Ok(S(ctx, cst)) => Ok(S(ctx, Cst::named_list(name, cst))),
                err => err,
            },
            Self::Override(exp) => match exp.parse(ctx) {
                Ok(S(ctx, cst)) => Ok(S(ctx, Cst::OverrideValue(Box::new(cst)))),
                err => err,
            },
            Self::OverrideList(exp) => match exp.parse(ctx) {
                Ok(S(ctx, cst)) => Ok(S(ctx, Cst::OverrideList(Box::new(cst)))),
                err => err,
            },
            Self::Group(exp) => exp.parse(ctx),
            Self::SkipGroup(exp) => {
                let S(new_ctx, _) = exp.parse(ctx)?;
                Ok(S(new_ctx, Cst::Nil))
            }
            Self::Lookahead(exp) => {
                let _ = exp.parse(ctx.clone())?;
                Ok(S(ctx, Cst::Nil))
            }
            Self::NegativeLookahead(exp) => {
                if let Ok(S(_, _)) = exp.parse(ctx.clone()) {
                    Err(ctx.failure("Not expecting: ???"))
                } else {
                    Ok(S(ctx, Cst::Nil))
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
                        Ok(S(new_ctx, cst)) => {
                            results.push(cst);
                            ctx = new_ctx;
                        }
                        err => return err,
                    }
                }
                Ok(S(ctx, Cst::from(results)))
            }
            Self::Choice(options) => {
                let mut furthest: Option<F> = None;

                for option in options.iter() {
                    match option.parse(ctx.clone()) {
                        Ok(S(mut new_ctx, cst)) => {
                            new_ctx.uncut();
                            return Ok(S(new_ctx, cst));
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
                Ok(S(new_ctx, cst)) => Ok(S(new_ctx, cst)),
                Err(mut f) => {
                    // If the expression committed with a cut, we cannot be optional.
                    if f.cut_seen() {
                        f.uncut();
                        return Err(f);
                    }
                    // Otherwise, we forgive the failure and return the original ctx.
                    ctx.uncut();
                    Ok(S(ctx, Cst::Nil))
                }
            },

            Self::Closure(exp) => {
                let mut res = Vec::new();
                let new_ctx = repeat(exp.deref(), ctx, &mut res);
                Ok(S(new_ctx, Cst::from(res)))
            }
            Self::PositiveClosure(exp) => {
                let mut res: Vec<Cst> = Vec::new();
                match exp.parse(ctx) {
                    Ok(S(new_ctx, cst)) => {
                        ctx = new_ctx;
                        res.push(cst);
                    }
                    err => return err,
                };

                let new_ctx = repeat(exp.deref(), ctx, &mut res);
                Ok(S(new_ctx, Cst::from(res)))
            }
            Self::Join { exp, sep } => {
                let mut res: Vec<Cst> = Vec::new();

                match add_exp(exp.deref(), ctx, &mut res) {
                    Ok(new_ctx) => {
                        let ctx =
                            repeat_with_pre(exp.deref(), sep.deref(), new_ctx, &mut res, true);
                        Ok(S(ctx, Cst::from(res)))
                    }
                    Err(err_ctx) => Ok(S(err_ctx, Cst::from(res))),
                }
            }
            Self::PositiveJoin { exp, sep } => {
                let mut res: Vec<Cst> = Vec::new();

                match exp.parse(ctx) {
                    Ok(S(new_ctx, cst)) => {
                        res.push(cst);
                        ctx = new_ctx;
                    }
                    err => return err,
                };

                let new_ctx = repeat_with_pre(exp.deref(), sep.deref(), ctx, &mut res, true);
                Ok(S(new_ctx, Cst::from(res)))
            }
            Self::Gather { exp, sep } => {
                let mut res: Vec<Cst> = Vec::new();
                match add_exp(exp.deref(), ctx, &mut res) {
                    Ok(new_ctx) => {
                        let ctx =
                            repeat_with_pre(exp.deref(), sep.deref(), new_ctx, &mut res, false);
                        Ok(S(ctx, Cst::from(res)))
                    }
                    Err(err_ctx) => Ok(S(err_ctx, Cst::from(res))),
                }
            }
            Self::PositiveGather { exp, sep } => {
                let mut res: Vec<Cst> = Vec::new();

                match exp.parse(ctx) {
                    Ok(S(new_ctx, cst)) => {
                        ctx = new_ctx;
                        res.push(cst);
                    }
                    err => return err,
                };

                let new_ctx = repeat_with_pre(exp.deref(), sep.deref(), ctx, &mut res, false);
                Ok(S(new_ctx, Cst::from(res)))
            }
        }
    }
}
