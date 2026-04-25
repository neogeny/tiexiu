// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for pattern - uses compile() which has BUG

use tiexiu::Result;
use tiexiu::api::compile;

#[test]
fn test_patterns_with_newlines() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /[ \t]/
        start = /\w+/ $ ;
    "#;
    compile(grammar, &[])?;
    Ok(())
}
