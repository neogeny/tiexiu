// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::context::Ctx;
use crate::peg::error::ParseFailure::*;
use crate::peg::error::ParseResult;
use crate::trees::Tree;
use crate::types::Str;
use crate::{Exp, ExpKind};
use std::rc::Rc;

impl Exp {
    /// Returns the lookahead set as a boxed slice of strings.
    pub fn la_boxed(&self) -> Rc<[Str]> {
        self.la
            .as_ref()
            .map(|la| la.iter().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
            .into()
    }

    /// Parses an ordered choice — tries each option in sequence, committing on cut.
    pub fn parse_choice<C: Ctx>(&self, ctx: &mut C, options: &[Exp]) -> ParseResult {
        let start = ctx.mark();

        for option in options.iter() {
            if let ExpKind::Alt(exp) = &option.kind {
                ctx.push_cut();
                let result = exp.parse_at(ctx);
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
            } else {
                ctx.reset(start);
                return Err(ctx.failure(start, ChoiceOptionWithNoAlt));
            }
        }
        Err(ctx.failure(start, NoViableOption(self.lookahead_str())))
    }

    /// Parses an optional expression — succeeds with `Tree::Nil` if the inner expression fails.
    pub fn parse_optional<C: Ctx>(&self, ctx: &mut C, exp: &Exp) -> ParseResult {
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
