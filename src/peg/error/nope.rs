// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::failure::ParseFailure;
use crate::Tree;
use crate::cfg::types::{Ref, Str};
use crate::engine::{Ctx, CtxI};
use crate::input::memento::Memento;
use std::fmt::Debug;
use std::panic::Location;

pub type ParseResult<C> = Result<Yeap<C>, Nope>;

#[derive(Debug, Clone, PartialEq)]
pub struct Yeap<C: Ctx>(pub C, pub Tree);

#[derive(Clone, Debug)]
pub struct DisasterReport {
    pub start: usize,
    pub mark: usize, // The position where the disaster occurred
    pub pos: (usize, usize),
    pub la: Str,
    pub error: Ref<ParseFailure>,
    pub location: &'static Location<'static>,
    pub memento: Box<Memento>,
}

#[derive(thiserror::Error, Clone, Debug)]
pub struct Nope {
    pub cutseen: bool,
}

impl std::fmt::Display for Nope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl std::fmt::Display for DisasterReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.memento, f)
    }
}

impl std::error::Error for DisasterReport {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

impl DisasterReport {
    #[track_caller]
    pub fn new(start: usize, ctx: &dyn CtxI, error: &ParseFailure) -> Self {
        let memento = Self::new_memento(start, ctx, error);
        Self {
            start,
            mark: ctx.mark(),
            memento: memento.into(),
            pos: ctx.cursor().pos(),
            la: ctx.cursor().lookahead(start).into(),
            error: error.clone().into(),
            location: Location::caller(),
        }
    }

    pub fn new_memento(start: usize, ctx: &dyn CtxI, error: &ParseFailure) -> Memento {
        Memento {
            input_source: ctx.cursor().input_source().into(),
            start,
            mark: ctx.mark(),
            msg: error.to_string().into(),
            text: ctx.cursor().as_str().into(),
            callstack: ctx.callstack(),
        }
    }
}

impl Nope {
    #[track_caller]
    pub fn new(ctx: &dyn CtxI) -> Self {
        Self {
            cutseen: ctx.cut_seen(),
        }
    }

    pub fn setcut(&mut self) {
        self.cutseen = true;
    }

    pub fn take_cut(&mut self) -> bool {
        let was_cut = self.cutseen;
        self.cutseen = false;
        was_cut
    }

    pub fn restore_cut(&mut self, was_cut: bool) {
        if !was_cut {
            self.cutseen = false;
        }
    }
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
    use crate::engine::strctx::StrCtx;
    use crate::peg::error::nope::{Nope, Yeap};

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
