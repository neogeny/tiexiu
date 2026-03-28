// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::input::Cursor;
use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};

// #1
pub struct NULL<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for NULL<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        unimplemented!()
    }
}


// #2
pub struct Void<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for Void<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        unimplemented!()
    }
}


// #3
pub struct Synth<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for Synth<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        unimplemented!()
    }
}


// #4
pub struct Rule<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for Rule<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        unimplemented!()
    }
}


// #5
pub struct Grammar<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for Grammar<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        unimplemented!()
    }
}

