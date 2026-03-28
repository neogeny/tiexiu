// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::input::Cursor;
use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};

// #6
pub struct Dot {
}

impl<C> CanParse<C> for Dot
where
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        ctx.dot()
    }
}


// #?
pub struct Void {
}

impl<C> CanParse<C> for Void
where
    C: Cursor,
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        Ok((ctx, Cst::Nil))
    }
}

// #7
pub struct Fail {
}

impl<C> CanParse<C> for Fail
where
    C: Cursor,
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        Err(ctx)
    }
}


// #8
pub struct Eof {
}

impl<C> CanParse<C> for Eof
where
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        ctx.eof_check()
    }
}


// #9
pub struct Token {
    pub token: &'static str,
}

impl<C> CanParse<C> for Token
where
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        ctx.token(self.token)
    }
}


// #10
pub struct Constant {
    pub literal: &'static str,
}

impl<C> CanParse<C> for Constant
where
    C: Cursor,
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        Ok((ctx, Cst::Literal(self.literal)))
    }
}


// #11
pub struct Alert {
    pub literal: &'static str,
    pub level: u8
}

impl<C> CanParse<C> for Alert
where
    C: Cursor,
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        Ok((ctx, Cst::Literal(self.literal)))
    }
}


// #12
pub struct Cut {}

impl<C> CanParse<C> for Cut
where
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        ctx.cut();
        Ok((ctx, Cst::Nil))
    }
}
