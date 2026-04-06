// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::state::Ctx;
use crate::trees::Tree;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct S<C: Ctx>(pub C, pub Tree);

#[derive(Debug, Clone, PartialEq)]
pub struct F {
    pub mark: usize,   // The position where the disaster occurred
    pub msg: Box<str>, // The "Why" (using your Boxed str for density)
    pub cut: bool,
}

pub type ParseResult<C> = Result<S<C>, F>;

pub trait Parser<C: Ctx>: Debug {
    fn parse(&self, ctx: C) -> ParseResult<C>;
}

impl F {
    pub fn new(mark: usize, msg: &str, cut: bool) -> Self {
        Self {
            mark,
            msg: msg.into(),
            cut,
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
