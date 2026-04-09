// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's model_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::api::compile;
use crate::trees::Tree;

#[test]
fn test_node_kwargs() {
    struct Atom {
        ast: Option<serde_json::Value>,
        symbol: Option<String>,
    }

    impl Atom {
        fn new() -> Self {
            Self {
                ast: None,
                symbol: None,
            }
        }

        fn with_symbol(mut self, symbol: String) -> Self {
            self.symbol = Some(symbol);
            self
        }

        fn with_ast(mut self, ast: serde_json::Value) -> Self {
            self.ast = Some(ast);
            self
        }
    }

    let atom = Atom::new().with_symbol("foo".to_string());
    assert_eq!(format!("{:?}", atom).split('<').next().unwrap(), "Atom");
    assert!(atom.ast.is_none());
    assert_eq!(atom.symbol.as_ref().unwrap(), "foo");

    let atom = Atom::new()
        .with_symbol("foo".to_string())
        .with_ast(serde_json::json!({}));
    assert_eq!(atom.ast, Some(serde_json::json!({})));
    assert!(atom.symbol.is_some());

    let atom = Atom::new()
        .with_ast(serde_json::json!(42))
        .with_symbol("foo".to_string());
    assert_eq!(atom.ast, Some(serde_json::json!(42)));
    assert!(atom.symbol.is_some());

    let atom = Atom::new()
        .with_ast(serde_json::json!({"bar": 1}))
        .with_symbol("foo".to_string());
    assert_eq!(atom.ast, Some(serde_json::json!({"bar": 1})));
}

#[test]
fn test_children() {
    let grammar = r#"
        @@grammar::Calc

        start
            =
            expression $
            ;


        expression
            =
            | add:addition
            | sub:subtraction
            | term:term
            ;


        addition::Add
            =
            left:term op:'+' ~ right:expression
            ;


        subtraction::Subtract
            =
            left:term op:'-' ~ right:expression
            ;


        term
            =
            | mul:multiplication
            | div:division
            | factor:factor
            ;


        multiplication::Multiply
            =
            left:factor op:'*' ~ right:term
            ;


        division::Divide
            =
            left:factor '/' ~ right:term
            ;


        factor
            =
            | subexpression
            | number
            ;


        subexpression
            =
            '(' ~ @:expression ')'
            ;


        number::int
            =
            /\d+/
            ;
    "#;

    let parser = compile(grammar).expect("Failed to compile");
    let model = parser
        .parse("3 + 5 * ( 10 - 20 )")
        .expect("Failed to parse");
    assert!(!model.to_string().is_empty());
}

#[test]
fn test_model_repr() {
    let node = Tree::new();
    let repr = format!("{:?}", node);
    assert!(repr.contains("Node"));
}

#[test]
fn test_calc_repr() {
    let grammar = r#"
        @@grammar::CALC
        start = expression $ ;
        expression = addition | subtraction | term ;
        addition[Add] = left:expression '+' ~ right:term ;
        subtraction[Subtract] = left:expression '-' ~ right:term ;
        term = multiplication | division | factor ;
        multiplication[Multiply] = left:term '*' ~ right:factor ;
        division[Divide] = left:term '/' ~ right:factor ;
        factor = '(' ~ =expression ')' | number ;
        number[int] = /\d+/ ;
    "#;

    let model = compile(grammar, "CALC").expect("Failed to compile");
    let modelrepr = model.to_string();
    assert!(modelrepr.contains("start"));
}

#[test]
fn test_model_json() {
    let grammar = r#"
        @@grammar::Test
        start = 'hello' $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let json = model.to_json_string();
    assert!(json.is_ok());
}

#[test]
fn test_name_guard() {
    let grammar_nameguard = r#"
        @@grammar::Test
        @@nameguard :: True

        start = 'if' NAME $ ;
        NAME = /[a-z]+/ ;
    "#;

    let guard_model = compile(grammar_nameguard).expect("Failed to compile");
    let _ = guard_model.parse("if").expect("Failed to parse");

    let grammar_no_nameguard = r#"
        @@grammar::Test
        @@nameguard :: False

        start = 'if' NAME $ ;
        NAME = /[a-z]+/ ;
    "#;

    let no_guard_model = compile(grammar_no_nameguard).expect("Failed to compile");
    let _ = no_guard_model.parse("if iffy").expect("Failed to parse");
}

#[test]
fn test_whitespace_directive() {
    let grammar = r#"
        @@grammar::Test
        @@whitespace :: /\s+/

        start = 'hello' 'world' $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("hello   world").expect("Failed to parse");
}

#[test]
fn test_whitespace_none() {
    let grammar = r#"
        @@grammar::Test
        @@whitespace :: None

        start = 'hello' 'world' $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("helloworld");
    assert!(result.is_err());
}

