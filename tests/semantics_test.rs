// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for semantics actions — validates the Semantics trait pipeline
//! by intercepting Meta nodes during boot-grammar parsing and converting
//! them to strongly-typed NameMeta/IntMeta/etc. before the compiler sees them.

#[macro_use]
extern crate json;
use std::sync::Arc;
use tiexiu::Str;
use tiexiu::cfg::CfgKey;
use tiexiu::context::{Semantics, SemanticsRef};
use tiexiu::peg::error::ParseResult;
use tiexiu::trees::{Tree, TreeRef};
use tiexiu::{Result, parse};

/// A Semantics implementation that converts generic `Meta` nodes
/// (produced by the @name/@int/@uint/@float/@bool boot-grammar rules)
/// into strongly-typed `NameMeta`, `IntMeta`, etc. nodes.
///
/// The compiler has parallel cases for both `Meta` and `NameMeta`/etc.,
/// so this can be used to test the semantics pipeline end-to-end.
#[derive(Debug)]
struct MetaToNameMeta;

impl Semantics for MetaToNameMeta {
    fn apply(&self, node: TreeRef, _rule_name: &str, _params: &[Str]) -> ParseResult {
        if let Tree::Node { typename, tree } = node.as_ref()
            && typename.as_ref() == "Meta"
        {
            let text = tree.value();
            let new_typename: Str = match text.as_ref() {
                "name" => "NameMeta",
                "int" => "IntMeta",
                "uint" => "UIntMeta",
                "float" => "FloatMeta",
                "bool" => "BoolMeta",
                _ => return Ok(Tree::Bottom.into()),
            }
            .into();
            return Ok(Tree::Node {
                typename: new_typename,
                tree: tree.clone(),
            }
            .into());
        }
        Ok(Tree::Bottom.into())
    }
}

#[test]
fn test_semantics_meta_to_name() -> Result<()> {
    let sem: SemanticsRef = Arc::new(MetaToNameMeta);
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @name $ ;
    "#;
    let tree = parse(grammar, "hello_world", &[CfgKey::Semantics(sem.clone())])?;
    assert_eq!(tree.to_json(), value!("hello_world"));
    Ok(())
}

#[test]
fn test_semantics_meta_to_int() -> Result<()> {
    let sem: SemanticsRef = Arc::new(MetaToNameMeta);
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @int $ ;
    "#;
    let tree = parse(grammar, "42", &[CfgKey::Semantics(sem.clone())])?;
    assert_eq!(tree.to_json(), value!("42"));
    Ok(())
}

#[test]
fn test_semantics_meta_to_float() -> Result<()> {
    let sem: SemanticsRef = Arc::new(MetaToNameMeta);
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @float $ ;
    "#;
    let tree = parse(grammar, "3.14", &[CfgKey::Semantics(sem.clone())])?;
    assert_eq!(tree.to_json(), value!("3.14"));
    Ok(())
}

#[test]
fn test_semantics_meta_to_bool() -> Result<()> {
    let sem: SemanticsRef = Arc::new(MetaToNameMeta);
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @bool $ ;
    "#;
    let tree = parse(grammar, "true", &[CfgKey::Semantics(sem.clone())])?;
    assert_eq!(tree.to_json(), value!("true"));
    Ok(())
}

#[test]
fn test_semantics_meta_to_uint() -> Result<()> {
    let sem: SemanticsRef = Arc::new(MetaToNameMeta);
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @uint $ ;
    "#;
    let tree = parse(grammar, "007", &[CfgKey::Semantics(sem.clone())])?;
    assert_eq!(tree.to_json(), value!("7"));
    Ok(())
}

#[test]
fn test_semantics_without_semantics_falls_through() -> Result<()> {
    // Without semantics, the compiler's `Meta` case handles the nodes directly.
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = @name $ ;
    "#;
    let tree = parse(grammar, "hello_world", &[])?;
    assert_eq!(tree.to_json(), value!("hello_world"));
    Ok(())
}

#[test]
fn test_semantics_passes_through_other_rules() -> Result<()> {
    // Semantics only intercepts `Meta` nodes; other rules pass through.
    let sem: SemanticsRef = Arc::new(MetaToNameMeta);
    let grammar = r#"
        @@whitespace :: /[\t ]+/
        start = word $ ;
        word = /\w+/ ;
    "#;
    let tree = parse(grammar, "hello", &[CfgKey::Semantics(sem.clone())])?;
    assert_eq!(tree.to_json(), value!("hello"));
    Ok(())
}
