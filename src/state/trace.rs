// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::input::Cursor;
use crate::peg::F;

/// Explicit no-op implementation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NullTracer;

impl Tracer for NullTracer {}

pub trait Tracer: Default {
    fn trace(&self, msg: &str) {
        let _ = msg;
    }

    fn trace_event(&self, cursor: &dyn Cursor, event: &str) {
        let _ = cursor;
        let _ = event;
    }

    fn trace_entry(&self, cursor: &dyn Cursor) {
        let _ = cursor;
    }

    fn trace_success(&self, cursor: &dyn Cursor) {
        let _ = cursor;
    }

    fn trace_failure(&self, cursor: &dyn Cursor, f: Option<&F>) {
        let _ = cursor;
        let _ = f;
    }

    fn trace_recursion(&self, cursor: &dyn Cursor) {
        let _ = cursor;
    }

    fn trace_cut(&self, cursor: &dyn Cursor) {
        let _ = cursor;
    }

    fn trace_match(&self, cursor: &dyn Cursor, token: &str, name: Option<&str>, failed: bool) {
        let _ = cursor;
        let _ = token;
        let _ = name;
        let _ = failed;
    }

    fn rulestack(&self) -> String {
        String::new()
    }
}
