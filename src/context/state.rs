// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A translation of the TatSu module with the same name

use super::memo::{KeyTrack, MemoCache};
use super::trace::{NULL_TRACER, Tracer};
use crate::input::Cursor;
use crate::parser::TokenStack;
use crate::peg::error::DisasterReport;
use crate::types::{Str, StrSet};
use crate::util::fuse::Fuse;
use crate::util::pyre::Pattern;
use std::collections::HashMap;

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
    fuse: Fuse,
    pub cursor: U,
    pub cutseen: bool,
    pub callstack: CallStack,
    pub keytrack: KeyTrack,
}

#[derive(Debug, Clone)]
pub struct HeavyState<'t> {
    pub memos: MemoCache,
    pub patterns: PatternCache,
    pub keywords: Box<[Str]>,
    pub strings: StrSet,
    pub furthest_failure: Option<DisasterReport>,
    pub tracer: &'t dyn Tracer,
}

#[derive(Debug, Clone)]
pub struct ParseStateStack<U: Cursor + Clone> {
    pub state_stack: Vec<ParseState<U>>,
}

impl<U: Cursor + Clone> Clone for ParseState<U> {
    fn clone(&self) -> Self {
        let mut clone = Self::new(self.cursor.clone());
        clone.callstack = self.callstack.clone();
        clone
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
            strings: StrSet::new(),
            furthest_failure: None,
            tracer: &NULL_TRACER,
        }
    }

    pub fn get_pattern(&mut self, pattern: &str) -> Pattern {
        self.patterns
            .entry(pattern.to_string())
            .or_insert_with(|| Pattern::new(pattern).unwrap())
            .clone()
    }

    pub fn intern(&mut self, s: &str) -> Str {
        if let Some(existing) = self.strings.get(s) {
            return existing.clone();
        }

        let new: Str = s.into();
        self.strings.insert(new.clone());
        new
    }

    pub fn set_furthest_failure(&mut self, dis: &DisasterReport) {
        let furthest = self.furthest_failure.clone();
        match furthest {
            Some(prev) if dis.mark >= prev.mark => {
                self.furthest_failure = Some(dis.clone());
            }
            None => {
                self.furthest_failure = Some(dis.clone());
            }
            _ => {}
        }
    }
}

impl<U: Cursor + Clone> ParseState<U> {
    pub fn new(cursor: U) -> Self {
        Self {
            cursor,
            cutseen: false,
            callstack: CallStack::new(),
            fuse: Fuse::default(),
            keytrack: KeyTrack::default(),
        }
    }

    pub fn from_state(other: &Self) -> Self {
        Self {
            cursor: other.cursor.clone(),
            cutseen: false,
            callstack: other.callstack.clone(),
            fuse: Fuse::default(),
            keytrack: KeyTrack::default(),
        }
    }

    pub fn merge(&mut self, prev: &mut Self) -> &mut Self {
        prev.burn();
        self.cursor.reset(prev.cursor.mark());
        self.callstack = prev.callstack.clone();
        self
    }

    pub fn burn(&mut self) {
        self.fuse.burn();
    }

    pub fn pop(&mut self, into: &mut Self) {
        into.callstack = self.callstack.clone();
        into.cursor.reset(self.cursor.mark());
        self.burn();
    }

    pub fn undo(&mut self, into: &mut Self) {
        into.callstack = self.callstack.clone();
        self.burn();
    }

    pub fn is_popped(&self) -> bool {
        self.fuse.is_burnt()
    }
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
        let mut prev = self.pop();
        self.state_mut().merge(&mut prev)
    }
}
