//! Constraints Tests (Lookahead and Cut)

#[macro_use]
extern crate json;
use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn positive_lookahead() -> Result<()> {
    let grammar = r#"
        start: &'a' 'a'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a", &[])?;
    assert_eq!(tree.to_json(), value!("a"));
    Ok(())
}

#[test]
fn negative_lookahead() -> Result<()> {
    let grammar = r#"
        start: !'b' 'a'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a", &[])?;
    assert_eq!(tree.to_json(), value!("a"));
    Ok(())
}

// Cut commits the parser - passes with whitespace between tokens
#[test]
fn cut() -> Result<()> {
    let grammar = r#"
        @@whitespace :: /\s+/
        start: 'a'~'b'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let _tree = parse_input(&grammar, "a b", &[])?;
    Ok(())
}
