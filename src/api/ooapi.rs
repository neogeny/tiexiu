// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Object-oriented API for TieXiu

use crate::engine::new_ctx;
use crate::input::{Cursor, StrCursor};
use crate::peg::error::Yeap;
use crate::peg::grammar::PrettyPrint;
use crate::peg::*;
pub use crate::trees::Tree;
use crate::{Error, Result};
use json::JsonValue;
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

    pub fn update_cfg(&mut self, cfg: &[crate::cfg::CfgKey]) {
        self.cfg = cfg.into();
    }

    pub fn get(&mut self, grammar: &str) -> Option<Grammar> {
        let hash = compute_hash(grammar);
        self.cache.read().ok()?.get(&hash).cloned()
    }

    pub fn get_or_compile(&mut self, grammar: &str) -> Result<Grammar> {
        crate::ensure!(!grammar.is_empty())?;
        let hash = compute_hash(grammar);

        {
            let cache = self
                .cache
                .read()
                .map_err(|_| Error::from("lock poisoned"))?;
            if let Some(existing) = cache.get(&hash) {
                return Ok(existing.clone());
            }
        }

        let tree = self.parse_grammar_with(StrCursor::new(grammar))?;
        let compiled_grammar = Grammar::compile(&tree, &self.cfg)?;

        let mut cache = self
            .cache
            .write()
            .map_err(|_| Error::from("lock poisoned"))?;
        cache.insert(hash, compiled_grammar.clone());
        Ok(compiled_grammar)
    }

    pub fn parse_grammar(&mut self, grammar: &str) -> Result<Tree> {
        self.parse_grammar_with(StrCursor::new(grammar))
    }

    pub fn parse_grammar_to_json(&mut self, grammar: &str) -> Result<JsonValue> {
        let tree = self.parse_grammar(grammar)?;
        Ok(tree.to_json())
    }

    pub fn parse_grammar_to_json_string(&mut self, grammar: &str) -> Result<String> {
        let tree = self.parse_grammar(grammar)?;
        Ok(tree.to_json_string())
    }

    pub fn parse_grammar_with<U>(&mut self, cursor: U) -> Result<Tree>
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

    pub fn parse_grammar_to_json_with<U>(&mut self, cursor: U) -> Result<JsonValue>
    where
        U: Cursor + Clone,
    {
        let tree = self.parse_grammar_with(cursor)?;
        Ok(tree.to_json())
    }

    pub fn compile(&mut self, grammar: &str) -> Result<Grammar> {
        self.get_or_compile(grammar)
    }

    pub fn compile_to_json(&mut self, grammar: &str) -> Result<JsonValue> {
        let grammar = self.compile(grammar)?;
        Ok(grammar.to_json())
    }

    pub fn compile_to_json_string(&mut self, grammar: &str) -> Result<String> {
        let grammar = self.compile(grammar)?;
        grammar.to_json_string().map_err(Error::from)
    }

    pub fn compile_with<U>(&mut self, cursor: U) -> Result<Grammar>
    where
        U: Cursor + Clone,
    {
        let text = cursor.as_str().to_string();
        self.compile(&text)
    }

    pub fn compile_to_json_with<U>(&mut self, cursor: U) -> Result<JsonValue>
    where
        U: Cursor + Clone,
    {
        let grammar = self.compile_with(cursor)?;
        Ok(grammar.to_json())
    }

    pub fn load(&mut self, json: &str) -> Result<Grammar> {
        Ok(Grammar::from_json(json)?)
    }

    pub fn load_to_json(&mut self, json: &str) -> Result<JsonValue> {
        let grammar = self.load(json)?;
        Ok(grammar.to_json())
    }

    pub fn load_tree(&mut self, json: &str) -> Result<Tree> {
        Tree::from_json_str(json).map_err(Error::from)
    }

    pub fn load_tree_to_json(&mut self, json: &str) -> Result<JsonValue> {
        let tree = self.load_tree(json)?;
        Ok(tree.to_json())
    }

    pub fn grammar_pretty(&mut self, grammar: &str) -> Result<String> {
        let grammar = self.compile(grammar)?;
        Ok(grammar.pretty_print())
    }

    pub fn parse(&mut self, grammar: &str, text: &str) -> Result<Tree> {
        let parser = self.compile(grammar)?;
        let cursor = StrCursor::new(text);
        let ctx = new_ctx(cursor, &self.cfg);
        match parser.parse(ctx) {
            Ok(Yeap(_, tree)) => Ok(tree),
            Err(failure) => Err(failure.into()),
        }
    }

    pub fn parse_to_json(&mut self, grammar: &str, text: &str) -> Result<JsonValue> {
        let tree = self.parse(grammar, text)?;
        Ok(tree.to_json())
    }

    pub fn parse_to_json_string(&mut self, grammar: &str, text: &str) -> Result<String> {
        let tree = self.parse(grammar, text)?;
        Ok(tree.to_json_string())
    }

    pub fn parse_input(&mut self, parser: &Grammar, text: &str) -> Result<Tree> {
        let cursor = StrCursor::new(text);
        let ctx = new_ctx(cursor, &self.cfg);
        match parser.parse(ctx) {
            Ok(Yeap(_, tree)) => Ok(tree),
            Err(failure) => Err(failure.into()),
        }
    }

    pub fn parse_input_to_json(&mut self, parser: &Grammar, text: &str) -> Result<JsonValue> {
        let tree = self.parse_input(parser, text)?;
        Ok(tree.to_json())
    }

    pub fn parse_input_to_json_string(&mut self, parser: &Grammar, text: &str) -> Result<String> {
        let tree = self.parse_input(parser, text)?;
        Ok(tree.to_json_string())
    }

    pub fn boot_grammar(&mut self) -> Result<Grammar> {
        Ok(crate::json::boot::boot_grammar()?)
    }

    pub fn load_boot(&mut self) -> Result<Grammar> {
        self.boot_grammar()
    }

    pub fn boot_grammar_to_json(&mut self) -> Result<JsonValue> {
        let grammar = self.load_boot()?;
        Ok(grammar.to_json())
    }

    pub fn boot_grammar_to_json_string(&mut self) -> Result<String> {
        let grammar = self.load_boot()?;
        grammar.to_json_string().map_err(Error::from)
    }

    pub fn boot_grammar_pretty(&mut self) -> Result<String> {
        let boot = self.boot_grammar()?;
        Ok(boot.pretty_print())
    }
}
