// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::cst::Cst;
use super::ctx::Ctx;
use super::strctx::StrCtx;
use crate::grammars::Rule;
use crate::grammars::{ParseResult, S};

impl<'c> StrCtx<'c> {
    pub fn recursive_call(mut self, rule: &Rule) -> ParseResult<Self> {
        // The 'start_mark' is our anchor for the Memo Key
        let key = self.key(rule.name);
        let start_mark = self.mark();

        // 1. The Unified Memo Guard
        if let Some(memo) = self.memo(&key) {
            if let Cst::Bottom = memo.cst {
                // Recursive hit: return Err to force the base-case branch
                return Err(self);
            }

            // The Teleport: Jump the cursor to the recorded end and return
            self.reset(memo.mark);
            return Ok(S(self, memo.cst.clone()));
        }

        // 2. Fast Path: Non-LRec rules
        if !rule.is_lrec {
            return match rule.parse(self) {
                Ok(S(mut next_ctx, cst)) => {
                    // next_ctx.memoize uses its own current mark() as the end-point
                    next_ctx.memoize(&key, &cst);
                    Ok(S(next_ctx, cst))
                }
                Err(err_ctx) => Err(err_ctx),
            };
        }

        // 3. The Seed-Growing Loop (LRec Ratchet)
        // Plant the seed. memoize() will record the current (start) mark.
        self.memoize(&key, &Cst::Bottom);

        let mut best_cst: Option<Cst> = None;
        let mut high_water_mark = start_mark;

        loop {
            // Prospecting: attempt a fresh parse from the original start
            let mut trial_ctx = self.clone();
            trial_ctx.reset(start_mark);

            match rule.parse(trial_ctx) {
                Ok(S(mut next_ctx, new_cst)) => {
                    let end_mark = next_ctx.mark();

                    // Progress Check: Did we move the cursor further?
                    if end_mark > high_water_mark {
                        high_water_mark = end_mark;
                        best_cst = Some(new_cst.clone());

                        // Update the shared memo with the new 'End Mark'
                        // next_ctx.memoize() captures its own 'end_mark' internally.
                        self.memoize(&key, &new_cst);

                        // Advance our primary context to this successful state
                        self = next_ctx;
                    } else {
                        // Fixed point reached
                        break;
                    }
                }
                Err(_) => break, // No further growth possible
            }
        }

        // 4. Finalize
        if let Some(final_cst) = best_cst {
            Ok(S(self, final_cst))
        } else {
            Err(self)
        }
    }
}
