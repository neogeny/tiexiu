// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::engine::{Cst, Ctx};
use super::model::{CanParse, ParseResult};

pub struct Group<'g> {
    pub exp: Box<&'g dyn CanParse>,
}

impl<'g> Group<'g> {
    fn new(exp: &'g dyn CanParse) -> Self {
        Self {
            exp: Box::new(exp),
        }
    }
}

impl<'g> CanParse for Group<'g>
{
    fn parse<'a>(&self, ctx: Ctx<'a>) -> ParseResult<'a> {
        self.exp.parse(ctx)
    }
}


pub struct SkipGroup<'g> {
    pub exp: Box<&'g dyn CanParse>,
}

impl<'g> CanParse for SkipGroup<'g>
{
    fn parse<'a>(&self, ctx: Ctx<'a>) -> ParseResult<'a> {
        let (new_ctx, _) = self.exp.parse(ctx)?;
        Ok((new_ctx, Cst::Nil))
    }
}


