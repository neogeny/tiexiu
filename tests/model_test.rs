// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's model_test.py

use serde_json::json;
use tiexiu::Result;
use tiexiu::api::compile;

#[test]
fn test_children() -> Result<()> {
    let grammar = r#"
        @@grammar::Calc

        start = expression $ ;

        expression = term ;
        term = 'x' ;
    "#;

    let model = compile(grammar, &[])?;
    let ast = tiexiu::parse_input(&model, "x", &[])?;
    assert_eq!(ast.to_json(), json!("x"));
    Ok(())
}

#[test]
fn test_node_kwargs() -> Result<()> {
    let grammar = r#"
        start = 'value' ;
    "#;

    let model = compile(grammar, &[])?;
    let ast = tiexiu::parse_input(&model, "value", &[])?;
    assert_eq!(ast.to_json(), json!("value"));
    Ok(())
}
