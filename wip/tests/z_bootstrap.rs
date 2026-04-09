// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's z_bootstrap_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.
//!
//! Note: The bootstrap tests are the most important tests for TieXiu,
//! as they verify that TieXiu can parse its own grammar definition.

use crate::api::{boot_grammar, compile, parse, pretty};

use crate::api::{boot_grammar, boot_grammar_json, boot_grammar_pretty, compile, parse, pretty};

#[test]
fn test_00_with_bootstrap_grammar() {
    let text = crate::api::tatsu_grammar();
    let g = crate::api::TatSuParser::new("Bootstrap");
    let grammar0 = g.parse(text, &mut ASTSemantics::new(), false);
    assert!(grammar0.is_ok());
}

#[test]
fn test_01_with_parser_generator() {
    let text = crate::api::tatsu_grammar();
    let g = TatSuParserGenerator::new("TatSuBootstrap");
    let result = g.parse(text);
    assert!(result.is_ok());
}

#[test]
fn test_02_previous_output_generator() {
    let text = std::fs::read_to_string("./tmp/01.tatsu").unwrap_or_default();
    let g = TatSuParserGenerator::new("TatSuBootstrap");
    let result = g.parse(&text);
    assert!(result.is_ok());
}

#[test]
fn test_03_repeat_02() {
    let text = std::fs::read_to_string("./tmp/02.tatsu").unwrap_or_default();
    let g = TatSuParser::new("TatSuBootstrap");
    let ast3 = g.parse(&text);
    assert!(ast3.is_ok());
}

#[test]
fn test_04_repeat_03() {
    let text = std::fs::read_to_string("./tmp/01.tatsu").unwrap_or_default();
    let g = TatSuParserGenerator::new("TatSuBootstrap");
    let result = g.parse(&text);
    assert!(result.is_ok());
}

#[test]
fn test_05_parse_with_model() {
    let text = std::fs::read_to_string("./tmp/02.tatsu").unwrap_or_default();
    let g = TatSuParserGenerator::new("TatSuBootstrap");
    let parser = g.parse(&text);
    assert!(parser.is_ok());
}

#[test]
fn test_06_generate_code() {
    let grammar = r#"
        @@grammar::Test
        start = 'hello' $ ;
    "#;
    let model = compile(grammar, "Test").expect("Failed to compile");
    let code = crate::codegen::generate(&model);
    assert!(code.is_ok());
}

#[test]
fn test_07_import_generated_code() {
    let grammar = r#"
        @@grammar::Test
        start = 'hello' $ ;
    "#;
    let model = compile(grammar, "Test").expect("Failed to compile");
    let code = crate::codegen::generate(&model).expect("Failed to generate");
    let parsed = parse(&code, "");
    assert!(parsed.is_ok());
}

#[test]
fn test_08_compile_with_generated() {
    let grammar = r#"
        @@grammar::Test
        start = 'hello' $ ;
    "#;
    let model = compile(grammar, "Test").expect("Failed to compile");
    let code = crate::codegen::generate(&model).expect("Failed to generate");
    let compiled = compile(&code, "Generated");
    assert!(compiled.is_ok());
}

#[test]
fn test_09_parser_with_semantics() {
    let grammar = r#"
        start::sum = {number}+ $ ;
        number::int = /\d+/ ;
    "#;
    let model = compile(grammar, "Test").expect("Failed to compile");
    let mut semantics = crate::semantics::ModelBuilderSemantics::new();
    let ast = model.parse("5 4 3 2 1", &mut semantics);
    assert!(ast.is_ok());
}

#[test]
fn test_10_with_model_and_semantics() {
    let grammar = r#"
        start::sum = {number}+ $ ;
        number::int = /\d+/ ;
    "#;
    let model = crate::api::asmodel(grammar).expect("Failed to create model");
    let mut semantics = crate::semantics::ModelBuilderSemantics::new();
    let ast = model.parse("5 4 3 2 1", &mut semantics);
    assert!(ast.is_ok());
}

#[test]
fn test_11_with_pickle_and_retry() {
    use std::sync::Arc;

    let grammar = r#"
        start = 'hello' $ ;
    "#;
    let model = compile(grammar, "Test").expect("Failed to compile");
    let serialized = serde_json::to_string(&model).expect("Failed to serialize");
    let deserialized: crate::model::Grammar =
        serde_json::from_str(&serialized).expect("Failed to deserialize");
    let result = deserialized.parse("hello");
    assert!(result.is_ok());
}

#[test]
fn test_12_walker() {
    let grammar = r#"
        start = {item}+ $ ;
        item = /\w+/ ;
    "#;
    let model = compile(grammar, "Test").expect("Failed to compile");
    let ast = model.parse("hello world").expect("Failed to parse");
    let walker = crate::walker::PrintWalker::new();
    let output = walker.walk(&ast);
    assert!(!output.is_empty());
}

#[test]
fn test_13_diagram() {
    let grammar = r#"
        @@grammar::Test
        start = 'hello' 'world' ;
    "#;
    let model = compile(grammar, "Test").expect("Failed to compile");
    let diagram = crate::diagram::to_diagram(&model);
    assert!(diagram.is_ok());
}

// ============================================================================
// Simplified Bootstrap Test
// ============================================================================

#[test]
fn test_bootstrap_loads_tatsu_grammar() {
    let boot = boot_grammar().expect("Failed to load bootstrap grammar");
    assert!(!boot.to_string().is_empty());
}

#[test]
fn test_bootstrap_pretty_print() {
    let pretty = boot_grammar_pretty().expect("Failed to get pretty grammar");
    assert!(!pretty.is_empty());
    assert!(pretty.contains("start"));
}

#[test]
fn test_bootstrap_json_roundtrip() {
    let json = boot_grammar_json().expect("Failed to get grammar JSON");
    assert!(!json.is_empty());
    assert!(json.contains("rules"));
}
