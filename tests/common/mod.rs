// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Shared test helpers for integration tests.

use tiexiu::api::compile;
use tiexiu::{parse_input, Result};
use tiexiu::peg::Grammar;
use tiexiu::trees::Tree;

/// Compile a grammar and parse input text, returning the parse tree.
pub fn compile_and_parse(grammar: &str, input: &str) -> Result<Tree> {
    let grammar = compile(grammar, &[])?;
    parse_input(&grammar, input, &[])
}

/// Compile a grammar string and return the compiled Grammar.
pub fn compile_grammar(grammar: &str) -> Result<Grammar> {
    compile(grammar, &[])
}
