// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

// use std::sync::{Arc, OnceLock};
use crate::input::Cursor;
use super::model::{CanParse, ParseResult};
use crate::engine::{Ctx};


pub struct Call<M> {
    pub name: &'static str,
    pub rule: Option<Box<M>>
}

impl<M> Call<M> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            // rule: OnceLock::new(),
            rule: None
        }
    }
}

impl<M, C> CanParse<C> for Call<M>
where
    M: CanParse<C>,
    C: Cursor
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        // First-hit resolution:
        // 1. Check the cache.
        // 2. If empty, ask Ctx to find the Arc<dyn CanParse<C>> by name.
        // 3. Store the Arc in the cache for all future calls.
        // let rule = self.cache.get_or_init(|| {
        //     // ctx.resolve(self.name)
        //     ()
        // });
        if let Some(rule) = &self.rule {
            match rule.parse(ctx) {
                Ok((new_ctx, cst)) => {
                    Ok((new_ctx, cst.distill()))
                },
                err => err,
            }
        }
        else {
            Err(ctx)
        }
    }
}