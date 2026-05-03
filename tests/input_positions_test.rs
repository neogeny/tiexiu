// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Input Position Tests

#[macro_use]
extern crate json;
use tiexiu::engine::{CtxI, new_ctx};
use tiexiu::input::strcursor::StrCursor;
use tiexiu::parse_input;
use tiexiu::peg::error::Yeap;
use tiexiu::*;

#[test]
fn basic_position_tracking() -> Result<()> {
    let grammar = r#"
        start: 'hello'
    "#;
    let grammar = compile(grammar, &[])?;

    let cursor = StrCursor::new("hello");
    let ctx = new_ctx(cursor, &[]);

    let Yeap(ctx, _tree) = grammar.parse(ctx.clone())?;
    assert!(ctx.cursor().at_end(), "Should be at end of input");
    Ok(())
}

#[test]
fn multiline_input() -> Result<()> {
    let grammar = r#"
        start: 'hello' 'world'
    "#;
    let grammar = compile(grammar, &[])?;

    let tree = parse_input(&grammar, "hello\nworld", &[])?;
    assert_eq!(tree.to_json(), array!["hello", "world"]);
    Ok(())
}
