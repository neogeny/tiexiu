// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::CtxI;
use crate::peg::ParseError;
use console::style;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NullTracer {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ConsoleTracer {}

impl Tracer for NullTracer {}

impl Tracer for ConsoleTracer {
    fn trace(&self, msg: &str) {
        eprintln!("{}", msg);
    }
}

pub static NULL_TRACER: NullTracer = NullTracer {};
pub static CONSOLE_TRACER: ConsoleTracer = ConsoleTracer {};

pub enum Event {
    Entry,
    Success,
    Failure,
    Recursion,
    Cut,
    Match,
    NoMatch,
}

pub trait Tracer: Debug {
    fn trace(&self, msg: &str) {
        let _ = msg;
    }

    fn trace_event(&self, ctx: &dyn CtxI, event: Event, msg: &str) {
        console::set_colors_enabled(true);
        let event_symbol: String = match event {
            Event::Entry => style("↙").yellow(),
            Event::Success => style("≡").green(),
            Event::Failure => style("≢").red(),
            Event::Recursion => style("⟲").blue(),
            Event::Cut => style("⚔").yellow(),
            Event::Match => style("≡").green(),
            Event::NoMatch => style("≢").red(),
        }
        .to_string();
        let lookahead = ctx.cursor().lookahead(ctx.mark()).replace(" ", "·");
        let callstack = style(
            ctx.callstack()
                .to_vec()
                .join(style("←").green().to_string().as_str()),
        )
        .white()
        .bold();

        let location = ctx.cursor().location();
        let source = location.source.to_string();
        let (line, col) = location.pos;

        let pos = style(format!("[{line}:{col}]→")).black().bright();

        let msg = format!("{event_symbol}{msg} {callstack}{source}\n{pos}{lookahead}");
        self.trace(msg.as_str());
    }

    fn trace_entry(&self, ctx: &dyn CtxI) {
        self.trace_event(ctx, Event::Entry, "");
    }

    fn trace_success(&self, ctx: &dyn CtxI) {
        self.trace_event(ctx, Event::Success, "");
    }

    fn trace_failure(&self, ctx: &dyn CtxI, error: &ParseError) {
        let errstr = format!(" {}", style(error).red());
        self.trace_event(ctx, Event::Failure, &errstr);
    }

    fn trace_recursion(&self, ctx: &dyn CtxI) {
        self.trace_event(ctx, Event::Recursion, "");
    }

    fn trace_cut(&self, ctx: &dyn CtxI) {
        self.trace_event(ctx, Event::Cut, "");
    }

    fn trace_match(&self, ctx: &dyn CtxI, token: &str, name: &str) -> bool {
        let msg = style(format!("'{token}'/{name}/")).green().to_string();
        self.trace_event(ctx, Event::Match, &msg);
        true
    }

    fn trace_no_match(&self, ctx: &dyn CtxI, name: &str) -> bool {
        let msg = style(format!("'/{name}/")).red().to_string();
        self.trace_event(ctx, Event::NoMatch, &msg);
        false
    }
}
