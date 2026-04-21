// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::error::ParseError;
use crate::input::memento::Memento;
use crate::state::{Ctx, CtxI};
use crate::trees::Tree;
pub use crate::util::tokenlist::TokenList;
use std::fmt::Debug;
use std::panic::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Succ<C: Ctx>(pub C, pub Tree);

#[derive(Debug)]
pub struct DisasterReport {
    pub pos: (usize, usize),
    pub la: Box<str>,
    pub callstack: TokenList,
    pub location: &'static Location<'static>,
    pub memento: Memento,
}

#[derive(Debug)]
pub struct Nope {
    pub start: usize,
    pub mark: usize, // The position where the disaster occurred
    pub cutseen: bool,
    pub source: Box<ParseError>,
    pub report: Box<DisasterReport>,
}

pub type ParseResult<C> = Result<Succ<C>, Nope>;

pub trait Parser<C: Ctx>: Debug {
    fn parse(&self, ctx: C) -> ParseResult<C>;
}

impl std::fmt::Display for Nope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)?;
        Ok(())
    }
}

impl std::error::Error for Nope {
    // source() is optional since ParseError is the cause,
    // but this is the "Rust Way" for chained errors.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

impl Nope {
    #[track_caller]
    pub fn new(start: usize, ctx: &dyn CtxI, error: ParseError) -> Self {
        let context = DisasterReport {
            pos: ctx.cursor().pos(),
            la: ctx.cursor().lookahead(start).into(),
            callstack: ctx.callstack(),
            location: Location::caller(),
            memento: Memento::new(
                ctx.cursor().textstr(),
                start,
                ctx.mark(),
                error.to_string().as_str(),
            ),
        };
        Self {
            start,
            mark: ctx.mark(),
            cutseen: ctx.cut_seen(),
            source: error.into(),
            report: context.into(),
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
        let size = size_of::<Nope>();
        assert!(size <= TARGET, "Fail size is {} > {} bytes", size, TARGET);
    }
}
