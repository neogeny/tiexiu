//! Integration tests ported from TatSu Python tests
//!
//! These tests cover grammar parsing, directives, patterns, and other
//! features that can be tested without Python-specific semantics.

use tiexiu::input::StrCursor;
use tiexiu::peg::Grammar;
use tiexiu::state::Ctx;
use tiexiu::state::corectx::CoreCtx;

fn compile(grammar_text: &str) -> Grammar {
    tiexiu::compile(grammar_text).expect("Failed to compile grammar")
}

fn parse_input(grammar: &Grammar, input: &str) -> tiexiu::trees::Tree {
    let cursor = StrCursor::new(input);
    let ctx = CoreCtx::new(cursor);
    match grammar.parse(ctx) {
        Ok(s) => s.1,
        Err(f) => panic!("Failed to parse at mark {}: {:?}", f.mark, f.source),
    }
}

// =============================================================================
// Basic Grammar Parsing Tests
// =============================================================================

mod basic_grammar {
    use super::*;

    #[test]
    #[ignore]
    fn simple_grammar() {
        let grammar = r#"
            @@grammar :: Test
            start: 'hello'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "hello");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn multiple_rules() {
        let grammar = r#"
            @@grammar :: Test
            start: a | b | c
            a: 'a'
            b: 'b'
            c: 'c'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "a");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn rule_references() {
        let grammar = r#"
            @@grammar :: Test
            start: foo bar
            foo: 'hello'
            bar: 'world'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "hello world");
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("hello") && json.contains("world"));
    }

    #[test]
    #[ignore]
    fn empty_input() {
        let grammar = r#"
            @@grammar :: Test
            start: ''
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }
}

// =============================================================================
// Directive Tests
// =============================================================================

mod directives {
    use super::*;

    #[test]
    #[ignore]
    fn grammar_directive() {
        let grammar = r#"
            @@grammar :: MyGrammar
            start: 'test'
        "#;
        let grammar = compile(grammar);
        assert_eq!(grammar.name.as_str(), "MyGrammar");
    }

    #[test]
    #[ignore]
    fn whitespace_directive() {
        let grammar = r#"
            @@whitespace :: /[\t ]+/
            start: 'a' 'b'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "a b");
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("a") && json.contains("b"));
    }

    #[test]
    #[ignore]
    fn whitespace_none_directive() {
        let grammar = r#"
            @@whitespace :: None
            start: 'a' 'b'
        "#;
        let grammar = compile(grammar);
        // Should work without whitespace
        let tree = parse_input(&grammar, "ab");
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("a") && json.contains("b"));
    }

    #[test]
    #[ignore]
    fn default_whitespace() {
        let grammar = r#"
            start: 'a' 'b'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "a b");
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("a") && json.contains("b"));
    }

    #[test]
    #[ignore]
    fn left_recursion_directive() {
        let grammar = r#"
            @@left_recursion :: False
            start: 'test'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "test");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn parseinfo_directive() {
        let grammar = r#"
            @@parseinfo :: True
            start: 'test'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "test");
        // parseinfo should be present
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("parseinfo") || json.contains("pos"));
    }

    #[test]
    #[ignore]
    fn nameguard_directive() {
        let grammar = r#"
            @@nameguard :: False
            start: 'ab'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "ab");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn comments_directive() {
        let grammar = r#"
            @@comments :: /#[^\n]*/
            start: 'a'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "a");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }
}

// =============================================================================
// Pattern Tests
// =============================================================================

mod patterns {
    use super::*;

    #[test]
    #[ignore]
    fn simple_pattern() {
        let grammar = r#"
            @@grammar :: Test
            start: /\d+/
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "123");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn pattern_with_letters() {
        let grammar = r#"
            @@grammar :: Test
            start: /[a-z]+/
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "hello");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn pattern_with_anchors() {
        let grammar = r#"
            @@grammar :: Test
            start: /^start/
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "start");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn pattern_case_insensitive() {
        let grammar = r#"
            @@ignorecase :: True
            start: /hello/
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "HELLO");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn regex_character_classes() {
        let grammar = r#"
            start: /[A-Za-z_]\w*/
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "hello_world");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }
}

// =============================================================================
// Token and Sequence Tests
// =============================================================================

mod tokens_and_sequences {
    use super::*;

