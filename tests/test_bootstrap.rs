//! Bootstrap tests for TieXiu
//!
//! Tests for parsing grammars in TatSu EBNF format.
//! Run with: cargo test --features bootstrap
//!

mod fixtures;

// =============================================================================
// Boot Grammar Parsing Tests - Grammar Syntax
// =============================================================================

mod parse_grammar {
    use tiexiu::api::parse_grammar;

    #[test]
    fn simple_grammar() {
        let tree = parse_grammar(
            r#"
            @@grammar :: Simple
            start: 'hello'
        "#,
            &[],
        )
        .expect("Failed to parse");
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
        assert!(tree.to_model_json_string().unwrap().contains("grammar"));
    }

    #[test]
    fn multiple_rules() {
        let grammar = r#"
            @@grammar :: Multi
            start: a | b | c
            a: 'a'
            b: 'b'
            c: 'c'
        "#;
        let tree = parse_grammar(grammar, &[]).expect("a tree");
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("rules"));
    }

    #[test]
    fn directive() {
        let grammar = r#"
            @@grammar :: Directives
            @@whitespace :: /\s+/
            start: 'hello'
        "#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("directive"));
    }
}

// =============================================================================
// Boot Grammar Parsing Tests - Expression Types
// =============================================================================

mod parse_expressions {
    use tiexiu::api::*;

    #[test]
    fn token() {
        let grammar = r#"@@grammar :: T start: 'foo' 'bar'"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Token"));
    }

    #[test]
    fn pattern() {
        let grammar = r#"@@grammar :: P start: /\d+/"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Pattern"));
    }

    #[test]
    fn sequence() {
        let grammar = r#"@@grammar :: S start: 'a' 'b' 'c'"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Sequence"));
    }

    #[test]
    fn choice() {
        let grammar = r#"@@grammar :: C start: 'a' | 'b' | 'c'"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Choice"));
    }

    #[test]
    fn optional() {
        let grammar = r#"@@grammar :: O start: 'a' 'b'? 'c'"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Optional"));
    }

    #[test]
    fn closure() {
        let grammar = r#"@@grammar :: Cl start: 'a'*"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Closure"));
    }

    #[test]
    fn positive_closure() {
        let grammar = r#"@@grammar :: PC start: 'a'+"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("PositiveClosure"));
    }

    #[test]
    fn group() {
        let grammar = r#"@@grammar :: G start: ('a' 'b')*"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Group"));
    }
}

// =============================================================================
// Boot Grammar Parsing Tests - Lookahead and Constraints
// =============================================================================

mod parse_constraints {
    use tiexiu::parse_grammar;

    #[test]
    fn lookahead() {
        let grammar = r#"@@grammar :: L start: &'a' 'a'"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Lookahead"));
    }

    #[test]
    fn negative_lookahead() {
        let grammar = r#"@@grammar :: NL start: !'b' 'a'"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("NegativeLookahead"));
    }

    #[test]
    fn cut() {
        let grammar = r#"@@grammar :: Cu start: 'a' ~ 'b'"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Cut"));
    }
}

// =============================================================================
// Boot Grammar Parsing Tests - Naming and References
// =============================================================================

mod parse_naming {
    use tiexiu::*;

    #[test]
    fn named() {
        let grammar = r#"@@grammar :: N start: name='a'"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Named"));
        assert!(json.contains("name"));
    }

    #[test]
    fn rule_call() {
        let grammar = r#"
            @@grammar :: RC
            start: foo
            foo: 'x'
        "#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Call"));
    }

    #[test]
    fn rule_include() {
        let grammar = r#"
            @@grammar :: RI
            start: >base
            base: 'x'
        "#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("RuleInclude"));
    }

    #[test]
    fn rule_with_params() {
        let grammar = r#"
            @@grammar :: RWP

            start: foo ;

            foo[Foo]: 'x' param ;
        "#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Foo"));
    }
}

// =============================================================================
// Boot Grammar Parsing Tests - Special Forms
// =============================================================================

mod parse_special {
    use tiexiu::*;

    #[test]
    fn void() {
        let grammar = r#"@@grammar :: V start: 'a' () 'b'"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Void"));
    }

    #[test]
    fn eof() {
        let grammar = r#"@@grammar :: E start: 'a' $"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("EOF"));
    }

    #[test]
    fn dot() {
        let grammar = r#"@@grammar :: D start: /./"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        eprintln!("{:#}", json);
        assert!(json.contains("Dot"));
    }

    #[test]
    fn constant() {
        let grammar = r#"@@grammar :: Cst start: `constant`"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Constant"));
    }

    #[test]
    fn join() {
        let grammar = r#"@@grammar :: J start: ','%{'a'}*"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Join"));
    }

    #[test]
    fn gather() {
        let grammar = r#"@@grammar :: Gt start: ','.{'a'}*"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Gather"));
    }

    #[test]
    fn skip_group() {
        let grammar = r#"@@grammar :: Sk start: (?: 'a' 'b')*"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("SkipGroup"));
    }

    #[test]
    fn alert() {
        let grammar = r#"@@grammar :: Al start: ^^`danger`"#;
        let tree = parse_grammar(grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();
        assert!(json.contains("Alert"));
    }
}

// =============================================================================
// Integration Tests
// =============================================================================

mod integration {
    use tiexiu::cfg::constants::PATH_TATSU_GRAMMAR_EBNF;
    use tiexiu::parse_grammar;

    #[test]
    fn complex_grammar() {
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
        let tree = parse_grammar(grammar, &[]).unwrap();
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }

    #[test]
    #[ignore]
    fn tatsu_own_grammar() {
        let tatsu_grammar =
            std::fs::read_to_string(PATH_TATSU_GRAMMAR_EBNF).expect("Failed to read grammar");

        let tree = parse_grammar(&tatsu_grammar, &[]).unwrap();
        let json = tree.to_model_json_string().unwrap();

        assert!(json.contains("Grammar"));
        assert!(json.contains("rules"));
    }
}

// =============================================================================
// Compilation Tests
// =============================================================================

mod compilation {
    use tiexiu::parse_input;

    #[test]
    #[ignore]
    fn compiled_grammar_parses_input() {
        let grammar = tiexiu::compile(
            r#"
            @@grammar :: Test
            start: 'hello' 'world'
        "#,
            &[],
        )
        .unwrap();

        let tree = parse_input(&grammar, "hello world", &[]).unwrap();
        assert!(matches!(tree, tiexiu::trees::Tree::Node { .. }));
    }
}

// =============================================================================
// Round-Trip Tests
// =============================================================================

mod round_trips {
    use tiexiu::peg::pretty::*;
    use tiexiu::*;

    #[test]
    fn pretty_print_roundtrip() {
        let grammar_text = r#"
            @@grammar :: Pretty
            start: 'a' | 'b'
        "#;
        let tree = parse_grammar(grammar_text, &[]).unwrap();
        eprintln!("TREE\n{:#?}", tree);

        let grammar = compile(grammar_text, &[]).unwrap();
        eprintln!("COMPILED\n{}", grammar);

        let pretty = grammar.pretty_print();
        eprintln!("COMPILED->PRETTY\n{:#}", pretty);

        let grammar2 = compile(&pretty, &[]).unwrap();
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
