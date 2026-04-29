// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Object-oriented API for TieXiu

use crate::engine::new_ctx;
use crate::input::{Cursor, StrCursor};
use crate::json::ToExpJson;
use crate::peg::error::Yeap;
use crate::peg::grammar::PrettyPrint;
use crate::peg::*;
pub use crate::trees::Tree;
use crate::{Error, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::RwLock;

pub struct TieXiu {
    cfg: Box<[crate::cfg::CfgKey]>,
    cache: RwLock<HashMap<u64, Grammar>>,
}

impl Default for TieXiu {
    fn default() -> Self {
        Self::new(&[])
    }
}

fn compute_hash(text: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

impl TieXiu {
    pub fn new(cfg: &[crate::cfg::CfgKey]) -> Self {
        Self {
            cfg: cfg.into(),
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub fn get(&self, grammar: &str) -> Option<Grammar> {
        let hash = compute_hash(grammar);
        self.cache.read().ok()?.get(&hash).cloned()
    }

    pub fn get_or_compile(&self, grammar: &str) -> Result<Grammar> {
        crate::ensure!(!grammar.is_empty())?;
        let hash = compute_hash(grammar);

        {
            let cache = self
                .cache
                .read()
                .map_err(|_| crate::Error::from("lock poisoned"))?;
            if let Some(existing) = cache.get(&hash) {
                return Ok(existing.clone());
            }
        }

        let tree = self.parse_grammar_with(StrCursor::new(grammar))?;
        let compiled_grammar = Grammar::compile(&tree, &self.cfg)?;

        let mut cache = self
            .cache
            .write()
            .map_err(|_| crate::Error::from("lock poisoned"))?;
        cache.insert(hash, compiled_grammar.clone());
        Ok(compiled_grammar)
    }

    pub fn parse_grammar(&self, grammar: &str) -> Result<Tree> {
        self.parse_grammar_with(StrCursor::new(grammar))
    }

    pub fn parse_grammar_to_json(&self, grammar: &str) -> Result<Value> {
        let tree = self.parse_grammar(grammar)?;
        Ok(tree.to_json())
    }

    pub fn parse_grammar_to_json_string(&self, grammar: &str) -> Result<String> {
        let tree = self.parse_grammar(grammar)?;
        Ok(tree.to_json_string()?)
    }

    pub fn parse_grammar_with<U>(&self, cursor: U) -> Result<Tree>
    where
        U: Cursor + Clone,
    {
        let boot = self.boot_grammar()?;
        let ctx = new_ctx(cursor, &self.cfg);
        match boot.parse(ctx) {
            Ok(Yeap(_, tree)) => Ok(tree),
            Err(failure) => Err(failure.into()),
        }
    }

    pub fn parse_grammar_to_json_with<U>(&self, cursor: U) -> Result<Value>
    where
        U: Cursor + Clone,
    {
        let tree = self.parse_grammar_with(cursor)?;
        Ok(tree.to_json())
    }

    pub fn compile(&self, grammar: &str) -> Result<Grammar> {
        let compiled = self.get_or_compile(grammar)?;
        Ok(compiled)
    }

    pub fn compile_to_json(&self, grammar: &str) -> Result<Value> {
        let compiled = self.compile(grammar)?;
        Ok(compiled.to_json())
    }

    pub fn compile_to_json_string(&self, grammar: &str) -> Result<String> {
        let compiled = self.compile(grammar)?;
        Ok(compiled.to_json_string()?)
    }

    pub fn compile_with<U>(&self, cursor: U) -> Result<Grammar>
    where
        U: Cursor + Clone,
    {
        let text = cursor.textstr().to_string();
        self.compile(&text)
    }

    pub fn compile_to_json_with<U>(&self, cursor: U) -> Result<Value>
    where
        U: crate::input::Cursor + Clone,
    {
        let compiled = self.compile_with(cursor)?;
        Ok(compiled.to_json())
    }

    pub fn load(&self, json: &str) -> Result<Grammar> {
        Ok(Grammar::serde_from_json(json)?)
    }

    pub fn load_to_json(&self, json: &str) -> Result<Value> {
        let grammar = self.load(json)?;
        Ok(grammar.to_json())
    }

    pub fn load_tree(&self, json: &str) -> Result<Tree> {
        Tree::from_json_str(json).map_err(Error::from)
    }

    pub fn load_tree_to_json(&self, json: &str) -> Result<Value> {
        let tree = self.load_tree(json)?;
        Ok(tree.to_json())
    }

    pub fn grammar_pretty(&self, grammar: &str) -> Result<String> {
        let grammar = self.compile(grammar)?;
        Ok(grammar.pretty_print())
    }

    pub fn pretty_tree(&self, tree: &Tree) -> Result<String> {
        Ok(tree.to_json_string()?)
    }

    pub fn pretty_tree_json(&self, tree: &Tree) -> Result<String> {
        tree.to_json_string().map_err(Error::from)
    }

    pub fn parse(&self, grammar: &str, text: &str) -> Result<Tree> {
        let parser = self.compile(grammar)?;
        self.parse_input(&parser, text)
    }

    pub fn parse_to_json(&self, grammar: &str, text: &str) -> Result<Value> {
        let parser = self.compile(grammar)?;
        self.parse_input_to_json(&parser, text)
    }

    pub fn parse_to_json_string(&self, grammar: &str, text: &str) -> Result<String> {
        let parser = self.compile(grammar)?;
        self.parse_input_to_json_string(&parser, text)
    }

    pub fn parse_input(&self, parser: &Grammar, text: &str) -> Result<Tree> {
        let cursor = StrCursor::new(text);
        let ctx = new_ctx(cursor, &self.cfg);
        match parser.parse(ctx) {
            Ok(Yeap(_, tree)) => Ok(tree),
            Err(failure) => Err(failure.into()),
        }
    }

    pub fn parse_input_to_json(&self, parser: &Grammar, text: &str) -> Result<Value> {
        let tree = self.parse_input(parser, text)?;
        Ok(tree.to_json())
    }

    pub fn parse_input_to_json_string(&self, parser: &Grammar, text: &str) -> Result<String> {
        let tree = self.parse_input(parser, text)?;
        tree.to_json_string().map_err(Error::from)
    }

    pub fn boot_grammar(&self) -> Result<Grammar> {
        Ok(crate::json::boot::boot_grammar()?)
    }

    pub fn load_boot(&self) -> Result<Grammar> {
        self.boot_grammar()
    }

    pub fn boot_grammar_to_json(&self) -> Result<Value> {
        let grammar = self.load_boot()?;
        Ok(grammar.to_json())
    }

    pub fn boot_grammar_to_json_string(&self) -> Result<String> {
        let grammar = self.load_boot()?;
        grammar.to_json_string().map_err(Error::from)
    }

    pub fn boot_grammar_pretty(&self) -> Result<String> {
        let boot = self.boot_grammar()?;
        Ok(boot.pretty_print())
    }
}
