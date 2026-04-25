//! Error Handling Tests

use tiexiu::engine::new_ctx;
use tiexiu::input::strcursor::StrCursor;
use tiexiu::*;

#[test]
fn invalid_input_fails() -> Result<()> {
    let grammar = r#"
        start: 'a'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    let cursor = StrCursor::new("b");
    let ctx = new_ctx(cursor, &[]);

    let result = grammar.parse(ctx);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn partial_match_fails() -> Result<()> {
    let grammar = r#"
        start: 'a' 'b'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    let cursor = StrCursor::new("a");
    let ctx = new_ctx(cursor, &[]);

    let result = grammar.parse(ctx);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn empty_input_fails_when_required() -> Result<()> {
    let grammar = r#"
        start: 'a'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    let cursor = StrCursor::new("");
    let ctx = new_ctx(cursor, &[]);

    let result = grammar.parse(ctx);
    assert!(result.is_err());
    Ok(())
}
