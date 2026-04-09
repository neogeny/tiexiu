// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/lookahead_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::compile;

#[test]
fn test_skip_to() {
    let grammar = r#"
        start = 'x' ab $ ;

        ab
            =
            | 'a' 'b'
            | ->'a' 'b'
            ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let ast = model.parse("x xx yyy a b").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_skip_to_with_lookahead() {
    let grammar = r#"
        start = 'x' ab $ ;

        ab
            =
            | 'a' 'b'
            | ->&'a' 'a' 'b'
            ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let ast = model.parse("x xx yyy a b").expect("Failed to parse");
    let _ = ast;
}
