// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::engine::Ctx;
use crate::peg::nope::Nope;
use crate::trees::Tree;
pub use crate::util::tokenstack::TokenStack;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Yeap<C: Ctx>(pub C, pub Tree);

pub type ParseResult<C> = Result<Yeap<C>, Nope>;

pub trait Parser<C: Ctx>: Debug {
    fn parse(&self, ctx: C) -> ParseResult<C>;
}

impl<C: Ctx> Yeap<C> {
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
    use crate::engine::StrCtx;

    const TARGET: usize = 64;

    #[test]
    fn test_succ_size() {
        let size = size_of::<Yeap<StrCtx>>();
        assert!(size <= TARGET, "Succ size is {} > {} bytes", size, TARGET);
    }

    #[test]
    fn test_fail_size() {
        let size = size_of::<Nope>();
        assert!(size <= TARGET, "Fail size is {} > {} bytes", size, TARGET);
    }
}
