// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Parsing state types: cursor position, memo cache, pattern cache, and call stack.

use super::memo::{KeyTrack, MemoCache};
use super::trace::{NULL_TRACER, Tracer};
use crate::cfg::HeartbeatRef;
use crate::context::SemanticsRef;
use crate::input::Cursor;
use crate::parser::TokenStack;
use crate::peg::error::DisasterReport;
use crate::types::Str;
use crate::util::pyre::Pattern;
use std::collections::HashMap;
use std::time::Instant;

/// Internal sentinel value used in parse trees.
pub const _AT_: &str = "__value__";

/// A cache of compiled regex patterns, keyed by their source string.
pub type PatternCache = HashMap<String, Pattern>;

/// The parser's call stack, implemented as a cons-list of rule names.
pub type CallStack = TokenStack;

/// Mutable parse state: cursor position and recursion tracking.
#[derive(Debug)]
pub struct ParseState<U: Cursor> {
    /// The input cursor.
    pub cursor: U,
    /// Tracks memo key recursion depth for left-recursion detection.
    pub keytrack: KeyTrack,
    pub last_cut_mark: usize,
    pub lookahead_depth: usize,
}

/// Shared heavyweight state used across context clones.
///
/// Holds memo tables, compiled patterns, keywords, the furthest failure,
/// tracer, heartbeat, call stack, and cut stack.
#[derive(Debug)]
pub struct HeavyState<'t> {
    /// Memoization cache for rule results.
    pub memos: MemoCache,
    /// Cache of compiled regex patterns.
    pub patterns: PatternCache,
    /// Sorted list of reserved keywords.
    pub keywords: Box<[Str]>,
    /// The furthest position at which a parse failure occurred.
    pub furthest_failure: Option<DisasterReport>,
    /// The active parse tracer.
    pub tracer: &'t dyn Tracer,
    /// Optional heartbeat callback for progress reporting.
    pub heartbeat: Option<HeartbeatRef>,
    /// Optional semantics actions for post-rule transformation.
    pub semantics: Option<SemanticsRef>,
    /// Total input length in bytes.
    pub input_len: usize,
    /// Timestamp of the last heartbeat tick.
    pub instant: Instant,
    /// Call stack tracking nested rule invocations.
    pub callstack: CallStack,
    /// Cut stack tracking which alternatives have seen a cut operator.
    pub cutstack: Vec<bool>,
}

impl<'t> Default for HeavyState<'t> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'t> HeavyState<'t> {
    /// Creates a new `HeavyState` with default values.
    pub fn new() -> Self {
        Self {
            memos: MemoCache::new(),
            patterns: PatternCache::new(),
            keywords: [].into(),
            furthest_failure: None,
            tracer: &NULL_TRACER,
            heartbeat: None,
            semantics: None,
            input_len: 0,
            instant: Instant::now(),
            callstack: CallStack::new(),
            cutstack: vec![false],
        }
    }

    /// Retrieves a compiled pattern from the cache, compiling it if necessary.
    pub fn get_pattern(&mut self, pattern: &str) -> Pattern {
        self.patterns
            .entry(pattern.to_string())
            .or_insert_with(|| Pattern::new(pattern).unwrap())
            .clone()
    }

    /// Records the furthest parse failure for error reporting.
    pub fn set_furthest_failure(&mut self, dis: DisasterReport) {
        self.furthest_failure = Some(dis);
    }
}

impl<U: Cursor> ParseState<U> {
    /// Creates a new `ParseState` with the given cursor.
    pub fn new(cursor: U) -> Self {
        Self {
            cursor,
            keytrack: KeyTrack::default(),
            last_cut_mark: 0,
            lookahead_depth: 0,
        }
    }
}
