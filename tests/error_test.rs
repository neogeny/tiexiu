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

    let result = tiexiu::api::compile(grammar, &[]);
    assert!(result.is_err(), "Expected error for missing rule 'test'");
    Ok(())
}

#[test]
fn test_error_exists() -> Result<()> {
    // Verify that Error type exists and can be matched
    let grammar = r#"
        start = 'test' $ ;
    "#;
    match tiexiu::api::compile(grammar, &[]) {
        Ok(_) => (),
        Err(e) => {
            // Error should be a proper error type
            let _: tiexiu::Error = e;
        }
    }
    Ok(())
}
