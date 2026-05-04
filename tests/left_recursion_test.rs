// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for left recursion - translated from TatSu's grammar/left_recursion_test.py

#[macro_use]
extern crate json;
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
    assert_eq!(ast.to_json(), array!["10", "-", "20"]);

    Ok(())
}
