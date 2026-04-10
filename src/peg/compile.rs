// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{Exp, Grammar, Rule};
use crate::trees::{Tree, TreeMap};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

pub type CompileResult<T> = Result<T, CompileError>;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum CompileError {
    #[error("expected {0} to be a Tree::Node")]
    ExpectedNode(&'static str),

    #[error("expected {0} to contain a Tree::Map")]
    ExpectedNodeMap(&'static str),

    #[error("expected {0} to be Tree::Text")]
    ExpectedText(&'static str),

    #[error("expected {0} to be Tree::List")]
    ExpectedList(&'static str),

    #[error("expected {0} to be Tree::List or Tree::Nil")]
    ExpectedListOrNil(&'static str),

    #[error("expected {0} to be Tree::Text or Tree::Nil")]
    ExpectedTextOrNil(&'static str),

    #[error("expected {context} to contain key '{key}'")]
    MissingKey {
        context: &'static str,
        key: &'static str,
    },

    #[error("expected {0}")]
    ExpectedField(&'static str),

    #[error("expected {expected}, found '{found}'")]
    UnexpectedNodeName {
        expected: &'static str,
        found: Box<str>,
    },

    #[error("{0} is not implemented")]
    NotImplemented(&'static str),

    #[error("compile_rhs() does not support node '{0}'")]
    UnsupportedRhs(Box<str>),
}

#[derive(Debug, Default)]
pub struct GrammarCompiler;

impl GrammarCompiler {
    fn compile_rules(&self, trees: &[Tree]) -> CompileResult<Vec<Rule>> {
        trees.iter().map(|tree| self.compile_rule(tree)).collect()
    }

    fn node_map<'a>(
        &self,
        tree: &'a Tree,
        expected: &'static str,
    ) -> CompileResult<(&'a str, &'a TreeMap)> {
        let Tree::Node { meta, tree } = tree else {
            return Err(CompileError::ExpectedNode(expected));
        };
        let Tree::Map(map) = tree.as_ref() else {
            return Err(CompileError::ExpectedNodeMap(expected));
        };
        Ok((meta.name.as_ref(), map.as_ref()))
    }

    fn text<'a>(&self, tree: &'a Tree, expected: &'static str) -> CompileResult<&'a str> {
        let Tree::Text(text) = tree else {
            return Err(CompileError::ExpectedText(expected));
        };
        Ok(text.as_ref())
    }

    fn expect_keys(
        &self,
        m: &TreeMap,
        expected: &[&'static str],
        context: &'static str,
    ) -> CompileResult<()> {
        for key in expected {
            if m.get(key).is_none() {
                return Err(CompileError::MissingKey { context, key });
            }
        }
        Ok(())
    }

    fn field<'a>(&self, m: &'a TreeMap, key: &'static str) -> CompileResult<&'a Tree> {
        let Some(tree) = m.get(key) else {
            return Err(CompileError::ExpectedField(key));
        };
        Ok(tree)
    }

    fn text_field<'a>(&self, m: &'a TreeMap, key: &'static str) -> CompileResult<&'a str> {
        self.text(self.field(m, key)?, key)
    }

    fn list_field<'a>(&self, m: &'a TreeMap, key: &'static str) -> CompileResult<&'a [Tree]> {
        self.list(self.field(m, key)?, key)
    }

    fn rhs_field(&self, m: &TreeMap, key: &'static str) -> CompileResult<Exp> {
        self.compile_rhs(self.field(m, key)?)
    }

    fn rhs_list(&self, m: &TreeMap, key: &'static str) -> CompileResult<Vec<Exp>> {
        self.list_field(m, key)?
            .iter()
            .map(|tree| self.compile_rhs(tree))
            .collect()
    }

    fn list<'a>(&self, tree: &'a Tree, expected: &'static str) -> CompileResult<&'a [Tree]> {
        let Tree::List(items) = tree else {
            return Err(CompileError::ExpectedList(expected));
        };
        Ok(items.as_ref())
    }

    fn text_list(&self, tree: &Tree, expected: &'static str) -> CompileResult<Vec<String>> {
        match tree {
            Tree::Nil => Ok(Vec::new()),
            Tree::List(items) => items
                .iter()
                .map(|item| self.text(item, expected).map(ToString::to_string))
                .collect(),
            _ => Err(CompileError::ExpectedListOrNil(expected)),
        }
    }

    fn contains_text(
        &self,
        tree: &Tree,
        expected: &'static str,
        item: &str,
    ) -> CompileResult<bool> {
        match tree {
            Tree::Nil => Ok(false),
            Tree::List(items) => {
                for entry in items.iter() {
                    if self.text(entry, expected)? == item {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            _ => Err(CompileError::ExpectedListOrNil(expected)),
        }
    }

    fn bool_text(&self, tree: &Tree, expected: &'static str) -> CompileResult<bool> {
        match tree {
            Tree::Text(text) => Ok(matches!(text.as_ref(), "True" | "true")),
            Tree::Nil => Ok(false),
            _ => Err(CompileError::ExpectedTextOrNil(expected)),
        }
    }

    fn directives(&self, tree: &Tree) -> CompileResult<HashMap<String, String>> {
        match tree {
            Tree::Nil => Ok(HashMap::new()),
            Tree::Map(map) => map
                .entries
                .iter()
                .map(|(key, value)| {
                    self.text(value, "directive value")
                        .map(|value| (key.to_string(), value.into()))
                })
                .collect(),
            _ => Err(CompileError::NotImplemented("Grammar.directives shape")),
        }
    }

    fn keywords(&self, tree: &Tree) -> CompileResult<HashSet<String>> {
        match tree {
            Tree::Nil => Ok(HashSet::new()),
            Tree::List(items) => items
                .iter()
                .map(|item| self.text(item, "keyword").map(ToString::to_string))
                .collect(),
            _ => Err(CompileError::ExpectedListOrNil("keywords")),
        }
    }

    fn rule_name<'a>(&self, tree: &'a Tree) -> CompileResult<&'a str> {
        let (name, m) = self.node_map(tree, "rule")?;
        if name != "Rule" {
            return Err(CompileError::UnexpectedNodeName {
                expected: "Rule",
                found: name.into(),
            });
        }
        self.text_field(m, "name")
    }

    fn bool_flag(&self, m: &TreeMap, key: &'static str) -> CompileResult<bool> {
        m.get(key)
            .map(|tree| self.bool_text(tree, key))
            .transpose()
            .map(|value| value.unwrap_or(false))
    }

    fn one_of(&self, m: &TreeMap, keys: &[&'static str], default: bool) -> CompileResult<bool> {
        keys.iter()
            .find_map(|key| m.get(key).map(|tree| self.bool_text(tree, key)))
            .transpose()
            .map(|value| value.unwrap_or(default))
    }

    fn compile_rule_decorators(&self, m: &TreeMap) -> CompileResult<(bool, bool)> {
        let decorators = self.field(m, "decorators")?;

        if self.contains_text(decorators, "decorators", "override")? {
            return Err(CompileError::NotImplemented("override decorator"));
        }

        let is_name = self.contains_text(decorators, "decorators", "name")?
            || self.contains_text(decorators, "decorators", "isname")?;
        let no_memo = self.contains_text(decorators, "decorators", "nomemo")?;

        Ok((is_name, no_memo))
    }

    fn unary(&self, m: &TreeMap, key: &'static str, build: fn(Exp) -> Exp) -> CompileResult<Exp> {
        self.rhs_field(m, key).map(build)
    }

    fn binary(
        &self,
        m: &TreeMap,
        left: &'static str,
        right: &'static str,
        build: fn(Exp, Exp) -> Exp,
    ) -> CompileResult<Exp> {
        Ok(build(self.rhs_field(m, left)?, self.rhs_field(m, right)?))
    }

    fn alert(&self, m: &TreeMap) -> CompileResult<Exp> {
        let message = m
            .get("message")
            .or_else(|| m.get("literal"))
            .ok_or(CompileError::ExpectedField("message"))?;
        let code = self.text_field(m, "level")?.len() as u8;
        Ok(Exp::alert(self.text(message, "message")?, code))
    }

    pub fn compile_grammar(&self, tree: &Tree) -> CompileResult<Grammar> {
        let (name, m) = self.node_map(tree, "grammar")?;
        if name != "Grammar" {
            return Err(CompileError::UnexpectedNodeName {
                expected: "Grammar",
                found: name.into(),
            });
        }
        self.expect_keys(m, &["name", "directives", "keywords", "rules"], "Grammar")?;

        let grammar_name = self.text_field(m, "name")?;
        let directives = self.directives(self.field(m, "directives")?)?;
        let keywords = self.keywords(self.field(m, "keywords")?)?;
        let rule_trees = self.list_field(m, "rules")?;
        let rules = self.compile_rules(rule_trees)?;

        let mut grammar = Grammar::new(grammar_name, &rules);
        grammar.directives = directives;
        grammar.keywords = keywords;
        Ok(grammar)
    }

    pub fn compile_rule(&self, tree: &Tree) -> CompileResult<Rule> {
        let (name, m) = self.node_map(tree, "rule")?;
        if name != "Rule" {
            return Err(CompileError::UnexpectedNodeName {
                expected: "Rule",
                found: name.into(),
            });
        }
        self.expect_keys(
            m,
            &["name", "params", "exp", "decorators", "base", "kwparams"],
            "Rule",
        )?;

        let rule_name = self.text_field(m, "name")?.to_string();
        let params = self.text_list(self.field(m, "params")?, "params")?;
        let exp = self.rhs_field(m, "exp")?;
        let (decorator_is_name, decorator_no_memo) = self.compile_rule_decorators(m)?;

        let is_name = self.bool_flag(m, "is_name")? || decorator_is_name;
        let is_tokn = self.bool_flag(m, "is_tokn")?;
        let no_memo = self.bool_flag(m, "no_memo")? || decorator_no_memo;
        let is_memo = self.one_of(m, &["is_memo", "is_memoizable"], true)?;
        let is_lrec = self.one_of(m, &["is_lrec", "is_leftrec"], false)?;

        if !matches!(self.field(m, "base")?, Tree::Nil) {
            return Err(CompileError::NotImplemented("base rules"));
        }

        match self.field(m, "kwparams")? {
            Tree::Nil => {}
            Tree::Map(kwparams) if kwparams.entries.is_empty() => {}
            Tree::Map(_) => return Err(CompileError::NotImplemented("kwparams")),
            _ => return Err(CompileError::NotImplemented("kwparams shape")),
        }

        Ok(Rule::from_parts(
            rule_name, params, exp, is_name, is_tokn, no_memo, is_memo, is_lrec,
        ))
    }

    pub fn compile_rhs(&self, tree: &Tree) -> CompileResult<Exp> {
        let (name, m) = self.node_map(tree, "expression")?;

        match name {
            "Sequence" => Ok(Exp::sequence(self.rhs_list(m, "sequence")?)),
            "Choice" => Ok(Exp::choice(self.rhs_list(m, "options")?)),
            "Option" => Ok(Exp::alt(self.rhs_field(m, "exp")?)),
            "Call" | "RuleRef" => Ok(Exp::call(self.text_field(m, "name")?)),
            "RuleInclude" => Ok(Exp::rule_include(self.rule_name(self.field(m, "rule")?)?)),
            "Token" => Ok(Exp::token(self.text_field(m, "token")?)),
            "Pattern" => Ok(Exp::pattern(self.text_field(m, "pattern")?)),
            "Constant" => Ok(Exp::constant(self.text_field(m, "literal")?)),
            "Alert" => self.alert(m),
            "Cut" => Ok(Exp::cut()),
            "Void" => Ok(Exp::void()),
            "Fail" => Ok(Exp::fail()),
            "Dot" => Ok(Exp::dot()),
            "EOF" => Ok(Exp::eof()),
            "Named" => Ok(Exp::named(
                self.text_field(m, "name")?,
                self.rhs_field(m, "exp")?,
            )),
            "NamedList" => Ok(Exp::named_list(
                self.text_field(m, "name")?,
                self.rhs_field(m, "exp")?,
            )),
            "Override" => self.unary(m, "exp", Exp::override_node),
            "OverrideList" => self.unary(m, "exp", Exp::override_list),
            "Group" => self.unary(m, "exp", Exp::group),
            "SkipGroup" => self.unary(m, "exp", Exp::skip_group),
            "Lookahead" => self.unary(m, "exp", Exp::lookahead),
            "NegativeLookahead" => self.unary(m, "exp", Exp::negative_lookahead),
            "SkipTo" => self.unary(m, "exp", Exp::skip_to),
            "Optional" => self.unary(m, "exp", Exp::optional),
            "Closure" => self.unary(m, "exp", Exp::closure),
            "PositiveClosure" => self.unary(m, "exp", Exp::positive_closure),
            "Join" => self.binary(m, "exp", "sep", Exp::join),
            "PositiveJoin" => self.binary(m, "exp", "sep", Exp::positive_join),
            "Gather" => self.binary(m, "exp", "sep", Exp::gather),
            "PositiveGather" => self.binary(m, "exp", "sep", Exp::positive_gather),
            other => Err(CompileError::UnsupportedRhs(other.into())),
        }
    }
}

impl Grammar {
    pub fn compile(tree: &Tree) -> CompileResult<Self> {
        GrammarCompiler.compile_grammar(tree)
    }
}
