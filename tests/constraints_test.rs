//! Constraints Tests (Lookahead and Cut)

use serde_json::json;
use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn positive_lookahead() -> Result<()> {
    let grammar = r#"
        start: &'a' 'a'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a", &[])?;
    assert_eq!(tree.to_value(), json!("a"));
    Ok(())
}

#[test]
fn negative_lookahead() -> Result<()> {
    let grammar = r#"
        start: !'b' 'a'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a", &[])?;
    assert_eq!(tree.to_value(), json!("a"));
    Ok(())
}

// Cut commits the parser to the first successful path within a sequence.
// ISSUE: The grammar `'a'~'b'` with input "ab" fails unexpectedly.
// Error: ExpectedToken("a") at position 0, even though input starts with 'a'.
// This is very puzzling - it suggests the token matching itself may be broken,
// or the grammar is not being parsed correctly by the boot grammar.
// DEBUGGING NEEDED:
// 1. Check if the boot grammar parses 'a'~'b' correctly
// 2. Verify the Exp::cut() implementation in src/peg/exp.rs
// 3. Check if the cut operator is somehow affecting token matching
// 4. Compare with the TatSu implementation in tatsu/tests/grammar/syntax_test.py::test_cut_scope
#[test]
fn cut() -> Result<()> {
    let grammar = r#"
        start: 'a'~'b'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let _tree = parse_input(&grammar, "ab", &[])?;
    Ok(())
}