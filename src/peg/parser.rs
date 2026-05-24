// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::context::Ctx;
use crate::peg::error::ParseResult;
pub(crate) use crate::util::tokenstack::TokenStack;
use std::fmt::Debug;

/// A trait for types that can parse input at a given context position.
pub trait Parser<C: Ctx>: Debug {
    /// Parse at the current context position, returning success or failure.
    fn parse_at(&self, ctx: &mut C) -> ParseResult;
}
