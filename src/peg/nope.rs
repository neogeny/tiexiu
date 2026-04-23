// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::ParseError;
use crate::engine::CtxI;
use crate::engine::state::CallStack;
use crate::input::memento::Memento;
use std::fmt::Debug;
use std::panic::Location;

#[derive(Debug)]
pub struct DisasterReport {
    pub pos: (usize, usize),
    pub la: Box<str>,
    pub callstack: CallStack,
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

impl std::fmt::Display for Nope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.report.memento, f)
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
