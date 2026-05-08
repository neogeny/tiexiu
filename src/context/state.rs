// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A translation of the TatSu module with the same name

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

pub const _AT_: &str = "__value__";

pub type PatternCache = HashMap<String, Pattern>;

pub type CallStack = TokenStack;

#[derive(Debug, Clone)]
pub struct Alert {
    pub level: usize,
    pub message: Str,
}

#[derive(Debug)]
pub struct ParseState<U: Cursor + Clone> {
    pub cursor: U,
    pub cutseen: bool,
    pub keytrack: KeyTrack,
}

#[derive(Debug, Clone)]
pub struct HeavyState<'t> {
    pub memos: MemoCache,
    pub patterns: PatternCache,
    pub keywords: Box<[Str]>,
    pub furthest_failure: Option<DisasterReport>,
    pub tracer: &'t dyn Tracer,
    pub heartbeat: Option<HeartbeatRef>,
    pub input_len: usize,
    pub instant: Instant,
    pub callstack: CallStack,
}

#[derive(Debug, Clone)]
pub struct ParseStateStack<U: Cursor + Clone> {
    pub state_stack: Vec<ParseState<U>>,
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
        }
    }

    pub fn get_pattern(&mut self, pattern: &str) -> Pattern {
        self.patterns
            .entry(pattern.to_string())
            .or_insert_with(|| Pattern::new(pattern).unwrap())
            .clone()
    }

    pub fn set_furthest_failure(&mut self, dis: &DisasterReport) {
        self.furthest_failure = Some(dis.clone());
    }
}

impl<U: Cursor + Clone> ParseState<U> {
    pub fn new(cursor: U) -> Self {
        Self {
            cursor,
            cutseen: false,
            keytrack: KeyTrack::default(),
        }
    }

    pub fn from_state(other: &Self) -> Self {
        Self {
            cursor: other.cursor.clone(),
            cutseen: false,
            keytrack: KeyTrack::default(),
        }
    }

    pub fn merge(&mut self, prev: &Self) -> &mut Self {
        self.cursor.reset(prev.cursor.mark());
        // self.callstack = prev.callstack.clone();
        self
    }

    pub fn pop(&mut self, _into: &mut Self) {}

    pub fn undo(&mut self, _into: &mut Self) {}
}

impl<U: Cursor + Clone> ParseStateStack<U> {
    pub fn new(cursor: U) -> Self {
        Self {
            state_stack: vec![ParseState::new(cursor)],
        }
    }

    #[track_caller]
    pub fn state(&self) -> &ParseState<U> {
        self.state_stack.last().expect("empty state stack")
    }

    #[track_caller]
    pub fn state_mut(&mut self) -> &mut ParseState<U> {
        self.state_stack.last_mut().expect("empty state stack")
    }

    #[track_caller]
    pub fn undo(&mut self) -> ParseState<U> {
        let mut prev = self.state_stack.pop().expect("empty state stack");
        prev.undo(self.state_mut());
        prev
    }

    #[track_caller]
    pub fn pop(&mut self) -> ParseState<U> {
        let mut prev = self.state_stack.pop().expect("empty state stack");
        prev.pop(self.state_mut());
        prev
    }

    pub fn new_state(&mut self) -> &mut ParseState<U> {
        let new_s = ParseState::new(self.state().cursor.clone());
        self.state_stack.push(new_s);
        self.state_mut()
    }

    pub fn push(&mut self) -> &mut ParseState<U> {
        let new_s = ParseState::from_state(self.state());
        self.state_stack.push(new_s);
        self.state_mut()
    }

    pub fn merge(&mut self) -> &mut ParseState<U> {
        let prev = self.pop();
        self.state_mut().merge(&prev)
    }
}
