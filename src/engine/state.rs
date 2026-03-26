// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;
use super::ast::{Ast, __AT__};
use super::cst::{Cst, CstRc};
use crate::input::text::CursorBox;

pub struct ParseState<'a> {
    pub cursor: CursorBox<'a>,
    pub ast: Ast,
    pub cst: CstRc,
    pub cutseen: bool,
}

/// Manages the lifecycle of states during the parse.
pub struct ParseStateStack<'a> {
    pub states: Vec<ParseState<'a>>,
    pub ruleinfo_stack: Vec<String>, // Placeholder for RuleInfo
}

impl<'a> ParseState<'a> {
    pub fn new(cursor: CursorBox<'a>) -> Self {
        Self {
            cursor,
            ast: Ast::new(),
            cst: Rc::new(Cst::Void),
            cutseen: false,
        }
    }

    pub fn clone_state(&self) -> Self {
        Self {
            cursor: self.cursor.clone(),
            ast: self.ast.clone(),
            cst: Rc::new(Cst::Void),
            cutseen: self.cutseen,
        }
    }

    pub fn merge(&mut self, other: ParseState<'a>) {
        self.cursor.goto(other.cursor.pos());
        self.extend((*other.cst).clone());
        self.ast.update(&other.ast);
    }

    pub fn append(&mut self, node: Cst) {
        let cst = std::mem::take(Rc::make_mut(&mut self.cst));
        self.cst = cst.add(node);
    }

    pub fn extend(&mut self, node: Cst) {
        let cst = std::mem::take(Rc::make_mut(&mut self.cst));
        self.cst = cst.merge(node);
    }

    /// Consumes the state and returns the resulting CST node.
    pub fn node(self) -> CstRc {
        if let Some(val) = self.ast.get(__AT__) {
            val.clone()
        } else if !self.ast.is_empty() {
            Cst::Ast(self.ast).into()
        } else {
            self.cst
        }
    }
}

impl<'a> ParseStateStack<'a> {
    pub fn new(cursor: CursorBox<'a>) -> Self {
        Self {
            states: vec![ParseState::new(cursor)],
            ruleinfo_stack: Vec::new(),
        }
    }

    pub fn top(&mut self) -> &mut ParseState<'a> {
        self.states.last_mut().expect("Empty state stack")
    }

    pub fn push(&mut self) {
        let new_state = self.top().clone_state();
        self.states.push(new_state);
    }

    pub fn undo(&mut self) -> ParseState<'a> {
        self.states.pop().expect("State stack underflow")
    }

    pub fn pop(&mut self) -> ParseState<'a> {
        let popped = self.states.pop().expect("State stack underflow");
        self.top().cursor.goto(popped.cursor.pos());
        popped
    }

    /// Merges the result of a successful rule into the parent state.
    pub fn merge(&mut self) {
        let child = self.states.pop().expect("State stack underflow");
        self.top().merge(child);
    }

    pub fn node(&mut self) -> CstRc {
        let state = self.states.pop().expect("Stack underflow");
        state.node()
    }
}
