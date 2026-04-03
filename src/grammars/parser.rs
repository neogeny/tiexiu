// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::astree::Cst;
use crate::contexts::Ctx;
use std::fmt::Debug;

pub type ParseResult<C> = Result<S<C>, C>;

pub trait Parser<C: Ctx>: Debug {
    fn parse(&self, ctx: C) -> ParseResult<C>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct S<C: Ctx>(pub C, pub Cst);

impl<C: Ctx> S<C> {
    #[inline]
    pub fn ctx(self) -> C {
        self.0
    }

    #[inline]
    pub fn cst(self) -> Cst {
        self.1
    }

    #[inline]
    pub fn ctx_ref(&self) -> &C {
        &self.0
    }

    #[inline]
    pub fn cst_ref(&self) -> &Cst {
        &self.1
    }
}
