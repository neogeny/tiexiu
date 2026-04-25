// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/alerts_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use tiexiu::Result;
use tiexiu::api::parse;

#[test]
#[ignore = "TODO: broken, check the Python original"]
fn test_alert_interpolation() -> Result<()> {
    let grammar = r#"
        start = a:number b: number i:^`"seen: {a}, {b}"` $ ;
        number::int = /\d+/ ;
    "#;

    let _ast = parse(grammar, "42 69", &[])?;
    Ok(())
}
