//! Constraints Tests (Lookahead and Cut)

use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn positive_lookahead() -> Result<()> {
    let grammar = r#"
        start: &'a' 'a'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    Ok(())
}

#[test]
fn negative_lookahead() -> Result<()> {
    let grammar = r#"
        start: !'b' 'a'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    Ok(())
}

#[test]
fn cut() -> Result<()> {
    let grammar = r#"
        start: 'a' ~ 'b'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "ab", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    Ok(())
}
