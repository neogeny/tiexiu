// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::{CfgA, CfgKey};
use crate::context::new_ctx;
use crate::input::{Cursor, StrCursor};
use crate::peg::grammar::PrettyPrint;
use crate::peg::*;
use crate::trees::{Tree, TreeRef};
use crate::{Error, Result, config};

use crate::api::ebnf_semantics::new_ebnf_grammar_semantics;
use crate::api::ooapi::TieXiu;

/// Create a default `TieXiu` instance with empty config.
pub fn pegapi() -> TieXiu {
    TieXiu::new(&[])
}

/// Parse a grammar string into a parse tree.
pub fn parse_grammar(grammar: &str, cfg: &CfgA) -> Result<Tree> {
    parse_grammar_with(StrCursor::new(grammar), cfg)
}

/// Parse grammar from a generic cursor source.
pub fn parse_grammar_with<U>(cursor: U, cfga: &CfgA) -> Result<Tree>
where
    U: Cursor + Clone,
{
    let boot = boot_grammar()?;

    let semkey = CfgKey::Semantics(new_ebnf_grammar_semantics());
    let cfg = config(&[semkey]).merge(&config(cfga));

    let mut ctx = new_ctx(cursor, &cfg);
    boot.parse_tree(&mut ctx)
}

/// Compile a grammar string into a `Grammar`.
pub fn compile(grammar: &str, cfg: &CfgA) -> Result<Grammar> {
    compile_with(StrCursor::new(grammar), cfg)
}

/// Compile grammar from a generic cursor source.
pub fn compile_with<U>(cursor: U, cfg: &CfgA) -> Result<Grammar>
where
    U: Cursor + Clone,
{
    let tree = parse_grammar_with(cursor, cfg)?;
    let tree: TreeRef = tree.into();
    Ok(Grammar::compile(&tree, cfg)?)
}

/// Load a grammar from a JSON string.
pub fn load_grammar_from_json(json: &str) -> Result<Grammar> {
    Ok(Grammar::from_json(json)?)
}

/// Load a parse tree from a JSON string.
pub fn load_tree_from_json(json: &str) -> Result<Tree> {
    Tree::from_json_str(json).map_err(Error::from)
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

/// Parse input text with a pre-compiled `Grammar`.
pub fn parse_input(parser: &Grammar, text: &str, cfg: &CfgA) -> Result<Tree> {
    parser.parse_input(text, cfg)
}

/// Load the boot grammar.
pub fn boot_grammar() -> Result<Grammar> {
    Ok(boot::boot_grammar()?)
}

/// Alias for `boot_grammar`.
pub fn load_boot() -> Result<Grammar> {
    boot_grammar()
}

/// Pretty-print the boot grammar.
pub fn boot_grammar_pretty() -> Result<String> {
    let boot = boot_grammar()?;
    Ok(boot.pretty_print())
}
