// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::parser::S;
use crate::astree::Cst;
use crate::context::Ctx;
use crate::model::E;

pub fn skip_exp<C: Ctx>(exp: &E, ctx: C) -> C {
    match exp.parse(ctx.clone()) {
        Ok(S(new_ctx, _)) => new_ctx,
        Err(_) => ctx,
    }
}

pub fn add_exp<C: Ctx>(exp: &E, ctx: C, res: &mut Vec<Cst>) -> Result<C, C> {
    match exp.parse(ctx.clone()) {
        Ok(S(new_ctx, cst)) => {
            res.push(cst);
            Ok(new_ctx)
        }
        Err(_) => Err(ctx),
    }
}

pub fn repeat<C: Ctx>(exp: &E, mut ctx: C, res: &mut Vec<Cst>) -> C {
    loop {
        match add_exp(exp, ctx.clone(), res) {
            Ok(new_ctx) => ctx = new_ctx,
            Err(ctx) => return ctx,
        }
    }
}

pub fn repeat_with_pre<C: Ctx>(
    exp: &E,
    pre: &E,
    mut ctx: C,
    res: &mut Vec<Cst>,
    keep_pre: bool,
) -> C {
    loop {
        match pre.parse(ctx.clone()) {
            Err(_) => return ctx,
            Ok(S(new_ctx, pre_cst)) => match exp.parse(new_ctx) {
                Err(_) => return ctx,
                Ok(S(repeat_ctx, exp_cst)) => {
                    if keep_pre {
                        res.push(pre_cst);
                    }
                    res.push(exp_cst);
                    ctx = repeat_ctx;
                }
            },
        }
    }
}
