// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for parameters - uses compile() which has BUG

use tiexiu::Result;
use tiexiu::api::compile;

#[test]
fn test_keyword_params() -> Result<()> {
    let grammar = r#"
        start = rule[param] ;
        rule[:param] = 'test' ;
    "#;
    compile(grammar, &[])?;
    Ok(())
}
