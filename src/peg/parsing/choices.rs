// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Exp;
use crate::engine::Ctx;
use crate::peg::error::ParseFailure::*;
use crate::peg::error::ParseResult;
use crate::peg::error::{Nope, Yeap};
use crate::trees::Tree;
use crate::types::Ref;

impl Exp {
    pub fn la_boxed(&self) -> Box<[Ref<str>]> {
        self.la.as_ref().map(|la| la.as_ref()).unwrap_or(&[]).into()
    }

    pub fn parse_choice<C: Ctx>(&self, mut ctx: C, options: &[Exp]) -> ParseResult<C> {
        let start = ctx.mark();
        let mut furthest: Option<Nope> = None;

        for option in options.iter() {
            match option.parse(ctx.push()) {
                Ok(Yeap(new_ctx, tree)) => {
                    return Ok(Yeap(ctx.merge(new_ctx), tree));
                }
                Err(mut nope) => {
                    if nope.take_cut() {
                        return Err(nope);
                    }

                    if furthest.as_ref().is_none_or(|prev| nope.mark >= prev.mark) {
                        furthest = Some(nope);
                    }
                }
            }
        }
        // Err(furthest.unwrap_or(ctx.failure(start, NoViableOption(self.la_boxed()))))
        Err(ctx.failure(start, NoViableOption(self.la_boxed())))
    }

    pub fn parse_optional<C: Ctx>(&self, mut ctx: C, exp: &Exp) -> ParseResult<C> {
        match exp.parse(ctx.push()) {
            Ok(Yeap(new_ctx, tree)) => Ok(Yeap(ctx.merge(new_ctx), tree)),
            Err(mut nope) => {
                if nope.take_cut() {
                    return Err(nope);
                }
                Ok(Yeap(ctx, Tree::Nil))
            }
        }
    }
}
