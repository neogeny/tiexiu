// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{Exp, Grammar, Rule};
use crate::trees::{Tree, TreeMap};
use indexmap::IndexMap;
use thiserror::Error;

pub type CompileResult<T> = Result<T, CompileError>;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum CompileError {
    #[error("expected {0} to be a Tree::Node")]
    ExpectedNode(String),

    #[error("expected {0} to contain a Tree::Map")]
    ExpectedMap(String),

    #[error("expected {0} to be Tree::Text")]
    ExpectedText(&'static str),

    #[error("expected {0} to be Tree::List")]
    ExpectedList(String),

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

    #[error("expected {expected}, found '{found}'")]
    UnexpectedTypeName { expected: Box<str>, found: Box<str> },

    #[error("{0} is not implemented")]
    NotImplemented(&'static str),

    #[error("compile_rhs() does not support node '{0}'")]
    UnsupportedRhs(Box<str>),
}

#[derive(Debug, Default)]
pub struct GrammarCompiler {
    pub rulemap: IndexMap<Box<str>, Rule>,
}

fn parse_node(node: &Tree) -> CompileResult<(Box<str>, &Tree)> {
    let Tree::Node { typename, tree } = node else {
        return Err(CompileError::ExpectedNode(format!("{:?}", node)));
    };
    Ok((typename.clone(), tree))
}

fn parse_node_check<'n>(node: &'n Tree, typename: &'static str) -> CompileResult<&'n Tree> {
    let (name, tree) = parse_node(node)?;
    if *name != *typename {
        return Err(CompileError::UnexpectedNodeName {
            expected: typename,
            found: name.clone(),
        });
    }
    Ok(tree)
}

fn _parse_map(node: &Tree) -> CompileResult<&TreeMap> {
    let Tree::Map(map) = node else {
        return Err(CompileError::ExpectedMap(format!("{:?}", node)));
    };
    Ok(map)
}

fn _parse_list(node: &Tree) -> CompileResult<&[Tree]> {
    match node {
        Tree::List(list) | Tree::Closed(list) => Ok(list),
        _ => Err(CompileError::ExpectedList(format!("{:?}", node))),
    }
}

fn map_get<'m>(map: &'m Tree, context: &'static str, key: &'static str) -> CompileResult<&'m Tree> {
    match map.get(key) {
        Some(node) => Ok(node),
        None => Err(CompileError::MissingKey { context, key }),
    }
}

impl GrammarCompiler {
    pub fn new() -> GrammarCompiler {
        GrammarCompiler {
            rulemap: IndexMap::new(),
        }
    }

    pub fn compile_grammar(&self, tree: &Tree) -> CompileResult<Grammar> {
        // NOTE:
        //  If we get called then the `Tree` is not any generic `Tree` but one
        //  produced by parsing a grammar description written in the TieXiu/TatSu
        //  variant of EBNF they accept.
        //  _
        //  We know the exact structure of the `Tree` so we'll parse it top down
        //  validating the expected node type at each step.
        //  _
        //  All nodes of type `Tree::Node` are produced by a rule in the meta-grammar
        //  so it's possible to dispatch by the rule name in `node.meta.name`.
        //  _
        //  Some `Tree::Node` have an associated node type in `node.meta.params[0]`
        //  and that too can be verified

        let map = parse_node_check(tree, "Grammar")?;
        let rules = map_get(map, "Grammar", "rules")?;
        eprintln!("{:?}", rules);
        // let rule_trees = parse_list(rules_node)?;
        // panic!("SEE THE TREE");
        // if *meta.name != *"Grammar" {
        //     return Err(CompileError::UnexpectedNodeName {
        //         expected: "Grammar",
        //         found: meta.into(),
        //     });
        // }
        // self.expect_keys(m, &["name", "directives", "keywords", "rules"], "Grammar")?;
        //
        // let grammar_name = self.text_field(m, "name")?;
        // let directives = self.directives(self.field(m, "directives")?)?;
        // let keywords = self.keywords(self.field(m, "keywords")?)?;
        // let rule_trees = self.list_field(m, "rules")?;
        // let rules = self.compile_rules(rule_trees)?;

        let grammar = Grammar::new("GRAMMAR", &[]);
        // grammar.directives = directives;
        // grammar.keywords = keywords;
        Ok(grammar)
    }

    pub fn compile_rule(&self, _tree: &Tree) -> CompileResult<Rule> {
        // let (_meta, tree) = parse_node_check(tree, "rule", "Rule")?;
        // let map = parse_map(tree)?;
        // let name = map_get(map, "rule", "name")?;
        // let params = map_get(map, "rule", "params")?;
        // let body = map_get(map, "rule", "body")?;
        Ok(Rule::new("rule", &[], Exp::nil()))
    }
}

impl Grammar {
    pub fn compile(tree: &Tree) -> CompileResult<Self> {
        let compiler = GrammarCompiler::new();
        compiler.compile_grammar(tree)
    }
}
