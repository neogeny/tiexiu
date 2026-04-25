// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for join - uses compile() which has BUG

use tiexiu::Result;
use tiexiu::api::compile;

#[test]
fn test_positive_join() -> Result<()> {
    let grammar = r#"
        start = ','%{'x' 'y'}+ ;
    "#;
    compile(grammar, &[])?;
    Ok(())
}
