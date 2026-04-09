// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/firstfollow_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::compile;

// ============================================================================
// First/Follow Set Tests
// ============================================================================

// TODO: These tests require access to grammar internals (rulemap, lookahead)
// which are not exposed in TieXiu's public API yet.

#[test]
fn test_direct_left_recursion() {
    let grammar = r#"
        @@left_recursion :: True
        start
            =
            expre $
            ;

        expre
            =
            | expre '+' factor
            | expre '-' factor
            | expre '*' factor
            | expre '/' factor
            | factor
            ;

        factor
            =
            | '(' @:expre ')'
            | number
            ;

        number
            =
            /[0-9]+/
            ;
    "#;

    // TODO: rulemap and lookahead methods not implemented
    // model = compile(grammar, 'test')
    // expre = model.rulemap['expre']
    // factor = model.rulemap['factor']
    // number = model.rulemap['number']
    //
    // assert!(matches!(expre.exp, g.Choice))
    // assert!(expre.is_lrec)
    // assert!(g.ref('expre') in expre.lookahead())
    // assert!(g.ref('factor') in expre.lookahead())
    // assert!(g.ref('factor') not in factor.lookahead())
}

#[test]
fn test_indirect_left_recursion() {
    let grammar = r#"
        @@left_recursion :: True
        start = x $ ;
        x = expr ;
        expr = x '-' num | num;
        num = /[0-9]+/ ;
    "#;

    // TODO: rulemap and is_lrec methods not implemented
    // model = compile(grammar, 'test')
    // start = model.rulemap['start']
    // x = model.rulemap['x']
    // expr = model.rulemap['expr']
    // num = model.rulemap['num']
    //
    // assert!(x.is_lrec)
    // assert!(!expr.is_lrec)
    // assert!(!num.is_lrec)
}

#[test]
fn test_nullability() {
    let grammar = r#"
        start = e;
        e = p f;
        p = [e '+'] ;
        f = '0';
    "#;

    // TODO: rulemap and is_nullable methods not implemented
    // model = compile(grammar, 'test')
    // e = model.rulemap['e']
    // p = model.rulemap['p']
    // f = model.rulemap['f']
    //
    // assert!(e.is_lrec)
    // assert!(!p.is_lrec)
    // assert!(p.is_nullable())
    // assert!(!p.is_memo)
}
