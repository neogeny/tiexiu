// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Model, ParseResult, Parser};
use crate::contexts::Ctx;

#[derive(Debug, Clone)]
pub struct Rule<'r> {
    pub name: &'r str,
    pub is_memo: bool,
    pub is_lrec: bool,
    pub rhs: &'r Model,
}

impl<'r> Rule<'r> {
    pub fn new(name: &'r str, rhs: &'r Model) -> Self {
        Self {
            name,
            is_memo: true,
            is_lrec: false,
            rhs,
        }
    }

    pub fn parse<C: Ctx>(&self, ctx: C) -> ParseResult<C> {
        (self as &dyn Parser<C>).parse(ctx)
    }
}

impl<'r, C> Parser<C> for Rule<'r>
where
    C: Ctx,
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        self.rhs.parse(ctx)
    }
    
    fn is_left_recursive(&self) -> bool {
        self.is_lrec
    }
}
