// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::input::Cursor;
use crate::input::StrCursor;
use crate::json::{ToJson, boot_grammar};
use crate::peg::{Grammar, Succ};
use crate::state::corectx::CoreCtx;
use crate::trees::Tree;
use crate::{Error, Result};

pub fn parse_grammar(grammar: &str) -> Result<Tree> {
    parse_grammar_with(StrCursor::new(grammar))
}

pub fn parse_grammar_as_json(grammar: &str) -> Result<String> {
    let tree = parse_grammar(grammar)?;
    tree.to_json_string().map_err(Error::from)
}

pub fn parse_grammar_with<U>(cursor: U) -> Result<Tree>
where
    U: Cursor + Clone,
{
    let boot = boot_grammar()?;
    let mut ctx = CoreCtx::new(cursor);
    ctx.set_trace(true);

    match boot.parse(ctx) {
        Ok(Succ(_, tree)) => Ok(tree),
        Err(failure) => Err(failure.into()),
    }
}

pub fn parse_grammar_with_as_json<U>(cursor: U) -> Result<String>
where
    U: Cursor + Clone,
{
    let tree = parse_grammar_with(cursor)?;
    tree.to_json_string().map_err(Error::from)
}

pub fn compile(grammar: &str) -> Result<Grammar> {
    compile_with(StrCursor::new(grammar))
}

pub fn compile_to_json(grammar: &str) -> Result<String> {
    let compiled = compile(grammar)?;
    compiled.to_json_string().map_err(Error::from)
}

pub fn compile_with<U>(cursor: U) -> Result<Grammar>
where
    U: Cursor + Clone,
{
    let tree = parse_grammar_with(cursor)?;
    Ok(Grammar::compile(&tree)?)
}

pub fn compile_with_as_json<U>(cursor: U) -> Result<String>
where
    U: Cursor + Clone,
{
    let compiled = compile_with(cursor)?;
    compiled.to_json_string().map_err(Error::from)
}

pub fn load(json: &str) -> Result<Grammar> {
    Ok(Grammar::serde_from_json(json)?)
}

pub fn load_as_json(json: &str) -> Result<String> {
    let grammar = load(json)?;
    grammar.to_json_string().map_err(Error::from)
}

pub fn load_tree(json: &str) -> Result<Tree> {
    Tree::from_serde_json_str(json).map_err(Error::from)
}

pub fn load_tree_as_json(json: &str) -> Result<String> {
    let tree = load_tree(json)?;
    tree.to_json_string().map_err(Error::from)
}

pub fn pretty(grammar: &str) -> Result<String> {
    let grammar = compile(grammar)?;
    Ok(grammar.to_string())
}

pub fn pretty_tree(tree: &Tree) -> Result<String> {
    Ok(tree.to_json_string()?)
}

pub fn pretty_tree_json(tree: &Tree) -> Result<String> {
    tree.to_json_string().map_err(Error::from)
}

pub fn load_boot() -> Result<Grammar> {
    Ok(boot_grammar()?)
}

pub fn load_boot_as_json() -> Result<String> {
    let grammar = load_boot()?;
    grammar.to_json_string().map_err(Error::from)
}

pub fn boot_grammar_json() -> Result<String> {
    let boot = boot_grammar()?;
    match boot.to_json_string() {
        Ok(s) => Ok(s),
        Err(e) => Err(e.into()),
    }
}

pub fn boot_grammar_pretty() -> Result<String> {
    let boot = boot_grammar()?;
    Ok(boot.to_string())
}

pub fn parse_input(parser: &Grammar, text: &str) -> Result<Tree> {
    let cursor = StrCursor::new(text);
    let ctx = CoreCtx::new(cursor);

    match parser.parse(ctx) {
        Ok(Succ(_, tree)) => Ok(tree),
        Err(failure) => Err(failure.into()),
    }
}

pub fn parse_input_to_json(parser: &Grammar, text: &str) -> Result<String> {
    let tree = parse_input(parser, text)?;
    tree.to_json_string().map_err(Error::from)
}

pub fn parse(grammar: &str, text: &str) -> Result<Tree> {
    let parser = compile(grammar)?;
    parse_input(&parser, text)
}

pub fn parse_to_json(grammar: &str, text: &str) -> Result<String> {
    let parser = compile(grammar)?;
    parse_input_to_json(&parser, text)
}
