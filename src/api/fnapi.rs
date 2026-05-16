// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::CfgA;
use crate::context::new_ctx;
use crate::input::{Cursor, StrCursor};
use crate::peg::grammar::PrettyPrint;
use crate::peg::*;
use crate::trees::Tree;
use crate::{Error, Result};

use crate::api::ooapi::TieXiu;

/// Create a default `TieXiu` instance with empty config.
pub fn pegapi() -> TieXiu {
    TieXiu::new(&[])
}

/// Parse a grammar string into a parse tree.
pub fn parse_grammar(grammar: &str, cfg: &CfgA) -> Result<Tree> {
    parse_grammar_with(StrCursor::new(grammar), cfg)
}

/// Parse a grammar string into a JSON value.
pub fn parse_grammar_to_json(grammar: &str, cfg: &CfgA) -> Result<json::JsonValue> {
    let tree = parse_grammar(grammar, cfg)?;
    Ok(tree.to_json())
}

/// Parse a grammar string into a JSON string.
pub fn parse_grammar_to_json_string(grammar: &str, cfg: &CfgA) -> Result<String> {
    let tree = parse_grammar(grammar, cfg)?;
    Ok(tree.to_json_string())
}

/// Parse grammar from a generic cursor source.
pub fn parse_grammar_with<U>(cursor: U, cfg: &CfgA) -> Result<Tree>
where
    U: Cursor + Clone,
{
    let boot = boot_grammar()?;
    let ctx = new_ctx(cursor, cfg);
    boot.parse_tree(ctx)
}

/// Parse grammar from cursor and return as JSON value.
pub fn parse_grammar_to_json_with<U>(cursor: U, cfg: &CfgA) -> Result<json::JsonValue>
where
    U: Cursor + Clone,
{
    let tree = parse_grammar_with(cursor, cfg)?;
    Ok(tree.to_json())
}

/// Compile a grammar string into a `Grammar`.
pub fn compile(grammar: &str, cfg: &CfgA) -> Result<Grammar> {
    compile_with(StrCursor::new(grammar), cfg)
}

/// Compile grammar and return as JSON value.
pub fn compile_to_json(grammar: &str, cfg: &CfgA) -> Result<json::JsonValue> {
    let compiled = compile(grammar, cfg)?;
    Ok(compiled.to_json())
}

/// Compile grammar and return as JSON string.
pub fn compile_to_json_string(grammar: &str, cfg: &CfgA) -> Result<String> {
    let compiled = compile(grammar, cfg)?;
    Ok(compiled.to_json_string()?)
}

/// Compile grammar from a generic cursor source.
pub fn compile_with<U>(cursor: U, cfg: &CfgA) -> Result<Grammar>
where
    U: Cursor + Clone,
{
    let tree = parse_grammar_with(cursor, cfg)?;
    Ok(Grammar::compile(&tree, cfg)?)
}

/// Compile from cursor and return as JSON value.
pub fn compile_to_json_with<U>(cursor: U, cfg: &CfgA) -> Result<json::JsonValue>
where
    U: Cursor + Clone,
{
    let compiled = compile_with(cursor, cfg)?;
    Ok(compiled.to_json())
}

/// Load a grammar from a JSON string.
pub fn load_grammar_from_json(json: &str, _cfg: &CfgA) -> Result<Grammar> {
    Ok(Grammar::from_json(json)?)
}

/// Load grammar from JSON and re-serialize as JSON value.
pub fn load_grammar_from_json_to_json(json: &str, cfg: &CfgA) -> Result<json::JsonValue> {
    let grammar = load_grammar_from_json(json, cfg)?;
    Ok(grammar.to_json())
}

/// Load a parse tree from a JSON string.
pub fn load_tree_from_json(json: &str, _cfg: &CfgA) -> Result<Tree> {
    Tree::from_json_str(json).map_err(Error::from)
}

/// Load a tree from JSON and re-serialize as JSON value.
pub fn load_tree_to_json(json: &str, cfg: &CfgA) -> Result<json::JsonValue> {
    let tree = load_tree_from_json(json, cfg)?;
    Ok(tree.to_json())
}

/// Pretty-print a grammar string.
pub fn grammar_pretty(grammar: &str, cfg: &CfgA) -> Result<String> {
    let grammar = compile(grammar, cfg)?;
    Ok(grammar.pretty_print())
}

/// Parse input text against a grammar string.
pub fn parse(grammar: &str, text: &str, cfg: &CfgA) -> Result<Tree> {
    let parser = compile(grammar, cfg)?;
    parse_input(&parser, text, cfg)
}

/// Parse input text and return result as JSON value.
pub fn parse_to_json(grammar: &str, text: &str, cfg: &CfgA) -> Result<json::JsonValue> {
    let parser = compile(grammar, cfg)?;
    parse_input_to_json(&parser, text, cfg)
}
/// Parse input text and return result as JSON string.
pub fn parse_to_json_string(grammar: &str, text: &str, cfg: &CfgA) -> Result<String> {
    let parser = compile(grammar, cfg)?;
    parse_input_to_json_string(&parser, text, cfg)
}

/// Parse input text with a pre-compiled `Grammar`.
pub fn parse_input(parser: &Grammar, text: &str, cfg: &CfgA) -> Result<Tree> {
    parser.parse_input(text, cfg)
}

/// Parse input with pre-compiled grammar and return JSON value.
pub fn parse_input_to_json(parser: &Grammar, text: &str, cfg: &CfgA) -> Result<json::JsonValue> {
    let tree = parse_input(parser, text, cfg)?;
    Ok(tree.to_json())
}

/// Parse input with pre-compiled grammar and return JSON string.
pub fn parse_input_to_json_string(parser: &Grammar, text: &str, cfg: &CfgA) -> Result<String> {
    let tree = parse_input(parser, text, cfg)?;
    Ok(tree.to_json_string())
}

/// Load the boot grammar.
pub fn boot_grammar() -> Result<Grammar> {
    Ok(crate::json::boot::boot_grammar()?)
}

/// Alias for `boot_grammar`.
pub fn load_boot(_cfg: &CfgA) -> Result<Grammar> {
    boot_grammar()
}

/// Return the boot grammar as a JSON value.
pub fn boot_grammar_to_json(cfg: &CfgA) -> Result<json::JsonValue> {
    let grammar = load_boot(cfg)?;
    Ok(grammar.to_json())
}

/// Return the boot grammar as a JSON string.
pub fn boot_grammar_to_json_string(cfg: &CfgA) -> Result<String> {
    let grammar = load_boot(cfg)?;
    grammar.to_json_string().map_err(Error::from)
}

/// Pretty-print the boot grammar.
pub fn boot_grammar_pretty(_cfg: &CfgA) -> Result<String> {
    let boot = boot_grammar()?;
    Ok(boot.pretty_print())
}
