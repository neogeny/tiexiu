// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::input::Cursor;
use super::model::{CanParse, ParseResult};
use crate::engine::Ctx;


pub struct Choice<M> {
    pub options: Vec<Box<M>>,
}

impl<M, C> CanParse<C> for Choice<M>
where
    M: CanParse<C>,
    C: Cursor
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        let mut furthest_err = (Ctx::new(C::new()), String::new());

        for option in &self.options {
            match option.parse(ctx) {
                Ok(res) => return Ok(res),
                Err((mut err_ctx, msg)) => {
                    err_ctx.cut_seen = false;
                    if err_ctx.cut_seen {
                        return Err((err_ctx, msg));
                    }
                    if err_ctx.mark() >= furthest_err.0.mark() {
                        ctx = err_ctx.clone();
                        furthest_err = (err_ctx, msg);
                    }
                    else {
                        ctx = err_ctx;
                    }
                }
            }
        }
        Err(furthest_err)
    }
}
