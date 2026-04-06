// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::{Exp, ParserExp};
use super::parser::S;
use crate::state::Ctx;
use crate::trees::Tree;

impl ParserExp {
    pub fn skip_exp<C: Ctx>(ctx: C, exp: &Exp) -> C {
        match exp.parse(ctx.clone()) {
            Ok(S(new_ctx, _)) => new_ctx,
            Err(_) => ctx,
        }
    }

    pub fn add_exp<C: Ctx>(ctx: C, exp: &Exp, res: &mut Vec<Tree>) -> Result<C, C> {
        match exp.parse(ctx.clone()) {
            Ok(S(new_ctx, tree)) => {
                res.push(tree);
                Ok(new_ctx)
            }
            Err(_) => Err(ctx),
        }
    }

    pub fn repeat<C: Ctx>(mut ctx: C, exp: &Exp, res: &mut Vec<Tree>) -> C {
        loop {
            match Self::add_exp(ctx.clone(), exp, res) {
                Ok(new_ctx) => ctx = new_ctx,
                Err(ctx) => return ctx,
            }
        }
    }

    pub fn repeat_with_pre<C: Ctx>(
        mut ctx: C,
        exp: &Exp,
        pre: &Exp,
        res: &mut Vec<Tree>,
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
}
