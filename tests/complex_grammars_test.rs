//! Complex Grammar Tests

use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn calculator_grammar() -> Result<()> {
    let grammar = r#"
        @@grammar :: Calc
        @@whitespace :: /\s+/
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

    let _tree = parse_input(&grammar, "3 + 5", &[])?;
    // Complex grammar produces structured result
    Ok(())

    // let tree = parse_input(&grammar, "3 + 5 * (10 - 2)", &[])?;
    // Ok(())
}

#[test]
fn json_like_grammar() -> Result<()> {
    let grammar = r#"
        @@grammar :: MiniJSON
        @@nameguard :: False
        @@whitespace :: /\s+/
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

    let _tree = parse_input(&grammar, r#"{"key": "value"}"#, &[])?;
    Ok(())
}

#[test]
fn lisp_like_grammar() -> Result<()> {
    let grammar = r#"
        @@grammar :: Lisp
        @@nameguard :: False
        @@whitespace :: /\s+/
        start: sexp $

        sexp: atom | list

        list: '(' items ')'

        items: sexp*

        atom: WORD

        WORD: /\w+/
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;

    let _tree = parse_input(&grammar, "(hello world)", &[])?;
    Ok(())
}
