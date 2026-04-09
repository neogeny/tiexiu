// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/defines_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::{compile, parse};

// ============================================================================
// Name in Option Tests
// ============================================================================

#[test]
fn test_name_in_option() {
    let grammar = r#"
        start = expr_range ;

        expr_range =
            | [from: expr] '..' [to: expr]
            | expr
            ;

        expr =
            /[\d]+/
        ;
    "#;

    let model = compile(grammar).expect("Failed to compile");

    let ast = model.parse("1 .. 10").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("10").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse(" .. 10").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("1 .. ").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse(" .. ").expect("Failed to parse");
    let _ = ast;
}

// ============================================================================
// By Option Tests
// ============================================================================

#[test]
fn test_by_option() {
    let grammar = r#"
        start = expr_range ;

        expr_range =
            | [from: expr] '..' [to: expr]
            | left:expr ','  [right:expr]
            ;

        expr =
            /[\d]+/
        ;
    "#;

    let model = compile(grammar).expect("Failed to compile");

    let ast = model.parse(" .. 10").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("1, 2").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("1, ").expect("Failed to parse");
    let _ = ast;
}

// ============================================================================
// Inner Options Tests
// ============================================================================

#[test]
fn test_inner_options() {
    let grammar = r#"
        start = switch;
        switch = 'switch' [(on:'on'|off:'off')] ;
    "#;

    let model = compile(grammar).expect("Failed to compile");

    let ast = model.parse("switch on").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("switch off").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("switch").expect("Failed to parse");
    let _ = ast;
}
