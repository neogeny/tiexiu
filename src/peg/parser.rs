// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::error::ParseError;
use crate::state::Ctx;
use crate::trees::Tree;
pub use crate::util::tokenlist::TokenList;
use std::fmt::Debug;
use std::panic::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Succ<C: Ctx>(pub C, pub Tree);

#[derive(Debug, Clone)]
pub struct Fail {
    pub start: usize,
    pub mark: usize, // The position where the disaster occurred
    pub cutseen: bool,
    pub callstack: TokenList,
    pub source: Box<ParseError>,
    pub location: &'static Location<'static>,
}

pub type ParseResult<C> = Result<Succ<C>, Fail>;

pub trait Parser<C: Ctx>: Debug {
    fn parse(&self, ctx: C) -> ParseResult<C>;
}

impl std::fmt::Display for Fail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Use the three-liner style you prefer
        write!(f, "{} at {}: {}", self.source, self.mark, self.callstack)
    }
}

impl std::error::Error for Fail {
    // source() is optional since ParseError is the cause,
    // but this is the "Rust Way" for chained errors.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

impl Fail {
    #[track_caller]
    pub fn new(start: usize, mark: usize, cut: bool, error: ParseError, stack: TokenList) -> Self {
        Self {
            start,
            mark,
            cutseen: cut,
            source: error.into(),
            callstack: stack,
            location: Location::caller(),
        }
    }

    pub fn setcut(&mut self) {
        self.cutseen = true;
    }

    pub fn uncut(&mut self) {
        self.cutseen = false;
    }
}

impl<C: Ctx> Succ<C> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::StrCtx;

    const TARGET: usize = 64;

    #[test]
    fn test_succ_size() {
        let size = size_of::<Succ<StrCtx>>();
        assert!(size <= TARGET, "Succ size is {} > {} bytes", size, TARGET);
    }

    #[test]
    fn test_fail_size() {
        let size = size_of::<Fail>();
        assert!(size <= TARGET, "Fail size is {} > {} bytes", size, TARGET);
    }
}