#[test]
fn test_comments_directive() {
    let grammar = r#"
        @@grammar::Test
        @@comments :: /;\s*(.*)$/
        @@whitespace :: /\s+/

        start = 'hello' 'world' $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model
        .parse("hello ; comment\nworld")
        .expect("Failed to parse");
}

#[test]
fn test_keywords_directive() {
    let grammar = r#"
        @@grammar::Test
        @@keywords :: ['if', 'else', 'while']

        start = NAME $ ;
        NAME = /[a-z]+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let _ = model.parse("hello").expect("Failed to parse");
}

#[test]
fn test_left_recursion() {
    let grammar = r#"
        @@grammar::Arith
        @@left_recursion :: True

        start = expr $ ;

        expr = term (('+' | '-') term)* ;
        term = factor (('*' | '/') factor)* ;
        factor = NUMBER | '(' expr ')' ;

        NUMBER = /\d+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("3 + 5 * ( 10 - 20 )").expect("Failed to parse");
}

#[test]
fn test_left_recursion_off() {
    let grammar = r#"
        @@grammar::Arith
        @@left_recursion :: False

        start = expr $ ;

        expr = term (('+' | '-') term)* ;
        term = factor (('*' | '/') factor)* ;
        factor = NUMBER | '(' expr ')' ;

        NUMBER = /\d+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
}

#[test]
fn test_indirect_left_recursion() {
    let grammar = r#"
        @@grammar::Test

        start = expr $ ;

        expr = term ;
        term = factor ;
        factor = '(' expr ')' | atom ;
        atom = expr | 'x' ;

        x = 'x' ;
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}

#[test]
fn test_override_directive() {
    let grammar = r#"
        @@grammar::Test

        start = @:expression $ ;

        expression = term (('+' | '-') ~ term)* ;
        term = factor (('*' | '/') ~ factor)* ;
        factor = NUMBER | '(' expression ')' ;

        NUMBER = /\d+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("1 + 2 * 3").expect("Failed to parse");
}

#[test]
fn test_named_elements() {
    let grammar = r#"
        @@grammar::Test

        start = person $ ;

        person = first:name last:name age:number ;

        name = /[A-Z][a-z]+/ ;
        number = /\d+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("John Doe 42").expect("Failed to parse");
}

#[test]
fn test_named_list() {
    let grammar = r#"
        @@grammar::Test

        start = items+=item+ ;

        item = /\w+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("foo bar baz").expect("Failed to parse");
}

#[test]
fn test_optional() {
    let grammar = r#"
        @@grammar::Test

        start = 'hello' ['world'] $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result1 = model.parse("hello world").expect("Failed to parse");
    let result2 = model.parse("hello").expect("Failed to parse");
}

#[test]
fn test_closure() {
    let grammar = r#"
        @@grammar::Test

        start = 'a'* ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("aaa").expect("Failed to parse");
}

#[test]
fn test_positive_closure() {
    let grammar = r#"
        @@grammar::Test

        start = 'a'+ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("aaa").expect("Failed to parse");
    assert!(model.parse("").is_err());
}

#[test]
fn test_group() {
    let grammar = r#"
        @@grammar::Test

        start = ('a' 'b')* $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("ab ab ab").expect("Failed to parse");
}

#[test]
fn test_skip_group() {
    let grammar = r#"
        @@grammar::Test

        start = (?: 'a' 'b')* $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("ab ab ab").expect("Failed to parse");
}

#[test]
fn test_lookahead_positive() {
    let grammar = r#"
        @@grammar::Test

        start = &'a' 'a' 'b' ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("ab").expect("Failed to parse");
    assert!(model.parse("bb").is_err());
}

#[test]
fn test_lookahead_negative() {
    let grammar = r#"
        @@grammar::Test

        start = !'b' 'a' ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("a").expect("Failed to parse");
    assert!(model.parse("ba").is_err());
}

#[test]
fn test_grammar_directive() {
    let grammar = r#"
        @@grammar :: MyGrammar

        start = 'test' $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    assert_eq!(model.name, "MyGrammar");
}

#[test]
fn test_memoization() {
    let grammar = r#"
        @@grammar::Test

        start = expr $ ;

        expr = term (('+' | '-') term)* ;
        term = factor (('*' | '/') factor)* ;
        factor = NUMBER | '(' expr ')' ;

        NUMBER = /\d+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let _ = model.parse("1 + 2");
    let _ = model.parse("3 * 4");
    let _ = model.parse("5 - 6");
}

#[test]
fn test_memoization_off() {
    let grammar = r#"
        @@grammar::Test
        @@memoization :: False

        start = expr $ ;

        expr = term (('+' | '-') term)* ;
        term = NUMBER ;
        NUMBER = /\d+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("1 + 2").expect("Failed to parse");
}