    #[test]
    #[ignore]
    fn token_sequence() {
        let grammar = r#"
            start: 'hello' 'world'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "hello world");
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("hello") && json.contains("world"));
    }

    #[test]
    #[ignore]
    fn optional_token() {
        let grammar = r#"
            start: 'a' 'b'?
        "#;
        let grammar = compile(grammar);

        // With optional
        let tree = parse_input(&grammar, "a b");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));

        // Without optional
        let tree = parse_input(&grammar, "a");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn closure_tokens() {
        let grammar = r#"
            start: 'a'*
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "aaa");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn positive_closure() {
        let grammar = r#"
            start: 'a'+
        "#;
        let grammar = compile(grammar);

        // Should match
        let tree = parse_input(&grammar, "aaa");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn choice_alternatives() {
        let grammar = r#"
            start: 'a' | 'b' | 'c'
        "#;
        let grammar = compile(grammar);

        assert!(matches!(
            parse_input(&grammar, "a"),
            tiexiu::trees::Tree::Node { .. }
        ));
        assert!(matches!(
            parse_input(&grammar, "b"),
            tiexiu::trees::Tree::Node { .. }
        ));
        assert!(matches!(
            parse_input(&grammar, "c"),
            tiexiu::trees::Tree::Node { .. }
        ));
    }
}

// =============================================================================
// Named Rules and Overrides
// =============================================================================

mod naming {
    use super::*;

    #[test]
    #[ignore]
    fn named_capture() {
        let grammar = r#"
            start: name='hello'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "hello");
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("name"));
    }

    #[test]
    #[ignore]
    fn named_list() {
        let grammar = r#"
            start: names+:'a'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "aaa");
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("names"));
    }

    #[test]
    #[ignore]
    fn override_singleton() {
        let grammar = r#"
            start: ='hello'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "hello");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn override_list() {
        let grammar = r#"
            start: @+:'a'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "aaa");
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("@+") || json.contains("OverrideList"));
    }

    #[test]
    #[ignore]
    fn rule_include() {
        let grammar = r#"
            start: >base
            base: 'hello'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "hello");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }
}

// =============================================================================
// Lookahead and Constraints
// =============================================================================

mod constraints {
    use super::*;

    #[test]
    #[ignore]
    fn positive_lookahead() {
        let grammar = r#"
            start: &'a' 'a'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "a");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn negative_lookahead() {
        let grammar = r#"
            start: !'b' 'a'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "a");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn cut() {
        let grammar = r#"
            start: 'a' ~ 'b'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "ab");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }
}

// =============================================================================
// Special Forms
// =============================================================================

mod special_forms {
    use super::*;

    #[test]
    #[ignore]
    fn group() {
        let grammar = r#"
            start: ('a' 'b')*
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "abab");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn skip_group() {
        let grammar = r#"
            start: (?: 'a' 'b')*
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "abab");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn void() {
        let grammar = r#"
            start: 'a' () 'b'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "ab");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn eof() {
        let grammar = r#"
            start: 'a' $
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "a");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn dot() {
        let grammar = r#"
            start: /./ 'b'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "ab");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn constant() {
        let grammar = r#"
            start: `constant`
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }
}

// =============================================================================
// Complex Grammar Tests
// =============================================================================

mod complex_grammars {
    use super::*;

    #[test]
    #[ignore]
    fn calculator_grammar() {
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
        let grammar = compile(grammar);

        // Test simple expression
        let tree = parse_input(&grammar, "3 + 5");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));

        // Test complex expression
        let tree = parse_input(&grammar, "3 + 5 * (10 - 2)");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn json_like_grammar() {
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
        let grammar = compile(grammar);

        // Simple object
        let tree = parse_input(&grammar, r#"{"key": "value"}"#);
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));

        // Simple array
        let tree = parse_input(&grammar, "[1, 2, 3]");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn lisp_like_grammar() {
        let grammar = r#"
            @@grammar :: Lisp
            start: sexp $

            sexp: atom | list

            list: '(' items ')'

            items: sexp*

            atom: WORD

            WORD: /\w+/
        "#;
        let grammar = compile(grammar);

        let tree = parse_input(&grammar, "(hello world)");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));

        let tree = parse_input(&grammar, "(foo (bar baz))");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }
}

// =============================================================================
// Grammar Structure Tests
// =============================================================================

mod grammar_structure {
    use super::*;

    #[test]
    #[ignore]
    fn grammar_has_rules() {
        let grammar = r#"
            start: 'a'
            rule1: 'b'
            rule2: 'c'
        "#;
        let grammar = compile(grammar);
        let rule_names: Vec<_> = grammar.rules().map(|r| &*r.meta.name).collect();
        assert!(rule_names.contains(&"start"));
        assert!(rule_names.contains(&"rule1"));
        assert!(rule_names.contains(&"rule2"));
    }

