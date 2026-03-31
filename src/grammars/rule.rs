// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Model, ParseResult, Parser};
use crate::contexts::Ctx;

#[derive(Debug, Clone)]
pub struct Rule<'r> {
    name: &'r str,
    rhs: &'r Model,
}

impl<'r, C> Parser<C> for Rule<'r>
where
    C: Ctx,
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        self.rhs.parse(ctx)
    }
}
