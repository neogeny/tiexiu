// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::Exp;
use super::parser::{ParseResult, Succ};
use crate::peg::ParseError;
use crate::state::Ctx;
use crate::trees::Tree;

impl Exp {
    pub fn parse_choice<C: Ctx>(&self, ctx: C, options: &[Exp]) -> ParseResult<C> {
        let start = ctx.mark();
        let mut furthest: Option<crate::peg::Nope> = None;

        for option in options.iter() {
            match option.parse(ctx.clone()) {
                Ok(Succ(new_ctx, tree)) => {
                    return Ok(Succ(ctx.merge(&new_ctx), tree));
                }
                Err(mut f) => {
                    if f.take_cut() {
                        return Err(f);
                    }

                    if furthest.as_ref().is_none_or(|prev| f.start >= prev.start) {
                        furthest = Some(f);
                    }
                }
            }
        }
        Err(furthest.unwrap_or(ctx.failure(start, ParseError::NoViableOption(self.la.clone()))))
    }

    pub fn parse_optional<C: Ctx>(&self, ctx: C, exp: &Exp) -> ParseResult<C> {
        match exp.parse(ctx.clone()) {
            Ok(Succ(new_ctx, tree)) => Ok(Succ(ctx.merge(&new_ctx), tree)),
            Err(mut f) => {
                if f.take_cut() {
                    return Err(f);
                }
                Ok(Succ(ctx, Tree::Nil))
            }
        }
    }
}
