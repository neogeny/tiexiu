// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::input::Cursor;
use crate::input::StrCursor;
use crate::json::boot::boot_grammar;
use crate::peg::{Grammar, Succ};
use crate::state::corectx::CoreCtx;
use crate::trees::Tree;
use crate::util::indent::unindent;
use crate::{Error, Result};

pub fn parse(grammar: &str) -> Result<Tree> {
    // unindent in case the str was inlined
    let trimmed = unindent(grammar);
    parse_with(StrCursor::new(trimmed.as_str()))
}

pub fn parse_with<U>(cursor: U) -> Result<Tree>
where
    U: Cursor + Clone,
{
    let boot = boot_grammar()?;
    let ctx = CoreCtx::new(cursor);

    match boot.parse(ctx) {
        Ok(Succ(_, tree)) => Ok(tree),
        Err(failure) => Err(failure.into()),
    }
}

pub fn compile(grammar: &str) -> Result<Grammar> {
    // unindent in case the str was inlined
    let trimmed = unindent(grammar);
    compile_with(StrCursor::new(trimmed.as_str()))
}

pub fn compile_with<U>(cursor: U) -> Result<Grammar>
where
    U: Cursor + Clone,
{
    let tree = parse_with(cursor)?;
    Ok(Grammar::compile(&tree)?)
}

pub fn load(json: &str) -> Result<Grammar> {
    Ok(Grammar::from_json(json)?)
}

pub fn load_tree(json: &str) -> Result<Tree> {
    Tree::from_json_str(json).map_err(Error::from)
}

pub fn pretty(grammar: &str) -> Result<String> {
    let grammar = compile(grammar)?;
    Ok(grammar.to_string())
}

pub fn pretty_tree(tree: &Tree) -> Result<String> {
    Ok(tree.to_json_string()?)
}

pub fn load_boot() -> Result<Grammar> {
    Ok(boot_grammar()?)
}

pub fn boot_grammar_json() -> Result<String> {
    let boot = boot_grammar()?;
    let model: crate::json::tatsu::TatSuModel = boot.clone().try_into()?;
    Ok(serde_json::to_string_pretty(&model)?)
}

pub fn boot_grammar_pretty() -> Result<String> {
    let boot = boot_grammar()?;
    Ok(boot.to_string())
}

pub fn parse_input(grammar: &Grammar, input: &str) -> Result<Tree> {
    let cursor = StrCursor::new(input);
    let ctx = CoreCtx::new(cursor);

    match grammar.parse(ctx) {
        Ok(Succ(_, tree)) => Ok(tree),
        Err(failure) => Err(failure.into()),
    }
}
