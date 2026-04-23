// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub use crate::cfg::*;
pub use crate::engine::new_ctx;
pub use crate::input::{Cursor, StrCursor};
pub use crate::json::ToExpJson;
pub use crate::peg::grammar::PrettyPrint;
pub use crate::peg::*;
pub use crate::trees::Tree;
pub use crate::util;
pub use crate::{Error, Result};

pub(crate) fn boot_grammar() -> Result<Grammar> {
    Ok(crate::json::boot::boot_grammar()?)
}

pub fn parse_grammar(grammar: &str, cfg: &CfgA) -> Result<Tree> {
    parse_grammar_with(StrCursor::new(grammar), cfg)
}

pub fn parse_grammar_as_json(grammar: &str, cfg: &CfgA) -> Result<String> {
    let tree = parse_grammar(grammar, cfg)?;
    tree.to_model_json_string().map_err(Error::from)
}

pub fn parse_grammar_with<U>(cursor: U, cfg: &CfgA) -> Result<Tree>
where
    U: Cursor + Clone,
{
    let boot = boot_grammar()?;
    let ctx = new_ctx(cursor, cfg);

    match boot.parse(ctx) {
        Ok(Yeap(_, tree)) => Ok(tree),
        Err(failure) => Err(failure.into()),
    }
}

pub fn parse_grammar_with_as_json<U>(cursor: U, cfg: &CfgA) -> Result<String>
where
    U: Cursor + Clone,
{
    let tree = parse_grammar_with(cursor, cfg)?;
    tree.to_model_json_string().map_err(Error::from)
}

pub fn compile(grammar: &str, cfg: &CfgA) -> Result<Grammar> {
    compile_with(StrCursor::new(grammar), cfg)
}

pub fn compile_to_json(grammar: &str, cfg: &CfgA) -> Result<String> {
    let compiled = compile(grammar, cfg)?;
    compiled.to_json_exp_string().map_err(Error::from)
}

pub fn compile_with<U>(cursor: U, cfg: &CfgA) -> Result<Grammar>
where
    U: Cursor + Clone,
{
    let tree = parse_grammar_with(cursor, cfg)?;
    Ok(Grammar::compile(&tree, cfg)?)
}

pub fn compile_with_as_json<U>(cursor: U, cfg: &CfgA) -> Result<String>
where
    U: Cursor + Clone,
{
    let compiled = compile_with(cursor, cfg)?;
    compiled.to_json_exp_string().map_err(Error::from)
}

pub fn load(json: &str, _cfg: &CfgA) -> Result<Grammar> {
    Ok(Grammar::serde_from_json(json)?)
}

pub fn load_as_json(json: &str, cfg: &CfgA) -> Result<String> {
    let grammar = load(json, cfg)?;
    grammar.to_json_exp_string().map_err(Error::from)
}

pub fn load_tree(json: &str, _cfg: &CfgA) -> Result<Tree> {
    Tree::from_model_json(json).map_err(Error::from)
}

pub fn load_tree_as_json(json: &str, cfg: &CfgA) -> Result<String> {
    let tree = load_tree(json, cfg)?;
    tree.to_model_json_string().map_err(Error::from)
}

pub fn pretty(grammar: &str, cfg: &CfgA) -> Result<String> {
    let grammar = compile(grammar, cfg)?;
    Ok(grammar.pretty_print())
}

pub fn pretty_tree(tree: &Tree, _cfg: &CfgA) -> Result<String> {
    Ok(tree.to_model_json_string()?)
}

pub fn pretty_tree_json(tree: &Tree, _cfg: &CfgA) -> Result<String> {
    tree.to_model_json_string().map_err(Error::from)
}

pub fn load_boot(_cfg: &CfgA) -> Result<Grammar> {
    boot_grammar()
}

pub fn load_boot_as_json(cfg: &CfgA) -> Result<String> {
    let grammar = load_boot(cfg)?;
    grammar.to_json_exp_string().map_err(Error::from)
}

pub fn boot_grammar_json(_cfg: &CfgA) -> Result<String> {
    let boot = boot_grammar()?;
    match boot.to_json_exp_string() {
        Ok(s) => Ok(s),
        Err(e) => Err(e.into()),
    }
}

pub fn boot_grammar_pretty(_cfg: &CfgA) -> Result<String> {
    let boot = boot_grammar()?;
    Ok(boot.pretty_print())
}

pub fn parse_input(parser: &Grammar, text: &str, cfg: &CfgA) -> Result<Tree> {
    let cursor = StrCursor::new(text);
    let ctx = new_ctx(cursor, cfg);
    match parser.parse(ctx) {
        Ok(Yeap(_, tree)) => Ok(tree),
        Err(failure) => Err(failure.into()),
    }
}

pub fn parse_input_to_json(parser: &Grammar, text: &str, cfg: &CfgA) -> Result<String> {
    let tree = parse_input(parser, text, cfg)?;
    Ok(tree.as_json_str())
}

pub fn parse(grammar: &str, text: &str, cfg: &CfgA) -> Result<Tree> {
    let parser = compile(grammar, cfg)?;
    parse_input(&parser, text, cfg)
}

pub fn parse_to_json(grammar: &str, text: &str, cfg: &CfgA) -> Result<String> {
    let parser = compile(grammar, cfg)?;
    parse_input_to_json(&parser, text, cfg)
}
