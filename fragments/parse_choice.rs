// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

impl Model {
    fn parse_choice<C: Cursor>(&self, ctx: Ctx<C>) -> ParseResult<C> {
        let mut last_error = (ctx.clone(), "Empty choice".to_string());

        for model in &self.alternatives {
            // We clone the state to backtrack the position if the branch fails
            match model.parse(ctx.clone()) {
                Ok(success) => return Ok(success),
                Err((mut err_ctx, msg)) => {
                    if err_ctx.cut_seen {
                        // CONSUME: We saw the cut, so we stop here.
                        // We reset the flag so it doesn't kill outer choices
                        // unless this Choice itself is wrapped in a failing sequence.
                        err_ctx.cut_seen = false;
                        return Err((err_ctx, msg));
                    }
                    // No cut: store the error and try the next alternative
                    last_error = (err_ctx, msg);
                }
            }
        }

        // If we reach here, no alternatives matched and no cut was seen.
        Err(last_error)
    }
}
