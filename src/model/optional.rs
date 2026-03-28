// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::input::Cursor;
use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};


pub struct Optional<M> {
    pub exp: Box<M>,
}


impl<M, C> CanParse<C> for Optional<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        match self.exp.parse(ctx.clone()) {
            Ok(success) => Ok(success),
            Err(_) => Ok((ctx, Cst::Nil))
        }
    }
}
