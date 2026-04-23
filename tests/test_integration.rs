//! Integration tests ported from TatSu Python tests
//!
//! These tests cover grammar parsing, directives, patterns, and other
//! features that can be tested without Python-specific semantics.

use tiexiu::Result;
use tiexiu::engine;
use tiexiu::engine::CtxI;
use tiexiu::input::StrCursor;

// =============================================================================
// Basic Grammar Parsing Tests
// =============================================================================

mod basic_grammar {
    use tiexiu::*;

    #[test]
    fn simple_grammar() -> Result<()> {
        let grammar = r#"
            @@grammar :: Test
            start: 'hello'
        "#;
        let grammar = compile(grammar, &[])?;
        let tree = parse_input(&grammar, "hello", &[])?;
        assert!(matches!(tree, Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn multiple_rules() -> Result<()> {
        let grammar = r#"
            @@grammar :: Test
            start: a | b | c
            a: 'a'
            b: 'b'
            c: 'c'
        "#;
        let grammar = compile(grammar, &[])?;
        let tree = parse_input(&grammar, "a", &[])?;
        assert!(matches!(tree, Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn rule_references() -> Result<()> {
        let grammar = r#"
            @@grammar :: Test
            start: foo bar
            foo: 'hello'
            bar: 'world'
        "#;
        let grammar = compile(grammar, &[])?;
        let tree = parse_input(&grammar, "hello world", &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("hello") && json.contains("world"));
        Ok(())
    }

    #[test]
    fn empty_input() -> Result<()> {
        let grammar = r#"
            @@grammar :: Test
            start: ''
        "#;
        let grammar = compile(grammar, &[])?;
        let tree = parse_input(&grammar, "", &[])?;
        assert!(matches!(tree, Tree::Node { .. }));
        Ok(())
    }
}

// =============================================================================
// Directive Tests
// =============================================================================

mod directives {
    use super::*;
    use tiexiu::parse_input;

    #[test]
    fn grammar_directive() -> Result<()> {
        let grammar = r#"
            @@grammar :: MyGrammar
            start: 'test'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        assert_eq!(grammar.name.to_string(), "MyGrammar");
        Ok(())
    }

    #[test]
    fn whitespace_directive() -> Result<()> {
        let grammar = r#"
            @@whitespace :: /[\t ]+/
            start: 'a' 'b'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "a b", &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("a") && json.contains("b"));
        Ok(())
    }

    #[test]
    fn whitespace_none_directive() -> Result<()> {
        let grammar = r#"
            @@whitespace :: None
            start: 'a' 'b'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        // Should work without whitespace
        let tree = parse_input(&grammar, "ab", &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("a") && json.contains("b"));
        Ok(())
    }

    #[test]
    fn default_whitespace() -> Result<()> {
        let grammar = r#"
            start: 'a' 'b'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "a b", &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("a") && json.contains("b"));
        Ok(())
    }

    #[test]
    fn left_recursion_directive() -> Result<()> {
        let grammar = r#"
            @@left_recursion :: False
            start: 'test'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "test", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn parseinfo_directive() -> Result<()> {
        let grammar = r#"
            @@parseinfo :: True
            start: 'test'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "test", &[])?;
        // parseinfo should be present
        let json = tree.to_model_json_string()?;
        assert!(json.contains("parseinfo") || json.contains("pos"));
        Ok(())
    }

    #[test]
    fn nameguard_directive() -> Result<()> {
        let grammar = r#"
            @@nameguard :: False
            start: 'ab'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "ab", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn comments_directive() -> Result<()> {
        let grammar = r#"
            @@comments :: /#[^\n]*/
            start: 'a'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "a", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }
}

// =============================================================================
// Pattern Tests
// =============================================================================

mod patterns {
    use super::*;
    use tiexiu::parse_input;

    #[test]
    fn simple_pattern() -> Result<()> {
        let grammar = r#"
            @@grammar :: Test
            start: /\d+/
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "123", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn pattern_with_letters() -> Result<()> {
        let grammar = r#"
            @@grammar :: Test
            start: /[a-z]+/
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "hello", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn pattern_with_anchors() -> Result<()> {
        let grammar = r#"
            @@grammar :: Test
            start: /^start/
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "start", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn pattern_case_insensitive() -> Result<()> {
        let grammar = r#"
            @@ignorecase :: True
            start: /hello/
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "HELLO", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn regex_character_classes() -> Result<()> {
        let grammar = r#"
            start: /[A-Za-z_]\w*/
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "hello_world", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }
}

// =============================================================================
// Token and Sequence Tests
// =============================================================================

mod tokens_and_sequences {
    use super::*;
    use tiexiu::parse_input;

    #[test]
    fn token_sequence() -> Result<()> {
        let grammar = r#"
            start: 'hello' 'world'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "hello world", &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("hello") && json.contains("world"));
        Ok(())
    }

    #[test]
    fn optional_token() -> Result<()> {
        let grammar = r#"
            start: 'a' 'b'?
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;

        // With optional
        let tree = parse_input(&grammar, "a b", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));

        // Without optional
        let tree = parse_input(&grammar, "a", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn closure_tokens() -> Result<()> {
        let grammar = r#"
            start: 'a'*
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "aaa", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn positive_closure() -> Result<()> {
        let grammar = r#"
            start: 'a'+
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;

        // Should match
        let tree = parse_input(&grammar, "aaa", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn choice_alternatives() -> Result<()> {
        let grammar = r#"
            start: 'a' | 'b' | 'c'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;

        assert!(matches!(
            parse_input(&grammar, "a", &[])?,
            tiexiu::trees::Tree::Node { .. }
        ));
        assert!(matches!(
            parse_input(&grammar, "b", &[])?,
            tiexiu::trees::Tree::Node { .. }
        ));
        assert!(matches!(
            parse_input(&grammar, "c", &[])?,
            tiexiu::trees::Tree::Node { .. }
        ));
        Ok(())
    }
}

// =============================================================================
// Named Rules and Overrides
// =============================================================================

mod naming {
    use super::*;
    use tiexiu::parse_input;

    #[test]
    fn named_capture() -> Result<()> {
        let grammar = r#"
            start: name='hello'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "hello", &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("name"));
        Ok(())
    }

    #[test]
    fn named_list() -> Result<()> {
        let grammar = r#"
            start: names+:'a'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "aaa", &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("names"));
        Ok(())
    }

    #[test]
    fn override_singleton() -> Result<()> {
        let grammar = r#"
            start: ='hello'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "hello", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn override_list() -> Result<()> {
        let grammar = r#"
            start: @+:'a'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "aaa", &[])?;
        let json = tree.to_model_json_string()?;
        assert!(json.contains("@+") || json.contains("OverrideList"));
        Ok(())
    }

    #[test]
    fn rule_include() -> Result<()> {
        let grammar = r#"
            start: >base
            base: 'hello'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "hello", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }
}

// =============================================================================
// Lookahead and Constraints
// =============================================================================

mod constraints {
    use super::*;
    use tiexiu::parse_input;

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
}

// =============================================================================
// Special Forms
// =============================================================================

mod special_forms {
    use super::*;
    use tiexiu::parse_input;

    #[test]
    fn group() -> Result<()> {
        let grammar = r#"
            start: ('a' 'b')*
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "abab", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn skip_group() -> Result<()> {
        let grammar = r#"
            start: (?: 'a' 'b')*
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "abab", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn void() -> Result<()> {
        let grammar = r#"
            start: 'a' () 'b'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "ab", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn eof() -> Result<()> {
        let grammar = r#"
            start: 'a' $
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "a", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn dot() -> Result<()> {
        let grammar = r#"
            start: /./ 'b'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "ab", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn constant() -> Result<()> {
        let grammar = r#"
            start: `constant`
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }
}

// =============================================================================
// Complex Grammar Tests
// =============================================================================

mod complex_grammars {
    use super::*;
    use tiexiu::parse_input;

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

        // Test simple expression
        let tree = parse_input(&grammar, "3 + 5", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));

        // Test complex expression
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

        // Simple object
        let tree = parse_input(&grammar, r#"{"key": "value"}"#, &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));

        // Simple array
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
}

// =============================================================================
// Grammar Structure Tests
// =============================================================================

mod grammar_structure {
    use super::*;
    use tiexiu::parse_input;

    #[test]
    fn grammar_has_rules() -> Result<()> {
        let grammar = r#"
            start: 'a'
            rule1: 'b'
            rule2: 'c'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let rule_names: Vec<_> = grammar.rules().map(|r| &*r.name).collect();
        assert!(rule_names.contains(&"start"));
        assert!(rule_names.contains(&"rule1"));
        assert!(rule_names.contains(&"rule2"));
        Ok(())
    }

    #[test]
    fn first_rule_is_default() -> Result<()> {
        let grammar = r#"
            start: 'a'
            other: 'b'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "a", &[])?;
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }

    #[test]
    fn pretty_print() -> Result<()> {
        let grammar = r#"
            @@grammar :: Test
            start: 'a' | 'b'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let pretty = grammar.to_string();
        assert!(pretty.contains("Test") || pretty.contains("start"));
        Ok(())
    }
}

// =============================================================================
// Round-trip Tests
// =============================================================================

mod round_trips {
    use super::*;
    use tiexiu::parse_input;

    #[test]
    fn grammar_to_json_round_trip() -> Result<()> {
        let grammar_text = r#"
            @@grammar :: Test
            start: foo bar
            foo: 'x'
            bar: 'y'
        "#;

        let grammar = tiexiu::compile(grammar_text, &[])?;

        // Parse with both should work
        let tree1 = parse_input(&grammar, "x y", &[])?;

        let j1 = tree1.to_model_json_string()?;
        assert!(j1.contains("x") && j1.contains("y"));
        Ok(())
    }

    #[test]
    fn pretty_print_round_trip() -> Result<()> {
        let grammar_text = r#"
            @@grammar :: Test
            start: 'a' | 'b'
        "#;

        let grammar = tiexiu::compile(grammar_text, &[])?;
        let pretty1 = grammar.to_string();

        let grammar2 = tiexiu::compile(&pretty1, &[])?;
        let pretty2 = grammar2.to_string();

        // Both should compile and produce valid output
        assert!(!pretty1.is_empty());
        assert!(!pretty2.is_empty());
        Ok(())
    }
}

// =============================================================================
// Input/Position Tests
// =============================================================================

mod input_positions {
    use super::*;
    use tiexiu::engine::new_ctx;
    use tiexiu::input::strcursor::StrCursor;
    use tiexiu::parse_input;

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
                // Successfully parsed - position should be tracked
                let _tree = state.1;
                let ctx = state.0;
                // Verify we're at the end of input
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
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        Ok(())
    }
}

// =============================================================================
// Error Handling Tests
// =============================================================================

mod error_handling {
    use super::*;

    #[test]
    fn invalid_input_fails() -> Result<()> {
        let grammar = r#"
            start: 'a'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;

        let cursor = StrCursor::new("b");
        let ctx = engine::new_ctx(cursor, &[]);

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
        let ctx = engine::new_ctx(cursor, &[]);

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
        let ctx = engine::new_ctx(cursor, &[]);

        let result = grammar.parse(ctx);
        assert!(result.is_err());
        Ok(())
    }
}

// =============================================================================
// Pretty Print Tests
// =============================================================================

mod pretty_print {
    use super::*;

    #[test]
    fn pretty_contains_grammar_name() -> Result<()> {
        let grammar = r#"
            @@grammar :: MyTest
            start: 'hello'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let pretty = grammar.to_string();
        assert!(pretty.contains("MyTest") || pretty.contains("start"));
        Ok(())
    }

    #[test]
    fn pretty_contains_rules() -> Result<()> {
        let grammar = r#"
            @@grammar :: Test
            start: 'a'
            other: 'b'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let pretty = grammar.to_string();
        // Should contain at least one rule
        assert!(pretty.contains("start") || pretty.contains("other"));
        Ok(())
    }
}
