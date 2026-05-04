// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::context::Ctx;
use crate::peg::error::ParseResult;
pub use crate::util::tokenstack::TokenStack;
use std::fmt::Debug;

pub trait Parser<C: Ctx>: Debug {
    fn parse(&self, ctx: C) -> ParseResult<C>;
}
