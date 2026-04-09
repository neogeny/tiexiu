// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/return_value_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::parse;

#[test]
fn test_mixed_return() {
    let grammar = r#"
        start: ('a' b='b') 'c' [d='d']
    "#;

    let model = parse(grammar).expect("Failed to compile");
    let value = model.parse("a b c").expect("Failed to parse");
    let _ = value;
}
