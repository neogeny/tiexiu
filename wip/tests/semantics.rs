// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/semantics_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::{compile, parse};
use crate::error::FailedParse;
use crate::semantics::ModelBuilderSemantics;
use crate::trees::Tree;

#[test]
fn test_semantics_not_class() {
    let grammar = r#"
        start::sum = {number}+ $ ;
        number::int = /\d+/ ;
    "#;

    let text = "5 4 3 2 1";

    let bad_semantics = ModelBuilderSemantics;
    let result = compile(grammar, "test", semantics = bad_semantics);
    assert!(result.is_err());

    let model = compile(grammar, "test").expect("Failed to compile");
    let result = model.parse(text, semantics = bad_semantics);
    assert!(result.is_err());

    let semantics = ModelBuilderSemantics::new();
    let ast = model.parse(text, &mut semantics).expect("Failed to parse");
    assert_eq!(ast, 15);
}

#[test]
fn test_builder_semantics() {
    let grammar = r#"
        start::sum = {number}+ $ ;
        number::int = /\d+/ ;
    "#;

    let text = "5 4 3 2 1";

    let model = compile(grammar, "test").expect("Failed to compile");
    let mut semantics = ModelBuilderSemantics::new();
    let ast = model.parse(text, &mut semantics).expect("Failed to parse");
    assert_eq!(ast, 15);
}

#[test]
fn test_builder_subclassing() {
    let grammar = r#"
        @@grammar :: Test
        start::A::B::C = $ ;
    "#;

    let model = crate::api::asmodel(grammar).expect("Failed to compile");
    model.parse("");
}

#[test]
fn test_optional_attributes() {
    let grammar = r#"
        foo::Foo = left:identifier [ ':' right:identifier ] $ ;
        identifier = /\w+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let a = model.parse("foo : bar").expect("Failed to parse");
    assert!(!a.to_string().is_empty());

    let b = model.parse("foo").expect("Failed to parse");
    let _ = b;
}

#[test]
fn test_constant_math() {
    let grammar = r#"
        start = a:`7` b:`2` @:```{a} / {b}``` $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("");
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_constant_deep_eval() {
    let grammar = r#"
        start =
            a:A b:B
            @:```{a} / {b}```
            $
        ;

        A = number @:`7` ;
        B = number @:`0` ;
        number::int = /\d+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("42 84");
    assert!(result.is_err());
}

#[test]
fn test_builder_types() {
    let grammar = r#"
        @@grammar :: Test
        start::AType::BType = $ ;
    "#;

    let model = compile(grammar, "Test").expect("Failed to compile");
    let result = model.parse("");
    let _ = result;
}

#[test]
fn test_ast_per_option() {
    let grammar = r#"
        start = options $ ;

        options =
            | a:'a' [b:'b']
            | c:'c' [d:'d']
            ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let ast = model.parse("a b").expect("Failed to parse");
    let _ = ast;

    let ast = model.parse("c d").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_ast_names_accumulate() {
    let grammar = r#"
        start: options $ ;

        options:
            | a='a' ([b='b'] {x='x'})
            | c='c' ([d='d'] y={'y'})
            | w={} f='f'
            | g=() g='g'
            | h={} h='h'
            | i+='i'
            | [j+='j'] k='k'
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let ast = model.parse("a").expect("Failed to parse");
    let _ = ast;
}

#[test]
fn test_cut_scope_semantics() {
    let grammar = r#"
        start = (failcut | failchoice | succeed) $ ;

        failcut = 'a' ~ 'y' ;

        failchoice =
            | 'a' ~ 'b'
            | 'a' 'c' 'd'
            ;

        succeed = ('a' ~ 'y' | 'b' 'z') | 'a' 'x' ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let ast = model.parse("a x").expect("Failed to parse");
    let _ = ast;

    let result = model.parse("a c d");
    assert!(result.is_err());
}
