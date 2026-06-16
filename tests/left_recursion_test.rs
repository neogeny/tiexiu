// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for left recursion - translated from TatSu's grammar/left_recursion_test.py

use serde_json::json;
use tiexiu::Result;
use tiexiu::api::compile;

#[test]
fn test_direct_left_recursion() -> Result<()> {
    let grammar = r#"
        @@left_recursion :: True
        @@whitespace :: /\s+/
        start = expression $ ;
        expression = expression '+' factor | expression '-' factor | factor ;
        factor = number ;
        number = /[0-9]+/ ;
    "#;
    let model = compile(grammar, &[])?;

    let ast = tiexiu::parse_input(&model, "10 - 20", &[])?;
    assert_eq!(ast.to_json(), json!(["10", "-", "20"]));

    Ok(())
}

#[test]
fn test_indirect_left_recursion() -> Result<()> {
    let grammar = r#"
        @@left_recursion :: True
        @@whitespace :: /\s+/
        start = A $ ;
        A = B | 'x' ;
        B = A | 'y' ;
    "#;
    let model = compile(grammar, &[])?;

    // Mutual left recursion: A → B → A via 'y'
    let ast = tiexiu::parse_input(&model, "y", &[])?;
    assert!(ast.to_json() == json!("y"));

    // Direct match: A → 'x'
    let ast = tiexiu::parse_input(&model, "x", &[])?;
    assert!(ast.to_json() == json!("x"));

    Ok(())
}

#[test]
fn test_lr_disabled_via_directive() -> Result<()> {
    // Grammar has left-recursive rules but LR is explicitly disabled
    let grammar = r#"
        @@left_recursion :: False
        @@whitespace :: /\s+/
        start = expression $ ;
        expression = expression '+' factor | expression '-' factor | factor ;
        factor = number ;
        number = /[0-9]+/ ;
    "#;
    let model = compile(grammar, &[])?;

    let result = tiexiu::parse_input(&model, "10 - 20", &[]);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_lr_disabled_normal_grammar() -> Result<()> {
    // Non-left-recursive grammar with LR disabled should parse fine
    let grammar = r#"
        @@left_recursion :: False
        @@whitespace :: /\s+/
        start = expr $ ;
        expr = '(' expr ')' | number ;
        number = /[0-9]+/ ;
    "#;
    let model = compile(grammar, &[])?;

    let ast = tiexiu::parse_input(&model, "((1))", &[])?;
    assert_eq!(ast.to_json(), json!(["(", json!(["(", "1", ")"]), ")"]));

    Ok(())
}
