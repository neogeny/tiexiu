// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for join - translated from TatSu's grammar/join_test.py

#[macro_use]
extern crate json;
use tiexiu::Result;
use tiexiu::api::compile;

#[test]
fn test_positive_join() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /\s+/
        @@nameguard :: False
        start = ','%{'x' 'y'}+ ;
    "#;
    let model = compile(grammar, &[])?;

    let ast = tiexiu::parse_input(&model, "x y, x y", &[])?;
    assert_eq!(
        ast.to_json(),
        array![array!["x", "y"], ",", array!["x", "y"]]
    );

    let ast = tiexiu::parse_input(&model, "x y x y", &[])?;
    assert_eq!(ast.to_json(), array![array!["x", "y"]]);

    let result = tiexiu::parse_input(&model, "y x", &[]);
    assert!(result.is_err(), "Expected failure: closure not positive");

    Ok(())
}
