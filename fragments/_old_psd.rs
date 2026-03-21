// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::cst::Cst;
use super::ast::Ast;

/// The core interface for anything produced by the parser.
pub trait Value {
    /// Returns the underlying parsed data, stripping any type labels.
    fn value(self) -> Parsed;
    
    /// Returns the grammar-defined typename, if any.
    fn typename(&self) -> Option<&str>;
}

/// The raw variants of parsed data.
pub enum Parsed {
    Token(String),
    Cst(Cst),
    Ast(Ast),
}

impl Value for Parsed {
    fn value(self) -> Parsed {
        self
    }

    fn typename(&self) -> Option<&str> {
        None
    }
}

/// A decorator that adds a type label to a Parsed value.
pub struct TypedParsed {
    pub typename: String,
    pub payload: Parsed,
}

impl TypedParsed {
    pub fn new(typename: String, payload: Parsed) -> Self {
        Self { typename, payload }
    }
}

impl Value for TypedParsed {
    fn value(self) -> Parsed {
        self.payload
    }

    fn typename(&self) -> Option<&str> {
        Some(&self.typename)
    }
}