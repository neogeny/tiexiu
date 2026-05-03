//! Input Position Tests

#[macro_use]
extern crate json;
use tiexiu::engine::{new_ctx, CtxI};
use tiexiu::input::strcursor::StrCursor;
use tiexiu::parse_input;
use tiexiu::*;
use tiexiu::peg::error::Yeap;

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
