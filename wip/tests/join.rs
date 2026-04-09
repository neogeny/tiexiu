// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/join_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::{compile, parse};

#[test]
fn test_positive_join() {
    let grammar = r#"
        start = ','%{'x' 'y'}+ ;
    "#;

    let grammar2 = r#"
        start = (','%{'x'}+|{}) ;
    "#;

    let grammar3 = r#"
        start = [','%{'x'}+] ;
    "#;

    let model = compile(grammar, "test").expect("Failed to compile");
    let ast = model.parse("x y, x y").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("x y x y").expect("Failed to parse");
    let _ = ast;

    let result = model.parse("y x");
    assert!(result.is_err());
}

#[test]
fn test_normal_join() {
    let grammar = r#"
        start = ','%{'x' 'y'} 'z' ;
    "#;

    let model = compile(grammar, "test").expect("Failed to compile");
    let ast = model.parse("x y, x y z").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_group_join() {
    let grammar = r#"
        start = ('a' 'b')%{'x'}+ ;
    "#;

    let model = compile(grammar, "test").expect("Failed to compile");
    let ast = model.parse("x a b x").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_positive_gather() {
    let grammar = r#"
        start = ','.{'x' 'y'}+ ;
    "#;

    let grammar2 = r#"
        start = (','.{'x'}+|{}) ;
    "#;

    let grammar3 = r#"
        start = [','.{'x'}+] ;
    "#;

    let model = compile(grammar, "test").expect("Failed to compile");
    let ast = model.parse("x y, x y").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_normal_gather() {
    let grammar = r#"
        start = ','.{'x' 'y'} 'z' ;
    "#;

    let model = compile(grammar, "test").expect("Failed to compile");
    let ast = model.parse("x y, x y z").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_group_gather() {
    let grammar = r#"
        start = ('a' 'b').{'x'}+ ;
    "#;

    let model = compile(grammar, "test").expect("Failed to compile");
    let ast = model.parse("x a b x").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_left_join() {
    let grammar = r#"
        start = (op)<{number}+ $ ;
        op = '+' | '-' ;
        number = /\d+/ ;
    "#;

    let model = compile(grammar, "test").expect("Failed to compile");
    let ast = model.parse("1 + 2 - 3 + 4").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_right_join() {
    let grammar = r#"
        start = (op)>{number}+ $ ;
        op = '+' | '-' ;
        number = /\d+/ ;
    "#;

    let model = compile(grammar, "test").expect("Failed to compile");
    let ast = model.parse("1 + 2 - 3 + 4").expect("Failed to parse");
    let _ = ast;
}
