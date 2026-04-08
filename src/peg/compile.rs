// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{Exp, Grammar, Rule};
use crate::trees::{Tree, TreeMap};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct GrammarCompiler;

impl GrammarCompiler {
    fn compile_rules(&self, trees: &[Tree]) -> Vec<Rule> {
        trees.iter().map(|tree| self.compile_rule(tree)).collect()
    }

    fn node_map<'a>(&self, tree: &'a Tree, expected: &'static str) -> (&'a str, &'a TreeMap) {
        let Tree::Node { info, tree } = tree else {
            panic!("expected {expected} to be a Tree::Node");
        };
        let Tree::Map(map) = tree.as_ref() else {
            panic!("expected {expected} to contain a Tree::Map");
        };
        (info.name.as_ref(), map.as_ref())
    }

    fn text<'a>(&self, tree: &'a Tree, expected: &'static str) -> &'a str {
        let Tree::Text(text) = tree else {
            panic!("expected {expected} to be Tree::Text");
        };
        text.as_ref()
    }

    fn expect_keys(&self, treemap: &TreeMap, expected: &[&str], context: &'static str) {
        for key in expected {
            assert!(
                treemap.get(key).is_some(),
                "expected {context} to contain key '{key}'"
            );
        }
    }

    fn field<'a>(&self, treemap: &'a TreeMap, key: &'static str) -> &'a Tree {
        let Some(tree) = treemap.get(key) else {
            panic!("expected {key}");
        };
        tree
    }

    fn text_field<'a>(&self, treemap: &'a TreeMap, key: &'static str) -> &'a str {
        self.text(self.field(treemap, key), key)
    }

    fn list_field<'a>(&self, treemap: &'a TreeMap, key: &'static str) -> &'a [Tree] {
        self.list(self.field(treemap, key), key)
    }

    fn rhs_field(&self, treemap: &TreeMap, key: &'static str) -> Exp {
        self.compile_rhs(self.field(treemap, key))
    }

    fn rhs_list(&self, treemap: &TreeMap, key: &'static str) -> Vec<Exp> {
        self.list_field(treemap, key)
            .iter()
            .map(|tree| self.compile_rhs(tree))
            .collect()
    }

    fn list<'a>(&self, tree: &'a Tree, expected: &'static str) -> &'a [Tree] {
        let Tree::List(items) = tree else {
            panic!("expected {expected} to be Tree::List");
        };
        items.as_ref()
    }

    fn text_list(&self, tree: &Tree, expected: &'static str) -> Vec<String> {
        match tree {
            Tree::Nil => Vec::new(),
            Tree::List(items) => items
                .iter()
                .map(|item| self.text(item, expected).to_string())
                .collect(),
            _ => panic!("expected {expected} to be Tree::List or Tree::Nil"),
        }
    }

    fn contains_text(&self, tree: &Tree, expected: &'static str, item: &str) -> bool {
        match tree {
            Tree::Nil => false,
            Tree::List(items) => items.iter().any(|entry| self.text(entry, expected) == item),
            _ => panic!("expected {expected} to be Tree::List or Tree::Nil"),
        }
    }

    fn bool_text(&self, tree: &Tree, expected: &'static str) -> bool {
        match tree {
            Tree::Text(text) => matches!(text.as_ref(), "True" | "true"),
            Tree::Nil => false,
            _ => panic!("expected {expected} to be Tree::Text or Tree::Nil"),
        }
    }

    fn directives(&self, tree: &Tree) -> HashMap<String, String> {
        match tree {
            Tree::Nil => HashMap::new(),
            Tree::Map(map) => map
                .entries
                .iter()
                .map(|(key, value)| (key.to_string(), self.text(value, "directive value").into()))
                .collect(),
            _ => panic!("expected Grammar.directives to be Tree::Map or Tree::Nil"),
        }
    }

    fn keywords(&self, tree: &Tree) -> HashSet<String> {
        match tree {
            Tree::Nil => HashSet::new(),
            Tree::List(items) => items
                .iter()
                .map(|item| self.text(item, "keyword").to_string())
                .collect(),
            _ => panic!("expected Grammar.keywords to be Tree::List or Tree::Nil"),
        }
    }

    fn rule_name<'a>(&self, tree: &'a Tree) -> &'a str {
        let (name, treemap) = self.node_map(tree, "rule");
        assert_eq!(name, "Rule");
        self.text_field(treemap, "name")
    }

    fn bool_flag(&self, treemap: &TreeMap, key: &'static str) -> bool {
        treemap
            .get(key)
            .map(|tree| self.bool_text(tree, key))
            .unwrap_or(false)
    }

    fn one_of(&self, treemap: &TreeMap, keys: &[&'static str], default: bool) -> bool {
        keys.iter()
            .find_map(|key| treemap.get(key).map(|tree| self.bool_text(tree, key)))
            .unwrap_or(default)
    }

    fn compile_rule_decorators(&self, treemap: &TreeMap) -> (bool, bool) {
        let decorators = self.field(treemap, "decorators");

        assert!(
            !self.contains_text(decorators, "decorators", "override"),
            "override decorator is not implemented"
        );

        let is_name = self.contains_text(decorators, "decorators", "name")
            || self.contains_text(decorators, "decorators", "isname");
        let no_memo = self.contains_text(decorators, "decorators", "nomemo");

        (is_name, no_memo)
    }

    fn unary(&self, treemap: &TreeMap, key: &'static str, build: fn(Exp) -> Exp) -> Exp {
        build(self.rhs_field(treemap, key))
    }

    fn binary(
        &self,
        treemap: &TreeMap,
        left: &'static str,
        right: &'static str,
        build: fn(Exp, Exp) -> Exp,
    ) -> Exp {
        build(
            self.rhs_field(treemap, left),
            self.rhs_field(treemap, right),
        )
    }

    fn alert(&self, treemap: &TreeMap) -> Exp {
        let message = treemap
            .get("message")
            .or_else(|| treemap.get("literal"))
            .unwrap_or_else(|| panic!("expected message"));
        let code = self.text_field(treemap, "level").len() as u8;
        Exp::alert(self.text(message, "message"), code)
    }

    pub fn compile_grammar(&self, tree: &Tree) -> Grammar {
        let (name, treemap) = self.node_map(tree, "grammar");
        assert_eq!(name, "Grammar");
        self.expect_keys(
            treemap,
            &["name", "directives", "keywords", "rules"],
            "Grammar",
        );

        let grammar_name = self.text_field(treemap, "name");
        let directives = self.directives(self.field(treemap, "directives"));
        let keywords = self.keywords(self.field(treemap, "keywords"));
        let rule_trees = self.list_field(treemap, "rules");
        let rules = self.compile_rules(rule_trees);

        let mut grammar = Grammar::new(grammar_name, &rules);
        grammar.directives = directives;
        grammar.keywords = keywords;
        grammar
    }

    pub fn compile_rule(&self, tree: &Tree) -> Rule {
        let (name, treemap) = self.node_map(tree, "rule");
        assert_eq!(name, "Rule");
        self.expect_keys(
            treemap,
            &["name", "params", "exp", "decorators", "base", "kwparams"],
            "Rule",
        );

        let rule_name = self.text_field(treemap, "name").to_string();
        let params = self.text_list(self.field(treemap, "params"), "params");
        let exp = self.rhs_field(treemap, "exp");
        let (decorator_is_name, decorator_no_memo) = self.compile_rule_decorators(treemap);

        let is_name = self.bool_flag(treemap, "is_name") || decorator_is_name;
        let is_tokn = self.bool_flag(treemap, "is_tokn");
        let no_memo = self.bool_flag(treemap, "no_memo") || decorator_no_memo;
        let is_memo = self.one_of(treemap, &["is_memo", "is_memoizable"], true);
        let is_lrec = self.one_of(treemap, &["is_lrec", "is_leftrec"], false);

        assert!(
            matches!(self.field(treemap, "base"), Tree::Nil),
            "base rules are not implemented"
        );
        assert!(
            matches!(self.field(treemap, "kwparams"), Tree::Nil | Tree::Map(_)),
            "kwparams must be empty until implemented"
        );
        if let Tree::Map(kwparams) = self.field(treemap, "kwparams") {
            assert!(kwparams.entries.is_empty(), "kwparams are not implemented");
        }

        Rule::from_parts(
            rule_name, params, exp, is_name, is_tokn, no_memo, is_memo, is_lrec,
        )
    }

    pub fn compile_rhs(&self, tree: &Tree) -> Exp {
        let (name, treemap) = self.node_map(tree, "expression");

        match name {
            "Sequence" => Exp::sequence(self.rhs_list(treemap, "sequence")),
            "Choice" => Exp::choice(self.rhs_list(treemap, "options")),
            "Option" => Exp::alt(self.rhs_field(treemap, "exp")),
            "Call" | "RuleRef" => Exp::call(self.text_field(treemap, "name")),
            "RuleInclude" => {
                Exp::rule_include(self.rule_name(self.field(treemap, "rule")), Exp::nil())
            }
            "Token" => Exp::token(self.text_field(treemap, "token")),
            "Pattern" => Exp::pattern(self.text_field(treemap, "pattern")),
            "Constant" => Exp::constant(self.text_field(treemap, "literal")),
            "Alert" => self.alert(treemap),
            "Cut" => Exp::cut(),
            "Void" => Exp::void(),
            "Fail" => Exp::fail(),
            "Dot" => Exp::dot(),
            "EOF" => Exp::eof(),
            "Named" => Exp::named(
                self.text_field(treemap, "name"),
                self.rhs_field(treemap, "exp"),
            ),
            "NamedList" => Exp::named_list(
                self.text_field(treemap, "name"),
                self.rhs_field(treemap, "exp"),
            ),
            "Override" => self.unary(treemap, "exp", Exp::override_node),
            "OverrideList" => self.unary(treemap, "exp", Exp::override_list),
            "Group" => self.unary(treemap, "exp", Exp::group),
            "SkipGroup" => self.unary(treemap, "exp", Exp::skip_group),
            "Lookahead" => self.unary(treemap, "exp", Exp::lookahead),
            "NegativeLookahead" => self.unary(treemap, "exp", Exp::negative_lookahead),
            "SkipTo" => self.unary(treemap, "exp", Exp::skip_to),
            "Optional" => self.unary(treemap, "exp", Exp::optional),
            "Closure" => self.unary(treemap, "exp", Exp::closure),
            "PositiveClosure" => self.unary(treemap, "exp", Exp::positive_closure),
            "Join" => self.binary(treemap, "exp", "sep", Exp::join),
            "PositiveJoin" => self.binary(treemap, "exp", "sep", Exp::positive_join),
            "Gather" => self.binary(treemap, "exp", "sep", Exp::gather),
            "PositiveGather" => self.binary(treemap, "exp", "sep", Exp::positive_gather),
            other => panic!("compile_rhs() does not support node '{other}'"),
        }
    }
}

impl Grammar {
    pub fn compile(tree: &Tree) -> Self {
        GrammarCompiler.compile_grammar(tree)
    }
}
