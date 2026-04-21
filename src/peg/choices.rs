// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::Exp;
use super::parser::{ParseResult, Succ};
use crate::peg::ParseError;
use crate::state::Ctx;
use crate::trees::Tree;

impl Exp {
    pub fn parse_choice<C: Ctx>(&self, mut ctx: C, options: &[Exp]) -> ParseResult<C> {
        let start = ctx.mark();
        let was_cut = ctx.cut_seen();
        let mut furthest: Option<crate::peg::Nope> = None;

        for option in options.iter() {
            ctx.unset_cut();
            match option.parse(ctx.clone()) {
                Ok(Succ(mut new_ctx, tree)) => {
                    new_ctx.restore_if_was_cut(was_cut);
                    return Ok(Succ(new_ctx, tree));
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

    pub fn parse_optional<C: Ctx>(&self, mut ctx: C, exp: &Exp) -> ParseResult<C> {
        let was_cut = ctx.cut_seen();
        ctx.unset_cut();
        match exp.parse(ctx.clone()) {
            Ok(Succ(mut new_ctx, tree)) => {
                new_ctx.restore_if_was_cut(was_cut);
                Ok(Succ(new_ctx, tree))
            }
            Err(mut f) => {
                if f.take_cut() {
                    return Err(f);
                }
                ctx.restore_if_was_cut(was_cut);
                Ok(Succ(ctx, Tree::Nil))
            }
        }
    }
}
