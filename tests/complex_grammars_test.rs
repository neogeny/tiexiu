//! Complex Grammar Tests

use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn calculator_grammar() -> Result<()> {
    let grammar = r#"
        @@grammar :: Calc
        start: expression $

        expression:
            | term ('+' term)*
            | term ('-' term)*

        term:
            | factor ('*' factor)*
            | factor ('/' factor)*

        factor:
            | NUMBER
            | '(' expression ')'

        NUMBER: /\d+/
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    let tree = parse_input(&grammar, "3 + 5", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));

    let tree = parse_input(&grammar, "3 + 5 * (10 - 2)", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    Ok(())
}

#[test]
fn json_like_grammar() -> Result<()> {
    let grammar = r#"
        @@grammar :: MiniJSON
        start: value $

        value: object | array | string | number | 'true' | 'false' | 'null'

        object: '{' members? '}'

        array: '[' elements? ']'

        members: pair (',' pair)*

        elements: value (',' value)*

        pair: string ':' value

        string: '"' CONTENT '"'

        CONTENT: /[^"]*/

        number: /-?\d+(\.\d+)?/
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    let tree = parse_input(&grammar, r#"{"key": "value"}"#, &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));

    let tree = parse_input(&grammar, "[1, 2, 3]", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    Ok(())
}

#[test]
fn lisp_like_grammar() -> Result<()> {
    let grammar = r#"
        @@grammar :: Lisp
        start: sexp $

        sexp: atom | list

        list: '(' items ')'

        items: sexp*

        atom: WORD

        WORD: /\w+/
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    let tree = parse_input(&grammar, "(hello world)", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));

    let tree = parse_input(&grammar, "(foo (bar baz))", &[])?;
    assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    Ok(())
}
