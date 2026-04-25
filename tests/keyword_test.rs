// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for keyword - uses compile() which has BUG

use tiexiu::Result;

#[test]
fn test_keywords_in_rule_names() -> Result<()> {
    let grammar = r#"
        start = whitespace ;
        whitespace = ' ' ;
    "#;
    let _result = tiexiu::api::compile(grammar, &[])?;
    Ok(())
}
