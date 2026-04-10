//! Bootstrap tests for TieXiu
//!
//! Tests for parsing grammars in TatSu EBNF format.
//! Run with: cargo test --features bootstrap
//!

mod fixtures;

use fixtures::{boot_grammar, parse_ebnf, parse_with_boot};

use tiexiu::input::StrCursor;
use tiexiu::peg::Grammar;
use tiexiu::state::corectx::CoreCtx;

fn compile(grammar_text: &str) -> Grammar {
    tiexiu::compile(grammar_text).expect("Failed to compile grammar")
}

fn parse(grammar_text: &str) -> tiexiu::trees::Tree {
    let grammar = compile(grammar_text);
    parse_with_boot(&grammar, grammar_text)
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
// Boot Grammar Structure Tests
// =============================================================================

mod structure {
    use super::*;
    use fixtures::all_rules_linked;

    #[test]
    fn has_required_rules() {
        let boot = boot_grammar();

        let required = [
            "start",
            "grammar",
            "rule",
            "expre",
            "choice",
            "sequence",
            "option",
            "element",
            "term",
            "atom",
            "call",
            "named",
            "named_single",
            "named_list",
            "optional",
            "closure",
            "positive_closure",
            "lookahead",
            "negative_lookahead",
            "token",
            "pattern",
            "regex",
            "constant",
            "eof",
            "cut",
        ];

        for name in required {
            assert!(
                boot.get_rule_id(name).is_ok(),
                "Missing required rule: {}",
                name
            );
        }
    }

    #[test]
    fn all_rules_are_linked() {
        let boot = boot_grammar();
        let issues = all_rules_linked(&boot);
        assert!(
            issues.is_empty(),
            "Boot grammar has unlinked calls:\n{}",
            issues.join("\n")
        );
    }
}

// =============================================================================
// Boot Grammar Parsing Tests - Grammar Syntax
// =============================================================================

mod parse_grammar {
    use super::*;

    #[test]
    #[ignore]
    fn simple_grammar() {
        let tree = tiexiu::api::parse(
            r#"
            @@grammar :: Simple
            start: 'hello'
        "#,
        )
        .expect("Failed to parse");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        assert!(tree.to_json_string().unwrap().contains("grammar"));
    }

    #[test]
    #[ignore]
    fn multiple_rules() {
        let boot = boot_grammar();
        let grammar = r#"
            @@grammar :: Multi
            start: a | b | c
            a: 'a'
            b: 'b'
            c: 'c'
        "#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("rules"));
    }

    #[test]
    #[ignore]
    fn directive() {
        let boot = boot_grammar();
        let grammar = r#"
            @@grammar :: Directives
            @@whitespace :: /\s+/
            start: 'hello'
        "#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("directive"));
    }
}

// =============================================================================
// Boot Grammar Parsing Tests - Expression Types
// =============================================================================

mod parse_expressions {
    use super::*;

    #[test]
    #[ignore]
    fn token() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: T start: 'foo' 'bar'"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Token"));
    }

    #[test]
    #[ignore]
    fn pattern() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: P start: /\d+/"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Pattern"));
    }

    #[test]
    #[ignore]
    fn sequence() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: S start: 'a' 'b' 'c'"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Sequence"));
    }

    #[test]
    #[ignore]
    fn choice() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: C start: 'a' | 'b' | 'c'"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Choice"));
    }

    #[test]
    #[ignore]
    fn optional() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: O start: 'a' 'b'? 'c'"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Optional"));
    }

    #[test]
    #[ignore]
    fn closure() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: Cl start: 'a'*"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Closure"));
    }

    #[test]
    #[ignore]
    fn positive_closure() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: PC start: 'a'+"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("PositiveClosure"));
    }

    #[test]
    #[ignore]
    fn group() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: G start: ('a' 'b')*"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Group"));
    }
}

// =============================================================================
// Boot Grammar Parsing Tests - Lookahead and Constraints
// =============================================================================

mod parse_constraints {
    use super::*;

    #[test]
    #[ignore]
    fn lookahead() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: L start: &'a' 'a'"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Lookahead"));
    }

    #[test]
    #[ignore]
    fn negative_lookahead() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: NL start: !'b' 'a'"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("NegativeLookahead"));
    }

    #[test]
    #[ignore]
    fn cut() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: Cu start: 'a' ~ 'b'"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Cut"));
    }
}

// =============================================================================
// Boot Grammar Parsing Tests - Naming and References
// =============================================================================

mod parse_naming {
    use super::*;

