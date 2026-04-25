// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for lookahead - uses compile() which has BUG

use tiexiu::Result;
use tiexiu::api::compile;

#[test]
fn test_skip_to() -> Result<()> {
    let grammar = r#"
        start = 'x' ab $ ;
        ab = 'a' 'b' | -> 'b' ;
    "#;
    compile(grammar, &[])?;
    Ok(())
}