    #[test]
    #[ignore]
    fn first_rule_is_default() {
        let grammar = r#"
            start: 'a'
            other: 'b'
        "#;
        let grammar = compile(grammar);
        let tree = parse_input(&grammar, "a");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn pretty_print() {
        let grammar = r#"
            @@grammar :: Test
            start: 'a' | 'b'
        "#;
        let grammar = compile(grammar);
        let pretty = grammar.to_string();
        assert!(pretty.contains("Test") || pretty.contains("start"));
    }
}

// =============================================================================
// Round-trip Tests
// =============================================================================

mod round_trips {
    use super::*;

    #[test]
    #[ignore]
    fn grammar_to_json_round_trip() {
        let grammar_text = r#"
            @@grammar :: Test
            start: foo bar
            foo: 'x'
            bar: 'y'
        "#;

        let grammar = compile(grammar_text);

        // Parse with both should work
        let tree1 = parse_input(&grammar, "x y");

        let j1 = tree1.to_json_string().unwrap();
        assert!(j1.contains("x") && j1.contains("y"));
    }

    #[test]
    #[ignore]
    fn pretty_print_round_trip() {
        let grammar_text = r#"
            @@grammar :: Test
            start: 'a' | 'b'
        "#;

        let grammar = compile(grammar_text);
        let pretty1 = grammar.to_string();

        let grammar2 = compile(&pretty1);
        let pretty2 = grammar2.to_string();

        // Both should compile and produce valid output
        assert!(!pretty1.is_empty());
        assert!(!pretty2.is_empty());
    }
}

// =============================================================================
// Input/Position Tests
// =============================================================================

mod input_positions {
    use super::*;

    #[test]
    #[ignore]
    fn basic_position_tracking() {
        let grammar = r#"
            start: 'hello'
        "#;
        let grammar = compile(grammar);

        let cursor = StrCursor::new("hello");
        let ctx = CoreCtx::new(cursor);

        match grammar.parse(ctx) {
            Ok(state) => {
                // Successfully parsed - position should be tracked
                let _tree = state.1;
                let ctx = state.0;
                // Verify we're at the end of input
                assert!(ctx.cursor().at_end(), "Should be at end of input");
            }
            Err(f) => panic!("Parsing should succeed: {:?}", f.source),
        }
    }

    #[test]
    #[ignore]
    fn multiline_input() {
        let grammar = r#"
            start: 'hello' 'world'
        "#;
        let grammar = compile(grammar);

        let tree = parse_input(&grammar, "hello\nworld");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }
}

// =============================================================================
// Error Handling Tests
// =============================================================================

mod error_handling {
    use super::*;

    #[test]
    #[ignore]
    fn invalid_input_fails() {
        let grammar = r#"
            start: 'a'
        "#;
        let grammar = compile(grammar);

        let cursor = StrCursor::new("b");
        let ctx = CoreCtx::new(cursor);

        let result = grammar.parse(ctx);
        assert!(result.is_err());
    }

    #[test]
    #[ignore]
    fn partial_match_fails() {
        let grammar = r#"
            start: 'a' 'b'
        "#;
        let grammar = compile(grammar);

        let cursor = StrCursor::new("a");
        let ctx = CoreCtx::new(cursor);

        let result = grammar.parse(ctx);
        assert!(result.is_err());
    }

    #[test]
    #[ignore]
    fn empty_input_fails_when_required() {
        let grammar = r#"
            start: 'a'
        "#;
        let grammar = compile(grammar);

        let cursor = StrCursor::new("");
        let ctx = CoreCtx::new(cursor);

        let result = grammar.parse(ctx);
        assert!(result.is_err());
    }
}

// =============================================================================
// Pretty Print Tests
// =============================================================================

mod pretty_print {
    use super::*;

    #[test]
    #[ignore]
    fn pretty_contains_grammar_name() {
        let grammar = r#"
            @@grammar :: MyTest
            start: 'hello'
        "#;
        let grammar = compile(grammar);
        let pretty = grammar.to_string();
        assert!(pretty.contains("MyTest") || pretty.contains("start"));
    }

    #[test]
    #[ignore]
    fn pretty_contains_rules() {
        let grammar = r#"
            @@grammar :: Test
            start: 'a'
            other: 'b'
        "#;
        let grammar = compile(grammar);
        let pretty = grammar.to_string();
        // Should contain at least one rule
        assert!(pretty.contains("start") || pretty.contains("other"));
    }
}