#[test]
fn test_gather() {
    let grammar = r#"
        @@grammar::Test

        start = ','.{item}* $ ;

        item = /\w+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("foo, bar, baz").expect("Failed to parse");
}

#[test]
fn test_join() {
    let grammar = r#"
        @@grammar::Test

        start = ','%{'item'}* $ ;

        item = /\w+/ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("foo, bar, baz").expect("Failed to parse");
}

#[test]
fn test_pretty_print() {
    let grammar = r#"
        @@grammar::Test

        start = 'hello' 'world' ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let pretty = model.to_string();
    let reparsed = compile(&pretty).expect("Failed to recompile pretty output");
}

#[test]
fn test_constant() {
    let grammar = r#"
        @@grammar::Test

        start = `42` $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("").expect("Failed to parse");
}

#[test]
fn test_rule_include() {
    let grammar = r#"
        @@grammar::Test

        start = >base ;

        base = 'hello' $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("hello").expect("Failed to parse");
}

#[test]
fn test_dotted_rule_name() {
    let grammar = r#"
        @@grammar::Test

        start = foo.bar ;

        foo.bar = 'test' ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("test").expect("Failed to parse");
}

#[test]
fn test_eof() {
    let grammar = r#"
        @@grammar::Test

        start = 'hello' $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("hello").expect("Failed to parse");
}

#[test]
fn test_dot() {
    let grammar = r#"
        @@grammar::Test

        start = /./ $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let result = model.parse("x").expect("Failed to parse");
}

#[test]
fn test_patterns_with_newlines() {
    let grammar = r#"
        @@whitespace :: /[ \t]/
        start
            =
            blanklines $
            ;

        blanklines
            =
            blankline [blanklines]
            ;

        blankline
            =
            /(?m)^[^\n]*\n$/
            ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let _ast = model.parse("\n\n").expect("Failed to parse");
}

#[test]
fn test_ignorecase_not_for_pattern() {
    let grammar = r#"
        @@ignorecase
        start
            =
            {word} $
            ;

        word
            =
            /[a-z]+/
            ;
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}

#[test]
fn test_ignorecase_pattern() {
    let grammar = r#"
        start
            =
            {word} $
            ;

        word
            =
            /(?i)[a-z]+/
            ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let ast = model.parse("ABcD xYZ").expect("Failed to parse");
}

#[test]
fn test_multiline_pattern() {
    let grammar = r#"
        start =
        /(?x)
        foo
        bar
        / $ ;
    "#;

    let model = compile(grammar).expect("Failed to compile");
    let ast = model.parse("foobar").expect("Failed to parse");
}

#[test]
fn test_constant_interpolation() {
    let grammar = r#"
        start = a:number b: number i:`"seen: {a}, {b}"` $ ;
        number = /\d+/ ;
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}

#[test]
fn test_constant_interpolation_free() {
    let grammar = r#"
        start = a:number b: number i:`seen: {a}, {b}` $ ;
        number = /\d+/ ;
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}

#[test]
fn test_constant_interpolation_multiline() {
    let grammar = r#"
        start = a:number b: number
        i:```
        seen:
        {a}
        {b}
        ``` $ ;
        number = /\d+/ ;
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}

#[test]
fn test_evaluate_constant() {
    let grammar = r#"
        @@grammar :: constants
        start = int bool str null 'a' $;

        int = `42` ;
        bool = `True` ;
        str = `Something` ;
        null = `None` ;
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}

#[test]
fn test_parse_ebnf() {
    let grammar = r#"
        /*
            Example of a grammar that mixes TatSu and EBNF
        */
        @@grammar :: TatSu

        start := expression $

        expression := expression '+' term | expression '-' term | term

        term := term '*' factor | term '/' factor | factor

        factor := '(' expression ')' | number

        number := /\d+/
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}

#[test]
fn test_optional_syntax() {
    let grammar = r#"
        start:  '[' /abc/

        other := 'xyz'?
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}

#[test]
fn test_one_line_grammar() {
    let grammar = r#"
        start: lisp

        lisp: sexp | list | symbol

        sexp[SExp]: '(' cons=lisp '.' ~ cdr=lisp ')'

        list[List]: '(' ={lisp} ')'

        symbol[Symbol]: /[^\s().]+/

    "#;

    let _model = compile(grammar).expect("Failed to compile");
}

#[test]
fn test_skip_to() {
    let grammar = r#"
        start = 'x' ab $ ;

        ab
            =
            | 'a' 'b'
            | ->'a' 'b'
            ;
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}

#[test]
fn test_skip_to_with_lookahead() {
    let grammar = r#"
        start = 'x' ab $ ;

        ab
            =
            | 'a' 'b'
            | ->&'a' 'a' 'b'
            ;
    "#;

    let _model = compile(grammar).expect("Failed to compile");
}
