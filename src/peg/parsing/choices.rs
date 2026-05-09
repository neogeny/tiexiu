// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Exp;
use crate::context::Ctx;
use crate::peg::error::ParseFailure::*;
use crate::peg::error::ParseResult;
use crate::peg::error::Yeap;
use crate::trees::Tree;
use crate::types::Str;
use std::rc::Rc;

impl Exp {
    pub fn la_boxed(&self) -> Rc<[Str]> {
        self.la
            .as_ref()
            .map(|la| la.iter().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
            .into()
    }

    pub fn parse_choice<C: Ctx>(&self, mut ctx: C, options: &[Exp]) -> ParseResult<C> {
        let start = ctx.mark();

        for option in options.iter() {
            match option.parse_at(ctx.push()) {
                Ok(Yeap(new_ctx, tree)) => {
                    return Ok(Yeap(ctx.merge(&new_ctx), tree));
                }
                Err(mut nope) => {
                    if nope.take_cut() {
                        return Err(nope);
                    }
                }
            }
        }
        Err(ctx.failure(start, NoViableOption(self.lookahead_str())))
    }

    pub fn parse_optional<C: Ctx>(&self, ctx: C, exp: &Exp) -> ParseResult<C> {
        match exp.parse_at(ctx.push()) {
            Ok(Yeap(new_ctx, tree)) => Ok(Yeap(ctx.merge(&new_ctx), tree)),
            Err(mut nope) => {
                if nope.take_cut() {
                    return Err(nope);
                }
                Ok(Yeap(ctx, Tree::Nil.into()))
            }
        }
    }
}
