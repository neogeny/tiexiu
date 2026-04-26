//! Input Position Tests

use serde_json::json;
use tiexiu::engine::{CtxI, new_ctx};
use tiexiu::input::strcursor::StrCursor;
use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn basic_position_tracking() -> Result<()> {
    let grammar = r#"
        start: 'hello'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    let cursor = StrCursor::new("hello");
    let ctx = new_ctx(cursor, &[]);

    match grammar.parse(ctx) {
        Ok(state) => {
            let _tree = state.1;
            let ctx = state.0;
            assert!(ctx.cursor().at_end(), "Should be at end of input");
        }
        Err(f) => return Err(f.into()),
    }
    Ok(())
}

#[test]
fn multiline_input() -> Result<()> {
    let grammar = r#"
        start: 'hello' 'world'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    let tree = parse_input(&grammar, "hello\nworld", &[])?;
    assert_eq!(tree.to_json(), json!(["hello", "world"]));
    Ok(())
}
