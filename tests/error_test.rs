// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for error handling

use tiexiu::Result;

#[test]
fn test_missing_rule() -> Result<()> {
    let grammar = r#"
        @@grammar::TestGrammar
        block = test ;
    "#;

    let _result = tiexiu::api::compile(grammar, &[])?;
    Ok(())
}

#[test]
fn test_error_exists() -> Result<()> {
    // Simple check that Error type exists
    fn _check_error(_: tiexiu::Error) {}
    Ok(())
}
