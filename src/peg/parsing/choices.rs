// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::context::CtxSem;
use crate::peg::error::ParseFailure::*;
use crate::peg::error::ParseResult;
use crate::trees::Tree;
use crate::types::{Ref, Str};
use crate::{Exp, ExpKind};

impl Exp {
    /// Returns the lookahead set as a boxed slice of strings.
    pub fn la_boxed(&self) -> Ref<[Str]> {
        self.la
            .as_ref()
            .map(|la| la.iter().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
            .into()
    }

    /// Parses an ordered choice — tries each option in sequence, committing on cut.
    pub fn parse_choice<C: CtxSem>(&self, ctx: &mut C, options: &[Exp]) -> ParseResult {
        let start = ctx.mark();

        for option in options.iter() {
            let mut inner = option;
            // WARNING: TatSu >=v5.22 may optimize out Alt
            if let ExpKind::Alt(exp) = &option.kind {
                inner = exp
            }
            ctx.push_cut();
            let result = inner.parse_at(ctx);
            let cutseen = ctx.take_cut();
            match result {
                Ok(tree) => {
                    return Ok(tree);
                }
                Err(nope) => {
                    ctx.reset(start);
                    if cutseen {
                        return Err(nope);
                    }
                }
            }
        }
        Err(ctx.failure(start, NoViableOption(self.lookahead_str())))
    }

    /// Parses an optional expression — succeeds with `Tree::Nil` if the inner expression fails.
    pub fn parse_optional<C: CtxSem>(&self, ctx: &mut C, exp: &Exp) -> ParseResult {
        let start = ctx.mark();
        ctx.push_cut();
        let result = exp.parse_at(ctx);
        let cutseen = ctx.take_cut();
        match result {
            Ok(tree) => Ok(tree),
            Err(nope) => {
                ctx.reset(start);
                if cutseen {
                    return Err(nope);
                }
                Ok(Tree::Nil.into())
            }
        }
    }
}
