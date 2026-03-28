// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::input::Cursor;
use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};

// #22
pub struct Named<M> {
    pub exp: Box<M>,
    pub name: &'static str
}

impl<M, C> CanParse<C> for Named<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        match self.exp.parse(ctx) {
            Ok((ctx, cst)) => {
                Ok((ctx, Cst::Named(self.name, Box::new(cst))))
            },
            err => err
        }
    }
}


// #23
pub struct NamedList<M> {
    pub exp: Box<M>,
    pub name: &'static str
}


impl<M, C> CanParse<C> for NamedList<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        match self.exp.parse(ctx) {
            Ok((ctx, cst)) => {
                Ok((ctx, Cst::NamedList(self.name, Box::new(cst))))
            },
            err => err
        }
    }
}


// #24
pub struct Override<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for Override<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut _ctx: Ctx<C>) -> ParseResult<C> {
        unimplemented!()
    }
}


// #25
pub struct OverrideList<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for OverrideList<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, __ctx: Ctx<C>) -> ParseResult<C> {
        unimplemented!()
    }
}

