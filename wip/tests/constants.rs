// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/constants_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::parse;

#[test]
fn test_constant_interpolation() {
    let grammar = r#"
        start = a:number b: number i:`"seen: {a}, {b}"` $ ;
        number = /\d+/ ;
    "#;

    let result = parse(grammar, "42 69");
    assert!(result.is_ok());
}

#[test]
fn test_constant_interpolation_free() {
    let grammar = r#"
        start = a:number b: number i:`seen: {a}, {b}` $ ;
        number = /\d+/ ;
    "#;

    let result = parse(grammar, "42 69");
    assert!(result.is_ok());
}

#[test]
fn test_constant_interpolation_multiline() {
    let grammar = r#"
        start = a:number b: number
        i:```
        seen:
        {a}
        {b}
        ``` $ ;
        number = /\d+/ ;
    "#;

    let result = parse(grammar, "42 69");
    assert!(result.is_ok());
}

#[test]
fn test_evaluate_constant() {
    let grammar = r#"
        @@grammar :: constants
        start = int bool str null 'a' $;

        int = `42` ;
        bool = `True` ;
        str = `Something` ;
        null = `None` ;
    "#;

    let ast = parse(grammar, "a");
    assert!(ast.is_ok());
}