    #[test]
    #[ignore]
    fn named() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: N start: name='a'"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Named"));
        assert!(json.contains("name"));
    }

    #[test]
    #[ignore]
    fn rule_call() {
        let boot = boot_grammar();
        let grammar = r#"
            @@grammar :: RC
            start: foo
            foo: 'x'
        "#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Call"));
    }

    #[test]
    #[ignore]
    fn rule_include() {
        let boot = boot_grammar();
        let grammar = r#"
            @@grammar :: RI
            start: >base
            base: 'x'
        "#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("RuleInclude"));
    }

    #[test]
    #[ignore]
    fn rule_with_params() {
        let boot = boot_grammar();
        let grammar = r#"
            @@grammar :: RWP
            start: foo[bar]
            foo[param]: 'x' param
        "#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("params"));
    }
}

// =============================================================================
// Boot Grammar Parsing Tests - Special Forms
// =============================================================================

mod parse_special {
    use super::*;

    #[test]
    #[ignore]
    fn void() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: V start: 'a' () 'b'"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Void"));
    }

    #[test]
    #[ignore]
    fn eof() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: E start: 'a' $"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("EOF"));
    }

    #[test]
    #[ignore]
    fn dot() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: D start: /./"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Dot"));
    }

    #[test]
    #[ignore]
    fn constant() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: Cst start: `constant`"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Constant"));
    }

    #[test]
    #[ignore]
    fn join() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: J start: ','%{'a'}*"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Join"));
    }

    #[test]
    #[ignore]
    fn gather() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: Gt start: ','.{'a'}*"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Gather"));
    }

    #[test]
    #[ignore]
    fn skip_group() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: Sk start: (?: 'a' 'b')*"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("SkipGroup"));
    }

    #[test]
    #[ignore]
    fn alert() {
        let boot = boot_grammar();
        let grammar = r#"@@grammar :: Al start: ^^`danger`"#;
        let tree = parse_ebnf(&boot, grammar);
        let json = tree.to_json_string().unwrap();
        assert!(json.contains("Alert"));
    }
}

// =============================================================================
// Integration Tests
// =============================================================================

mod integration {
    use super::*;

    #[test]
    #[ignore]
    fn complex_grammar() {
        let boot = boot_grammar();
        let grammar = r#"
            @@grammar :: Complex
            @@whitespace :: /\s+/

            start: expression

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
        let tree = parse_ebnf(&boot, grammar);
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn tatsu_own_grammar() {
        let boot = boot_grammar();
        let tatsu_grammar = std::fs::read_to_string("grammar/tatsu.tatsu")
            .expect("Failed to read grammar/tatsu.tatsu");

        let tree = parse_ebnf(&boot, &tatsu_grammar);
        let json = tree.to_json_string().unwrap();

        assert!(json.contains("Grammar"));
        assert!(json.contains("rules"));
    }
}

// =============================================================================
// Compilation Tests
// =============================================================================

mod compilation {
    use super::*;
    use fixtures::all_rules_linked;

    #[test]
    #[ignore]
    fn simple_grammar_links() {
        let grammar = compile(
            r#"
            @@grammar :: Test
            start: 'hello' 'world'
        "#,
        );

        let issues = all_rules_linked(&grammar);
        assert!(
            issues.is_empty(),
            "Compiled grammar has unlinked calls:\n{}",
            issues.join("\n")
        );
    }

    #[test]
    #[ignore]
    fn compiled_grammar_parses_input() {
        let grammar = compile(
            r#"
            @@grammar :: Test
            start: 'hello' 'world'
        "#,
        );

        let tree = parse_input(&grammar, "hello world");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }
}

// =============================================================================
// Round-Trip Tests
// =============================================================================

mod round_trips {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    #[ignore]
    fn parse_round_trip() {
        let grammar_text = r#"
            @@grammar :: RoundTrip
            start: expression
            expression: term (('+' | '-') term)*
            term: factor (('*' | '/') factor)*
            factor: NUMBER | '(' expression ')'
            NUMBER: /\d+/
        "#;

        let tree1 = parse(grammar_text);
        let grammar = compile(grammar_text);
        let tree2 = parse_with_boot(&grammar, grammar_text);

        let json1 = tree1.to_json_string().expect("Serialize tree1");
        let json2 = tree2.to_json_string().expect("Serialize tree2");

        let mut h1 = DefaultHasher::new();
        json1.hash(&mut h1);
        let mut h2 = DefaultHasher::new();
        json2.hash(&mut h2);

        assert_eq!(
            h1.finish(),
            h2.finish(),
            "Round-trip produced different trees"
        );
    }

    #[test]
    #[ignore]
    fn pretty_print_round_trip() {
        let grammar_text = r#"
            @@grammar :: Pretty
            start: 'a' | 'b'
        "#;

        let grammar = compile(grammar_text);
        let pretty = grammar.to_string();

        let grammar2 = compile(&pretty);
        let pretty2 = grammar2.to_string();

        assert_eq!(
            pretty.trim(),
            pretty2.trim(),
            "Pretty-print round-trip failed:\nOriginal:\n{}\nPretty:\n{}",
            pretty,
            pretty2
        );
    }
}
