// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A translation of the TatSu module with the same name

use super::memo::MemoCache;
use super::trace::{NULL_TRACER, Tracer};
use crate::input::Cursor;
use crate::parser::TokenStack;
use crate::trees::Tree;
use crate::util::fuse::Fuse;
use crate::util::pyre::Pattern;
use std::collections::HashMap;

pub const _AT_: &str = "__value__";

pub type PatternCache = HashMap<String, Pattern>;

pub type CallStack = TokenStack;

#[derive(Debug, Clone)]
pub struct Alert {
    pub level: usize,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ParseState<U: Cursor + Clone> {
    fuse: Fuse,
    pub cursor: U,
    pub ast: Tree,
    pub cst: Tree,
    pub cutseen: bool,
    pub last_node: Tree,
    pub alerts: Vec<Alert>,
    pub callstack: CallStack,
}

#[derive(Debug, Clone)]
pub struct HeavyState<'t> {
    pub memos: MemoCache,
    pub patterns: PatternCache,
    pub keywords: Box<[Box<str>]>,
    pub tracer: &'t dyn Tracer,
}

#[derive(Debug, Clone)]
pub struct ParseStateStack<U: Cursor + Clone> {
    pub state_stack: Vec<ParseState<U>>,
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
            tracer: &NULL_TRACER,
        }
    }

    pub fn get_pattern(&mut self, pattern: &str) -> Pattern {
        self.patterns
            .entry(pattern.to_string())
            .or_insert_with(|| Pattern::new(pattern).unwrap())
            .clone()
    }
}

impl<U: Cursor + Clone> ParseState<U> {
    pub fn new(cursor: U) -> Self {
        Self {
            cursor,
            ast: Tree::Nil,
            cst: Tree::Nil,
            cutseen: false,
            last_node: Tree::Nil,
            alerts: Vec::new(),
            callstack: CallStack::new(),
            fuse: Fuse::default(),
        }
    }

    pub fn from_state(other: &Self) -> Self {
        Self {
            cursor: other.cursor.clone(),
            ast: other.ast.clone(),
            cst: Tree::Nil,
            cutseen: false,
            last_node: Tree::Nil,
            alerts: other.alerts.clone(),
            callstack: other.callstack.clone(),
            fuse: Fuse::default(),
        }
    }

    pub fn merge(&mut self, prev: &mut Self) -> &mut Self {
        prev.pop();
        self.ast = prev.ast.clone();
        self.extend(prev.cst.clone());
        self.alerts.extend(prev.alerts.clone());
        self.cursor.reset(prev.cursor.mark());
        self
    }

    pub fn pop(&mut self) {
        self.fuse.burn();
    }

    pub fn is_popped(&self) -> bool {
        self.fuse.is_burnt()
    }

    pub fn node(&self) -> Tree {
        // NOTE: In Python this checks if _AT_ in ast.
        if !matches!(self.ast, Tree::Nil) {
            // Check for __value__ in Tree::Map
            if let Tree::Map(m) = &self.ast {
                if let Some(val) = m.get(_AT_) {
                    return val.clone();
                }
            }
            self.ast.clone()
        } else {
            self.cst.clone()
        }
    }

    pub fn append(&mut self, node: Tree) -> Tree {
        self.last_node = node.clone();
        self.cst = self.cst.clone().append(node.clone());
        node
    }

    pub fn extend(&mut self, node: Tree) -> Tree {
        self.last_node = node.clone();
        self.cst = self.cst.clone().merge(node.clone());
        node
    }
}

impl<U: Cursor + Clone> ParseStateStack<U> {
    pub fn new(cursor: U) -> Self {
        Self {
            state_stack: vec![ParseState::new(cursor)],
        }
    }

    pub fn state(&self) -> &ParseState<U> {
        self.state_stack.last().expect("empty state stack")
    }

    pub fn state_mut(&mut self) -> &mut ParseState<U> {
        self.state_stack.last_mut().expect("empty state stack")
    }

    pub fn node(&self) -> Tree {
        self.state().node()
    }

    pub fn undo(&mut self) -> ParseState<U> {
        let mut prev = self.state_stack.pop().expect("empty state stack");
        prev.pop();
        prev
    }

    pub fn pop(&mut self) -> ParseState<U> {
        let mut prev = self.state_stack.pop().expect("empty state stack");
        prev.pop();
        self.state_mut().cursor.reset(prev.cursor.mark());
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

    pub fn alert(&mut self, level: usize, message: String) -> Alert {
        let alert = Alert { level, message };
        self.state_mut().alerts.push(alert.clone());
        alert
    }
}
