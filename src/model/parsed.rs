// Copyright (c) 2017-2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::cst::Cst;
use super::ast::Ast;

/// The internal variants of parsed data.
pub enum ParsedValue {
    Token(String),
    Cst(Cst),
    Ast(Ast),
}

/// The sovereign result of a parsing rule.
/// 
/// This struct replaces 'dyn Any' entirely. It carries both the 
/// structural data (Kind) and the grammar-defined type label.
pub struct Parsed {
    pub typename: Option<String>,
    pub value: ParsedValue,
}

impl Parsed {
    /// Creates a new, unlabeled Parsed result.
    pub fn new(kind: ParsedValue) -> Self {
        Self {
            typename: None,
            value: kind,
        }
    }

    /// Sets the grammar-defined type for this result.
    pub fn set_type(&mut self, name: &str) {
        self.typename = Some(name.to_string());
    }

    /// Clears the type label, returning the result to a raw state.
    pub fn clear_type(&mut self) {
        self.typename = None;
    }
}

// Ergonomic conversions to create Parsed from raw types
impl From<String> for Parsed {
    fn from(s: String) -> Self {
        Parsed::new(ParsedValue::Token(s))
    }
}

impl From<Cst> for Parsed {
    fn from(c: Cst) -> Self {
        Parsed::new(ParsedValue::Cst(c))
    }
}

impl From<Ast> for Parsed {
    fn from(a: Ast) -> Self {
        Parsed::new(ParsedValue::Ast(a))
    }
}