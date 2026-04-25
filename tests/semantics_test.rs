// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for semantics - uses compile() which has BUG

use tiexiu::Result;
use tiexiu::api::compile;

#[test]
fn test_semantics_not_class() -> Result<()> {
    let grammar = r#"
        start::sum = {number}+ $ ;
        number::int = /\d+/ ;
    "#;
    compile(grammar, &[])?;
    Ok(())
}
