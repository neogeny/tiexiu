// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::input::Cursor;
use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};

// #28
pub struct Lookahead<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for Lookahead<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        let _ = self.exp.parse(ctx.clone())?;
        Ok((ctx, Cst::Nil))
    }
}


// #29
pub struct NegativeLookahead<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for NegativeLookahead<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
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
    fn parse(&self, _ctx: Ctx<C>) -> ParseResult<C> {
        unimplemented!()
        // while let Err(err_ctx) = self.exp.parse(ctx) {
        //     match err_ctx.next() {
        //         None => Err(),
        //         Some(next_cxt) => {
        //             ctx = next_cxt;
        //             continue;
        //         }
        //     }
        //
        // }
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

