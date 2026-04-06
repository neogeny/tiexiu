// peg/rule.rs
// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub struct Rule<M: Model> {
    pub name: Box<str>,
    pub body: Box<M>,
}

impl<M: Model> Model for Rule<M> {
    fn parse<C: Cursor>(&self, ctx: Ctx<C>) -> ParseResult<C> {
        // 1. Execute the body
        match self.body.parse(ctx) {
            Ok((mut next_ctx, raw_cst)) => {
                // SUCCESS: Clear cut_seen so it doesn't affect the caller's choices.
                // The rule has 'committed' and succeeded; its internal cuts are over.
                next_ctx.cut_seen = false;

                // 2. The Lazy Boundary: Distill the "wet" CST into the final form.
                let final_cst = raw_cst.distill();

                Ok((next_ctx, final_cst))
            }
            Err((mut err_ctx, msg)) => {
                // FAILURE: We also clear cut_seen here before bubbling up.
                // This ensures the cut only affected this Rule's internal choices.
                // If the parent wants to cut, it must have its own Cut peg.
                err_ctx.cut_seen = false;

                Err((err_ctx, msg))
            }
        }
    }
}
