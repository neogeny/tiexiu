// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use super::ast::{Ast, __AT__};
use super::cst::Cst;
use crate::input::text::CursorBox;

pub struct ParseState<'a> {
    pub cursor: CursorBox<'a>,
    pub ast: Ast,
    pub cst: Cst,
    pub cutseen: bool,
    pub last_node: Cst,  // FIXME: cannot keep a copy of a complet parse tree
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
            cst: Cst::Void,
            cutseen: false,
            last_node: Cst::Void,
        }
    }

    pub fn clone_state(&self) -> Self {
        Self {
            cursor: self.cursor.clone(),
            ast: Ast {
                fields: self.ast.fields.clone(),
            },
            cst: Cst::Void,
            cutseen: self.cutseen,
            last_node: Cst::Void,
        }
    }

    pub fn merge(&mut self, other: ParseState<'a>) {
        self.cursor.goto(other.cursor.pos());
        self.extend(other.cst);
        for (key, value) in other.ast.fields {
            self.ast.fields.insert(key, value);
        }
    }

    pub fn append(&mut self, node: Cst) {
        self.last_node = node.clone();
        let prev = self.cst.clone();
        self.cst = prev.add(node);
    }

    pub fn extend(&mut self, node: Cst) {
        self.last_node = node.clone();
        let prev = self.cst.clone();
        self.cst = prev.merge(node);
    }

    pub fn node(&mut self) -> Cst {
        if let Some(val) = self.ast.fields.get(__AT__) {
            val.clone()
        }
        else if self.ast.fields.len() >= 1 {
            Cst::Ast(self.ast.clone())
        }
        else{
            self.cst.clone()
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
        let popped = self.states.pop().expect("State stack underflow");
        popped
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
}