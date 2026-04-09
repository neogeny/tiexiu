// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/pattern_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::compile;

// ============================================================================
// Pattern Tests
// ============================================================================

#[test]
fn test_patterns_with_newlines() {
    let grammar = r#"
        @@whitespace :: /[ \t]/
        start
            =
            blanklines $
            ;

        blanklines
            =
            blankline [blanklines]
            ;

        blankline
            =
            /(?m)^[^\n]*\n$/
            ;
    "#;

    let model = compile(grammar, "test").expect("Failed to compile");
    let ast = model.parse("\n\n").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_ignorecase_not_for_pattern() {
    let grammar = r#"
        @@ignorecase
        start
            =
            {word} $
            ;

        word
            =
            /[a-z]+/
            ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("ABcD xYZ");
    assert!(result.is_err());
}

#[test]
fn test_ignorecase_pattern() {
    let grammar = r#"
        start
            =
            {word} $
            ;

        word
            =
            /(?i)[a-z]+/
            ;
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}

#[test]
fn test_multiline_pattern() {
    let grammar = r#"
        start =
        /(?x)
        foo
        bar
        / $ ;
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}
