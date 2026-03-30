// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::canparse::{CanParse, ParseResult};
use super::repeat::{add_exp, repeat, repeat_with_pre};
use crate::contexts::{Cst, Ctx};
use std::fmt::Debug;
use std::ops::Deref;

pub type ModelRef = Box<Model>;
pub type ModelRefArr = Box<[ModelRef]>;
pub type Str = Box<str>;

#[derive(Debug, Clone)]
pub enum Model {
    Cut,
    Void,
    Fail,
    Dot,
    Eof,
    Token(Str),
    Constant(Str),
    Alert(Str, u8),

    Named(Str, ModelRef),
    NamedList(Str, ModelRef),
    Override(ModelRef),
    OverrideList(ModelRef),

    Group(ModelRef),
    SkipGroup(ModelRef),

    Lookahead(ModelRef),
    NegativeLookahead(ModelRef),
    SkipTo(ModelRef),

    Sequence(ModelRefArr),
    Choice(ModelRefArr),
    Optional(ModelRef),
    Closure(ModelRef),
    PositiveClosure(ModelRef),

    Join { exp: ModelRef, sep: ModelRef },
    PositiveJoin { exp: ModelRef, sep: ModelRef },
    Gather { exp: ModelRef, sep: ModelRef },
    PositiveGather { exp: ModelRef, sep: ModelRef },
}

impl<C> CanParse<C> for Model
where
    C: Ctx,
{
    fn parse(&self, mut ctx: C) -> ParseResult<C> {
        match self {
            Self::Cut => {
                ctx.cut();
                Ok((ctx, Cst::Nil))
            }
            Self::Void => Ok((ctx, Cst::Nil)),
            Self::Fail => Err(ctx),
            Self::Dot => {
                if ctx.next().is_some() {
                    Ok((ctx, Cst::Nil))
                } else {
                    Err(ctx)
                }
            }
            Self::Eof => {
                if ctx.eof_check() {
                    Ok((ctx, Cst::Nil))
                } else {
                    Err(ctx)
                }
            }

            Self::Token(token) => {
                if ctx.token(token) {
                    Ok((ctx, Cst::Token(token.deref().into())))
                } else {
                    Err(ctx)
                }
            }
            Self::Constant(literal) => Ok((ctx, Cst::Literal(literal.to_string()))),
            Self::Alert(literal, _) => Ok((ctx, Cst::Literal(literal.to_string()))),

            Self::Named(name, exp) => match exp.parse(ctx) {
                Ok((ctx, cst)) => Ok((ctx, Cst::Named(name.to_string(), Box::new(cst)))),
                err => err,
            },
            Self::NamedList(name, exp) => match exp.parse(ctx) {
                Ok((ctx, cst)) => Ok((ctx, Cst::NamedList(name.to_string(), Box::new(cst)))),
                err => err,
            },
            Self::Override(exp) => match exp.parse(ctx) {
                Ok((ctx, cst)) => Ok((ctx, Cst::OverrideValue(Box::new(cst)))),
                err => err,
            },
            Self::OverrideList(exp) => match exp.parse(ctx) {
                Ok((ctx, cst)) => Ok((ctx, Cst::OverrideList(Box::new(cst)))),
                err => err,
            },
            Self::Group(exp) => exp.parse(ctx),
            Self::SkipGroup(exp) => {
                let (new_ctx, _) = exp.parse(ctx)?;
                Ok((new_ctx, Cst::Nil))
            }
            Self::Lookahead(exp) => {
                let _ = exp.parse(ctx.clone())?;
                Ok((ctx, Cst::Nil))
            }
            Self::NegativeLookahead(exp) => {
                if let Ok((_, _)) = exp.parse(ctx.clone()) {
                    Err(ctx)
                } else {
                    Ok((ctx, Cst::Nil))
                }
            }
            Self::SkipTo(exp) => loop {
                match exp.parse(ctx) {
                    Err(errctx) => match errctx.next() {
                        None => break Err(errctx),
                        Some(_) => {
                            ctx = errctx;
                        }
                    },
                    ok => break ok,
                }
            },

            Self::Sequence(sequence) => {
                let mut results = Vec::new();
                for exp in sequence.iter() {
                    match exp.parse(ctx) {
                        Ok((new_ctx, cst)) => {
                            results.push(cst);
                            ctx = new_ctx;
                        }
                        err => return err,
                    }
                }
                Ok((ctx, Cst::from(results)))
            }
            Self::Choice(options) => {
                for option in options.iter() {
                    match option.parse(ctx) {
                        Ok(res) => return Ok(res),
                        Err(mut err_ctx) => {
                            if err_ctx.cut_seen() {
                                err_ctx.uncut();
                                return Err(err_ctx);
                            }
                            ctx = err_ctx;
                        }
                    }
                }
                Err(ctx)
            }
            Self::Optional(exp) => match exp.parse(ctx.clone()) {
                Ok(success) => Ok(success),
                Err(_) => Ok((ctx, Cst::Nil)),
            },

            Self::Closure(exp) => {
                let mut res = Vec::new();
                let new_ctx = repeat(exp.deref(), ctx, &mut res);
                Ok((new_ctx, Cst::from(res)))
            }
            Self::PositiveClosure(exp) => {
                let mut res: Vec<Cst> = Vec::new();
                match exp.parse(ctx) {
                    Ok((new_ctx, cst)) => {
                        ctx = new_ctx;
                        res.push(cst);
                    }
                    err => return err,
                };

                let new_ctx = repeat(exp.deref(), ctx, &mut res);
                Ok((new_ctx, Cst::from(res)))
            }
            Self::Join { exp, sep } => {
                let mut res: Vec<Cst> = Vec::new();

                match add_exp(exp.deref(), ctx, &mut res) {
                    Ok(new_ctx) => {
                        let ctx =
                            repeat_with_pre(exp.deref(), sep.deref(), new_ctx, &mut res, true);
                        Ok((ctx, Cst::from(res)))
                    }
                    Err(err_ctx) => Ok((err_ctx, Cst::from(res))),
                }
            }
            Self::PositiveJoin { exp, sep } => {
                let mut res: Vec<Cst> = Vec::new();

                match exp.parse(ctx) {
                    Ok((new_ctx, cst)) => {
                        res.push(cst);
                        ctx = new_ctx;
                    }
                    err => return err,
                };

                let new_ctx = repeat_with_pre(exp.deref(), sep.deref(), ctx, &mut res, true);
                Ok((new_ctx, Cst::from(res)))
            }
            Self::Gather { exp, sep } => {
                let mut res: Vec<Cst> = Vec::new();
                match add_exp(exp.deref(), ctx, &mut res) {
                    Ok(new_ctx) => {
                        let ctx =
                            repeat_with_pre(exp.deref(), sep.deref(), new_ctx, &mut res, false);
                        Ok((ctx, Cst::from(res)))
                    }
                    Err(err_ctx) => Ok((err_ctx, Cst::from(res))),
                }
            }
            Self::PositiveGather { exp, sep } => {
                let mut res: Vec<Cst> = Vec::new();

                match exp.parse(ctx) {
                    Ok((new_ctx, cst)) => {
                        ctx = new_ctx;
                        res.push(cst);
                    }
                    err => return err,
                };

                let new_ctx = repeat_with_pre(exp.deref(), sep.deref(), ctx, &mut res, false);
                Ok((new_ctx, Cst::from(res)))
            }
        }
    }
}
