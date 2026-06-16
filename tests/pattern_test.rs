// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for patterns - translated from TatSu's grammar/pattern_test.py

use serde_json::json;
use tiexiu::Result;
use tiexiu::api::compile;

#[test]
fn test_patterns_with_newlines() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[ \t]/
        start = blanklines $ ;
        blanklines = blankline [blanklines] ;
        blankline = /(?m)^[^\n]*\n$/ ;
    "#;
    let model = compile(grammar, &[])?;
    let ast = tiexiu::parse_input(&model, "\n\n", &[])?;
    assert_eq!(ast.to_json(), json!(["\n", "\n"]));
    Ok(())
}
