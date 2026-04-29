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
