// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::input::Cursor;
use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};

// #33
pub struct RuleInclude<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for RuleInclude<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        unimplemented!()
    }
}


// #34
pub struct BasedRule<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for BasedRule<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        unimplemented!()
    }
}

