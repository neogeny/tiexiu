// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Exp;
use crate::context::Ctx;
use crate::peg::error::ParseFailure::*;
use crate::peg::error::ParseResult;
use crate::peg::error::Yeap;
use crate::trees::Tree;
use crate::types::Str;

impl Exp {
    pub fn la_boxed(&self) -> Box<[Str]> {
        self.la
            .as_ref()
            .map(|la| la.iter().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
            .into_boxed_slice()
    }

    pub fn parse_choice<C: Ctx>(&self, mut ctx: C, options: &[Exp]) -> ParseResult<C> {
        let start = ctx.mark();

        for option in options.iter() {
            match option.parse(ctx.push()) {
                Ok(Yeap(new_ctx, tree)) => {
                    return Ok(Yeap(ctx.merge(*new_ctx).into(), tree));
                }
                Err(mut nope) => {
                    if nope.take_cut() {
                        return Err(nope);
                    }
                }
            }
        }
        Err(ctx.failure(start, NoViableOption(self.la_boxed())))
    }

    pub fn parse_optional<C: Ctx>(&self, mut ctx: C, exp: &Exp) -> ParseResult<C> {
        match exp.parse(ctx.push()) {
            Ok(Yeap(new_ctx, tree)) => Ok(Yeap(ctx.merge(*new_ctx).into(), tree)),
            Err(mut nope) => {
                if nope.take_cut() {
                    return Err(nope);
                }
                Ok(Yeap(ctx.into(), Tree::Nil.into()))
            }
        }
    }
}
