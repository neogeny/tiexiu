// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for directives - translated from TatSu's grammar/directive_test.py

use serde_json::json;
use tiexiu::Result;
use tiexiu::api::compile;

#[test]
fn test_whitespace_directive() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        test = "test" $;
    "#;
    let model = compile(grammar, &[])?;
    let ast = tiexiu::parse_input(&model, "test", &[])?;
    assert_eq!(ast.to_json(), json!("test"));
    Ok(())
}
