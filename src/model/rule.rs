// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::engine::Ctx;

#[derive(Clone)]
pub struct Rule<'r> {
    pub name: &'static str,
    pub exp: Box<&'r dyn CanParse>,
}

impl<'r> CanParse for Rule<'r>
{
    fn parse(&self, ctx: Ctx) -> ParseResult {
        match self.exp.parse(ctx) {
            Ok((new_ctx, cst)) => Ok((new_ctx, cst.distill())),
            err => err,
        }
    }
}


