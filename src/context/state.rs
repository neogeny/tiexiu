// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Parsing state types: cursor position, memo cache, pattern cache, and call stack.

use super::memo::{KeyTrack, MemoCache};
use super::trace::{NULL_TRACER, Tracer};
use crate::cfg::HeartbeatRef;
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

/// A parser alert with severity level and message.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Alert {
    /// Alert severity level.
    level: usize,
    /// Alert message text.
    message: Str,
}

/// Mutable parse state: cursor position and recursion tracking.
#[derive(Debug)]
pub struct ParseState<U: Cursor + Clone> {
    /// The input cursor.
    pub cursor: U,
    /// Tracks memo key recursion depth for left-recursion detection.
    pub keytrack: KeyTrack,
}

/// Shared heavyweight state used across context clones.
///
/// Holds memo tables, compiled patterns, keywords, the furthest failure,
/// tracer, heartbeat, call stack, and cut stack.
#[derive(Debug, Clone)]
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
    /// Total input length in bytes.
    pub input_len: usize,
    /// Timestamp of the last heartbeat tick.
    pub instant: Instant,
    /// Call stack tracking nested rule invocations.
    pub callstack: CallStack,
    /// Cut stack tracking which alternatives have seen a cut operator.
    pub cutstack: Vec<bool>,
}

/// A stack-based container for `ParseState`, supporting push/undo/merge.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ParseStateStack<U: Cursor + Clone> {
    state_stack: Vec<ParseState<U>>,
}

impl<U: Cursor + Clone> Clone for ParseState<U> {
    fn clone(&self) -> Self {
        Self::new(self.cursor.clone())
    }
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

impl<U: Cursor + Clone> ParseState<U> {
    /// Creates a new `ParseState` with the given cursor.
    pub fn new(cursor: U) -> Self {
        Self {
            cursor,
            keytrack: KeyTrack::default(),
        }
    }

    /// Creates a `ParseState` by cloning the cursor from another state.
    pub fn from_state(other: &Self) -> Self {
        Self {
            cursor: other.cursor.clone(),
            keytrack: KeyTrack::default(),
        }
    }

    /// Resets the cursor to a previous position.
    #[allow(dead_code)]
    pub fn merge(&mut self, prev: &Self) -> &mut Self {
        self.cursor.reset(prev.cursor.mark());
        self
    }

    /// Placeholder for state pop logic.
    #[allow(dead_code)]
    pub fn pop(&mut self, _into: &mut Self) {}

    /// Placeholder for state undo logic.
    #[allow(dead_code)]
    pub fn undo(&mut self, _into: &mut Self) {}
}

#[allow(dead_code)]
impl<U: Cursor + Clone> ParseStateStack<U> {
    /// Creates a new stack with an initial parse state at the given cursor.
    pub fn new(cursor: U) -> Self {
        Self {
            state_stack: vec![ParseState::new(cursor)],
        }
    }

    /// Returns a reference to the current (top) parse state.
    #[track_caller]
    pub fn state(&self) -> &ParseState<U> {
        self.state_stack.last().expect("empty state stack")
    }

    /// Returns a mutable reference to the current (top) parse state.
    #[track_caller]
    pub fn state_mut(&mut self) -> &mut ParseState<U> {
        self.state_stack.last_mut().expect("empty state stack")
    }

    /// Pops the top state and applies undo to the new top.
    #[track_caller]
    pub fn undo(&mut self) -> ParseState<U> {
        let mut prev = self.state_stack.pop().expect("empty state stack");
        prev.undo(self.state_mut());
        prev
    }

    /// Pops the top state and applies pop to the new top.
    #[track_caller]
    pub fn pop(&mut self) -> ParseState<U> {
        let mut prev = self.state_stack.pop().expect("empty state stack");
        prev.pop(self.state_mut());
        prev
    }

    /// Pushes a new parse state initialized with a fresh cursor.
    pub fn new_state(&mut self) -> &mut ParseState<U> {
        let new_s = ParseState::new(self.state().cursor.clone());
        self.state_stack.push(new_s);
        self.state_mut()
    }

    /// Pushes a new parse state cloned from the current state.
    pub fn push(&mut self) -> &mut ParseState<U> {
        let new_s = ParseState::from_state(self.state());
        self.state_stack.push(new_s);
        self.state_mut()
    }

    /// Pops the top state and merges it into the new top.
    pub fn merge(&mut self) -> &mut ParseState<U> {
        let prev = self.pop();
        self.state_mut().merge(&prev)
    }
}
