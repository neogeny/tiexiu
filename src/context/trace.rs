// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::CtxI;
use console::style;
use std::fmt::Debug;
// use std::io::Write;

pub static NULL_TRACER: NullTracer = NullTracer {};
pub static CONSOLE_TRACER: ConsoleTracer = ConsoleTracer {};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NullTracer {}

#[derive(Debug, Default, Clone)]
pub struct ConsoleTracer {}

impl Tracer for NullTracer {}

impl Tracer for ConsoleTracer {
    fn trace(&self, ctx: &dyn CtxI, msg: &str) {
        let _ = ctx;
        let term = console::Term::stderr();
        term.write_line(msg).ok();
    }
}
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
    fn trace(&self, ctx: &dyn CtxI, msg: &str) {
        let _ = ctx;
        let _ = msg;
    }

    fn trace_event(&self, ctx: &dyn CtxI, event: Event, msg: &str) {
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
        let stack_symbol: String = match event {
            Event::Success => style("→").green(),
            Event::Failure => style("→").red(),
            Event::NoMatch => style("←").red(),
            Event::Match => style("←").green(),
            _ => style("←").yellow(),
        }
        .to_string();
        let lookahead = style(ctx.cursor().lookahead(ctx.mark()).replace(" ", "·"))
            .black()
            .bright();
        let mut callstack = String::new();
        // let term = Term::stderr();
        // let (_rows, cols) = term.size();
        for call in ctx.callstack().iter() {
            callstack.push_str(&style(call).white().bold().to_string());
            callstack.push_str(&stack_symbol);
            // if callstack.chars().count() > (cols - 5).into() {
            //     callstack.push_str(" ... ");
            //     break;
            // }
        }
        let location = ctx.cursor().location();
        let _source = location.source.to_string();
        let (line, col) = location.pos;

        let pos = style(format!("[{line}:{col}]→")).black().bright();

        let msg = format!("{event_symbol}{msg} {callstack} •\n{pos}{lookahead}");
        self.trace(ctx, msg.as_str());
    }

    fn trace_entry(&self, ctx: &dyn CtxI) {
        self.trace_event(ctx, Event::Entry, "");
    }

    fn trace_success(&self, ctx: &dyn CtxI) {
        self.trace_event(ctx, Event::Success, "");
    }

    fn trace_failure(&self, ctx: &dyn CtxI, error: &str) {
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
        let mut tag = name.to_string();
        if !tag.is_empty() {
            tag = format!("/{tag}/");
        }
        let msg = style(format!("'{token}'{tag}")).green().to_string();
        self.trace_event(ctx, Event::Match, &msg);
        true
    }

    fn trace_no_match(&self, ctx: &dyn CtxI, token: &str, name: &str) -> bool {
        let msg = if !token.is_empty() {
            style(format!(" '{token}'")).red().to_string()
        } else {
            style(format!(" /{name}/")).red().to_string()
        };
        self.trace_event(ctx, Event::NoMatch, &msg);
        false
    }
}
