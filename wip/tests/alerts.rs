// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/alerts_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::{compile, parse};

#[test]
fn test_alert_interpolation() {
    let grammar = r#"
        start = a:number b: number i:^`"seen: {a}, {b}"` $ ;
        number::int = /\d+/ ;
    "#;

    let ast = parse(grammar, "42 69");
    assert!(ast.is_ok());

    let ast = parse(grammar, "42 69");
    assert!(ast.is_ok());
}
