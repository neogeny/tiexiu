// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::input::Cursor;
use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};

// #28
pub struct Lookahead {
    pub exp: Box<dyn CanParse>,
}

impl CanParse for Lookahead
{
    fn parse<'a>(&self, ctx: Ctx<'a>) -> ParseResult<'a> {
        let _ = self.exp.parse(ctx.clone())?;
        Ok((ctx, Cst::Nil))
    }
}


// #29
pub struct NegativeLookahead {
    pub exp: Box<dyn CanParse>,
}

impl CanParse for NegativeLookahead
{
    fn parse<'a>(&self, ctx: Ctx<'a>) -> ParseResult<'a> {
        if let Ok((_, _)) = self.exp.parse(ctx.clone()) {
            return Err(ctx)
        }
        Ok((ctx, Cst::Nil))
    }
}


// #30
pub struct SkipTo<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for SkipTo<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        loop {
            match self.exp.parse(ctx) {
                Err(err_ctx) => {
                    match err_ctx.next() {
                        Ok(next_cxt) => {
                            ctx = next_cxt;
                        },
                        Err(e) => return Err(e)
                    }
                }
                ok => return ok,
            }
        }
    }
}


// #32
pub struct Call<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for Call<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, _ctx: Ctx<C>) -> ParseResult<C> {
        unimplemented!()
    }
}

