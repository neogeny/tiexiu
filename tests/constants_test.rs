// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for constants - uses compile() which has BUG

use tiexiu::Result;

#[test]
fn test_constant_interpolation() -> Result<()> {
    let grammar = r#"
        start = a:number b: number i:`"seen: {a}, {b}"` $ ;
        number = /\d+/ ;
    "#;
    let _result = tiexiu::api::compile(grammar, &[])?;
    Ok(())
}
