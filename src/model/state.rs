// Copyright (c) 2017-2026 Juancarlo Añez (apalala@gmail.com)
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
            ast: Ast { fields: self.ast.fields.clone() },
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
        // Merge the other AST into our own
        for (key, value) in other.ast.fields {
            self.ast.fields.insert(key, value);
        }
    }

    /// The final result of this state. Consumes the state's CST.
    pub fn node(&mut self) -> Parsed {
        // 1. Check for the special override value
        if let Some(val) = self.ast.fields.get("__value__").and_then(|v| v.as_ref()) {
            // We need to return a Parsed version of this CST item
            return Parsed::new(ParsedValue::Cst(val.clone()));
        }

        // 2. Otherwise return the finalized CST or Void
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