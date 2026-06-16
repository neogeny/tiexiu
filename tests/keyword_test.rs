// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for keyword - translated from TatSu's grammar/keyword_test.py

use serde_json::json;
use tiexiu::Result;
use tiexiu::api::compile;

#[test]
fn test_keywords_in_rule_names() -> Result<()> {
    let grammar = r#"
        start = whitespace ;
        whitespace = {'x'}+ ;
    "#;
    let model = compile(grammar, &[])?;
    let ast = tiexiu::parse_input(&model, "x", &[])?;
    assert_eq!(ast.to_json(), json!(["x"]));
    Ok(())
}
