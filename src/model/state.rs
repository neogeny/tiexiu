// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::ast::Ast;
use super::cst::{self, Cst};
use super::parsed::{Parsed, ParsedValue};
use crate::input::text::CursorBox;

pub struct ParseState<'a> {
    pub cursor: CursorBox<'a>,
    pub ast: Ast,
    pub cst: Option<Cst>,
    pub cutseen: bool,
    pub last_node: Option<Parsed>,
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
            cst: None,
            cutseen: false,
            last_node: None,
        }
    }

    pub fn clone_state(&self) -> Self {
        Self {
            cursor: self.cursor.clone(),
            ast: Ast {
                fields: self.ast.fields.clone(),
            },
            cst: None,
            cutseen: self.cutseen,
            last_node: None,
        }
    }

    pub fn merge(&mut self, other: ParseState<'a>) {
        self.cursor.goto(other.cursor.pos());
        if let Some(other_cst) = other.cst {
            self.extend(Parsed::new(ParsedValue::Cst(other_cst)));
        }
        for (key, value) in other.ast.fields {
            self.ast.fields.insert(key, value);
        }
    }

    pub fn node(&mut self) -> Parsed {
        if let Some(val) = self.ast.fields.get("__value__").and_then(|v| v.as_ref()) {
            return Parsed::new(ParsedValue::Cst(val.clone()));
        }

        match self.cst.take() {
            Some(c) => Parsed::new(ParsedValue::Cst(cst::cst_final(c))),
            None => Parsed::void(),
        }
    }

    pub fn append(&mut self, node: Parsed, as_list: bool) {
        self.last_node = Some(node.clone());
        self.cst = Some(cst::cst_add(self.cst.take(), node, as_list));
    }

    pub fn extend(&mut self, node: Parsed) {
        self.last_node = Some(node.clone());
        self.cst = Some(cst::cst_merge(self.cst.take(), node));
    }
}

impl<'a> ParseStateStack<'a> {
    pub fn new(cursor: CursorBox<'a>) -> Self {
        Self {
            states: vec![ParseState::new(cursor)],
            ruleinfo_stack: Vec::new(),
        }
    }

    /// Access the current active state.
    pub fn top(&mut self) -> &mut ParseState<'a> {
        self.states.last_mut().expect("Empty state stack")
    }

    /// Pushes a fresh state (used when starting a new rule).
    pub fn push(&mut self) {
        let new_state = self.top().clone_state();
        self.states.push(new_state);
    }

    /// Pops the top state. If pos is provided, moves the new top cursor to it.
    pub fn pop(&mut self, pos: Option<usize>) -> ParseState<'a> {
        let popped = self.states.pop().expect("State stack underflow");
        if let Some(p) = pos {
            self.top().cursor.goto(p);
        }
        popped
    }

    /// Merges the result of a successful rule into the parent state.
    pub fn merge(&mut self) {
        let child = self.states.pop().expect("State stack underflow");
        self.top().merge(child);
    }
}