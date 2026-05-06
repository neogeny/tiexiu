// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for lookahead - translated from TatSu's grammar/lookahead_test.py

use json::value;
use tiexiu::Result;
use tiexiu::api::compile;

#[test]
fn test_skip_to() -> Result<()> {
    let grammar = r#"
        start = 'x' ab $ ;
        ab = 'a' 'b' | -> 'b' ;
    "#;
    let model = compile(grammar, &[])?;
    let tree = tiexiu::parse_input(&model, "x yb", &[])?;
    // Python: assert ast == ['x', 'b']
    assert_eq!(tree.to_json(), value!(["x", "b"]));
    Ok(())
}
