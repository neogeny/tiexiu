// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's model_test.py

use tiexiu::api::compile;
use tiexiu::input::StrCursor;
use tiexiu::peg::Grammar;
use tiexiu::state::corectx::CoreCtx;

fn _parse_input(grammar: &Grammar, input: &str) -> tiexiu::trees::Tree {
    let cursor = StrCursor::new(input);
    let ctx = CoreCtx::new(cursor, &[]);
    match grammar.parse(ctx) {
        Ok(s) => s.1,
        Err(f) => panic!("Failed to parse at mark {}: {:?}", f.mark, f.source),
    }
}

#[test]
fn test_children() {
    let grammar = r#"
        @@grammar::Calc

        start = expression $ ;

        expression = term ;
        term = 'x' ;
    "#;

    let _model = compile(grammar, &[]).expect("Failed to compile");
}

#[test]
fn test_node_kwargs() {
    let grammar = r#"
        start = 'value' ;
    "#;

    let _model = compile(grammar, &[]).expect("Failed to compile");
}
