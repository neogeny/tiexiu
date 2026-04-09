// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::error::ParseError;
use crate::state::Ctx;
use crate::trees::Tree;
pub use crate::util::tokenlist::TokenList;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct S<C: Ctx>(pub C, pub Tree);

#[derive(Debug, Clone)]
pub struct F {
    pub start: usize,
    pub mark: usize, // The position where the disaster occurred
    pub cut: bool,
    pub error: ParseError,
    pub callstack: TokenList,
}

pub type ParseResult<C> = Result<S<C>, F>;

pub trait Parser<C: Ctx>: Debug {
    fn parse(&self, ctx: C) -> ParseResult<C>;
}

impl std::fmt::Display for F {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Use the three-liner style you prefer
        write!(f, "{} at {}: {}", self.error, self.mark, self.callstack)
    }
}

impl std::error::Error for F {
    // source() is optional since ParseError is the cause,
    // but this is the "Rust Way" for chained errors.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

impl F {
    pub fn new(start: usize, mark: usize, cut: bool, error: ParseError, stack: TokenList) -> Self {
        Self {
            start,
            mark,
            cut,
            error,
            callstack: stack,
        }
    }

    pub fn setcut(&mut self) {
        self.cut = true;
    }

    pub fn uncut(&mut self) {
        self.cut = false;
    }
}

impl<C: Ctx> S<C> {
    #[inline]
    pub fn ctx(self) -> C {
        self.0
    }

    #[inline]
    pub fn tree(self) -> Tree {
        self.1
    }

    #[inline]
    pub fn ctx_ref(&self) -> &C {
        &self.0
    }

    #[inline]
    pub fn cst_ref(&self) -> &Tree {
        &self.1
    }
}
