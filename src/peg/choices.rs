// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::Exp;
use super::parser::{ParseResult, Succ};
use crate::engine::Ctx;
use crate::peg::ParseError;
use crate::trees::Tree;

impl Exp {
    pub fn parse_choice<C: Ctx>(&self, mut ctx: C, options: &[Exp]) -> ParseResult<C> {
        let start = ctx.mark();
        let mut furthest: Option<crate::peg::Nope> = None;

        for option in options.iter() {
            match option.parse(ctx.push()) {
                Ok(Succ(new_ctx, tree)) => {
                    return Ok(Succ(ctx.merge(new_ctx), tree));
                }
                Err(mut nope) => {
                    if nope.take_cut() {
                        ctx.undo();
                        return Err(nope);
                    }

                    if furthest
                        .as_ref()
                        .is_none_or(|prev| nope.start >= prev.start)
                    {
                        furthest = Some(nope);
                    }
                }
            }
        }
        Err(furthest.unwrap_or(ctx.failure(start, ParseError::NoViableOption(self.la.clone()))))
    }

    pub fn parse_optional<C: Ctx>(&self, mut ctx: C, exp: &Exp) -> ParseResult<C> {
        match exp.parse(ctx.push()) {
            Ok(Succ(new_ctx, tree)) => Ok(Succ(ctx.merge(new_ctx), tree)),
            Err(mut nope) => {
                if nope.take_cut() {
                    ctx.undo();
                    return Err(nope);
                }
                Ok(Succ(ctx, Tree::Nil))
            }
        }
    }
}
