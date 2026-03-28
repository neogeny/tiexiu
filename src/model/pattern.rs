// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::input::Cursor;
use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};

// #26
pub struct Pattern<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for Pattern<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        unimplemented!()
    }
}

