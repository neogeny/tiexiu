// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/pretty_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::compile;

// ============================================================================
// Pretty Print Tests
// ============================================================================

#[test]
fn test_pretty() {
    let grammar = r#"
        start = lisp ;
        lisp = sexp | list | symbol;
        sexp[SExp]: '(' cons:lisp '.' ~ cdr:lisp ')' ;
        list[List]: '(' @:{lisp}* ')' ;
        symbol[Symbol]: /[^\s().]+/ ;
    "#;

    let expected_pretty = r#"
        start: lisp

        lisp: sexp | list | symbol

        sexp[SExp]: '(' lisp '.' ~ lisp ')'

        list[List]: '(' ={lisp} ')'

        symbol[Symbol]: /[^\s().]+/
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}

#[test]
fn test_pretty_lean() {
    let grammar = r#"
        start = lisp ;
        lisp = sexp | list | symbol;
        sexp[SExp]: '(' cons:lisp '.' ~ cdr:lisp ')' ;
        list[List]: '(' @:{lisp}* ')' ;
        symbol[Symbol]: /[^\s().]+/ ;
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}

#[test]
fn test_slashed_pattern() {
    let grammar = r#"
        start: ?"[a-z]+/[0-9]+" $
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}
