// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for lookahead - translated from TatSu's grammar/lookahead_test.py

use tiexiu::api::compile;
use tiexiu::Result;

#[test]
fn test_skip_to() -> Result<()> {
    let grammar = r#"
        start = 'x' ab $ ;
        ab = 'a' 'b' | -> 'b' ;
    "#;
    let _model = compile(grammar, &[])?;
    Ok(())
}
