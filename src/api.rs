// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::input::Cursor;
use crate::input::StrCursor;
use crate::json::ToExpJson;
use crate::peg::{Grammar, Succ};
use crate::state::corectx::CoreCtx;
use crate::trees::Tree;
use crate::util::cfg::*;
use crate::{Error, Result};

pub fn boot_grammar() -> Result<Grammar> {
    Ok(crate::json::boot::boot_grammar()?)
}

pub fn parse_grammar(grammar: &str, cfg: CfgA) -> Result<Tree> {
    parse_grammar_with(StrCursor::new(grammar), cfg)
}

pub fn parse_grammar_as_json(grammar: &str, cfg: CfgA) -> Result<String> {
    let tree = parse_grammar(grammar, cfg)?;
    tree.to_model_json_string().map_err(Error::from)
}

pub fn parse_grammar_with<U>(cursor: U, cfg: CfgA) -> Result<Tree>
where
    U: Cursor + Clone,
{
    let _ = cfg;
    let boot = boot_grammar()?;
    let mut ctx = CoreCtx::new(cursor);
    ctx.configure(&cfg.into());
    if Cfg::new(cfg).is_enabled("trace") {
        ctx.set_trace(true);
    }

    match boot.parse(ctx) {
        Ok(Succ(_, tree)) => Ok(tree),
        Err(failure) => Err(failure.into()),
    }
}

pub fn parse_grammar_with_as_json<U>(cursor: U, cfg: CfgA) -> Result<String>
where
    U: Cursor + Clone,
{
    let tree = parse_grammar_with(cursor, cfg)?;
    tree.to_model_json_string().map_err(Error::from)
}

pub fn compile(grammar: &str, cfg: CfgA) -> Result<Grammar> {
    compile_with(StrCursor::new(grammar), cfg)
}

pub fn compile_to_json(grammar: &str, cfg: CfgA) -> Result<String> {
    let compiled = compile(grammar, cfg)?;
    compiled.to_json_exp_string().map_err(Error::from)
}

pub fn compile_with<U>(cursor: U, cfg: CfgA) -> Result<Grammar>
where
    U: Cursor + Clone,
{
    let tree = parse_grammar_with(cursor, cfg)?;
    Ok(Grammar::compile(&tree)?)
}

pub fn compile_with_as_json<U>(cursor: U, cfg: CfgA) -> Result<String>
where
    U: Cursor + Clone,
{
    let compiled = compile_with(cursor, cfg)?;
    compiled.to_json_exp_string().map_err(Error::from)
}

pub fn load(json: &str, cfg: CfgA) -> Result<Grammar> {
    let _ = cfg;
    Ok(Grammar::serde_from_json(json)?)
}

pub fn load_as_json(json: &str, cfg: CfgA) -> Result<String> {
    let grammar = load(json, cfg)?;
    grammar.to_json_exp_string().map_err(Error::from)
}

pub fn load_tree(json: &str, cfg: CfgA) -> Result<Tree> {
    let _ = cfg;
    Tree::from_model_json(json).map_err(Error::from)
}

pub fn load_tree_as_json(json: &str, cfg: CfgA) -> Result<String> {
    let tree = load_tree(json, cfg)?;
    tree.to_model_json_string().map_err(Error::from)
}

pub fn pretty(grammar: &str, cfg: CfgA) -> Result<String> {
    let grammar = compile(grammar, cfg)?;
    Ok(grammar.to_string())
}

pub fn pretty_tree(tree: &Tree, cfg: CfgA) -> Result<String> {
    let _ = cfg;
    Ok(tree.to_model_json_string()?)
}

pub fn pretty_tree_json(tree: &Tree, cfg: CfgA) -> Result<String> {
    let _ = cfg;
    tree.to_model_json_string().map_err(Error::from)
}

pub fn load_boot(cfg: CfgA) -> Result<Grammar> {
    let _ = cfg;
    boot_grammar()
}

pub fn load_boot_as_json(cfg: CfgA) -> Result<String> {
    let grammar = load_boot(cfg)?;
    grammar.to_json_exp_string().map_err(Error::from)
}

pub fn boot_grammar_json(cfg: CfgA) -> Result<String> {
    let _ = cfg;
    let boot = boot_grammar()?;
    match boot.to_json_exp_string() {
        Ok(s) => Ok(s),
        Err(e) => Err(e.into()),
    }
}

pub fn boot_grammar_pretty(cfg: CfgA) -> Result<String> {
    let _ = cfg;
    let boot = boot_grammar()?;
    Ok(boot.to_string())
}

pub fn parse_input(parser: &Grammar, text: &str, cfg: CfgA) -> Result<Tree> {
    let cursor = StrCursor::new(text);
    let mut ctx = CoreCtx::new(cursor);
    if Cfg::new(cfg).is_enabled("trace") {
        ctx.set_trace(true);
    }

    match parser.parse(ctx) {
        Ok(Succ(_, tree)) => Ok(tree),
        Err(failure) => Err(failure.into()),
    }
}

pub fn parse_input_to_json(parser: &Grammar, text: &str, cfg: CfgA) -> Result<String> {
    let tree = parse_input(parser, text, cfg)?;
    Ok(tree.as_json_str().to_string())
}

pub fn parse(grammar: &str, text: &str, cfg: CfgA) -> Result<Tree> {
    let parser = compile(grammar, cfg)?;
    parse_input(&parser, text, cfg)
}

pub fn parse_to_json(grammar: &str, text: &str, cfg: CfgA) -> Result<String> {
    let parser = compile(grammar, cfg)?;
    parse_input_to_json(&parser, text, cfg)
}
