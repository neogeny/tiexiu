// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{Exp, Grammar, Rule};
use crate::trees::{FlagMap, Tree, TreeMap};
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

    #[error("Unknown expression type '{0}'")]
    UnknownExpressionType(Box<str>),
}

#[derive(Debug, Default)]
pub struct GrammarCompiler {}

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

fn map_get_default(map: &Tree, key: &'static str, default: &'static str) -> Box<str> {
    match map.get(key) {
        Some(node) => node.value(),
        None => default.into(),
    }
}

impl GrammarCompiler {
    pub fn new() -> GrammarCompiler {
        GrammarCompiler {}
    }

    pub fn compile_grammar(&mut self, tree: &Tree) -> CompileResult<Grammar> {
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
        eprintln!("GRAMMAR {:?}", map);

        let rule_trees = map_get(map, "Grammar", "rules")?.value_list();

        let mut rulemap: IndexMap<Box<str>, Rule> = IndexMap::new();
        for rtree in rule_trees {
            let rule = self.compile_rule(&rtree)?;
            eprintln!("{:?}", &rule);
            rulemap.insert(rule.name.clone(), rule);
        }

        let rules: Vec<Rule> = rulemap.into_iter().map(|(_, r)| r).collect();
        eprintln!("{:?}", rules);
        let name = map_get_default(map, "name", "grammar");
        let grammar = Grammar::new(&name, rules.as_slice());

        // let directives = self.directives(self.field(m, "directives")?)?;
        // let keywords = self.keywords(self.field(m, "keywords")?)?;
        // grammar.directives = directives;
        // grammar.keywords = keywords;
        Ok(grammar)
    }

    pub fn compile_rule(&self, tree: &Tree) -> CompileResult<Rule> {
        let ctx = "Rule";
        let map = parse_node_check(tree, ctx)?;
        eprintln!("RULE {:?}", map);
        let name = map_get(map, ctx, "name")?.value();

        // let decorators = map_get(map, ctx, "decorators")?.value_map();
        let _flags = FlagMap::new();
        let exp = self.parse_exp(map_get(map, ctx, "exp")?)?;
        let params = match map_get(map, ctx, "params") {
            Err(_) => [].into(),
            Ok(p) => p.value_str_list(),
        };
        Ok(Rule::new(&name, &params, exp))
    }

    pub fn parse_exp(&self, tree: &Tree) -> CompileResult<Exp> {
        let (typename, tree) = parse_node(tree)?;
        eprintln!("EXP {} {:?}", typename, tree);
        let exp: Exp = match &*typename {
            "Alert" => Exp::nil(),
            "BasedRule" => Exp::nil(),
            "Box" => Exp::nil(),
            "Call" => Exp::nil(),
            "Choice" => Exp::nil(),
            "Closure" => Exp::nil(),
            "Comment" => Exp::nil(),
            "Constant" => Exp::nil(),
            "Cut" => Exp::nil(),
            "Dot" => Exp::nil(),
            "EOF" => Exp::nil(),
            "EOLComment" => Exp::nil(),
            "EmptyClosure" => Exp::nil(),
            "Fail" => Exp::nil(),
            "Gather" => Exp::nil(),
            "Grammar" => Exp::nil(),
            "GrammarSemantics" => Exp::nil(),
            "Group" => Exp::nil(),
            "Join" => Exp::nil(),
            "LeftJoin" => Exp::nil(),
            "Lookahead" => Exp::nil(),
            "Model" => Exp::nil(),
            "ModelContext" => Exp::nil(),
            "NULL" => Exp::nil(),
            "Named" => Exp::nil(),
            "NamedBox" => Exp::nil(),
            "NamedList" => Exp::nil(),
            "NegativeLookahead" => Exp::nil(),
            "Option" => Exp::nil(),
            "Optional" => Exp::nil(),
            "Override" => Exp::nil(),
            "OverrideList" => Exp::nil(),
            "Pattern" => Exp::nil(),
            "Patterns" => Exp::nil(),
            "PositiveClosure" => Exp::nil(),
            "PositiveGather" => Exp::nil(),
            "PositiveJoin" => Exp::nil(),
            "RightJoin" => Exp::nil(),
            "Rule" => Exp::nil(),
            "RuleInclude" => Exp::nil(),
            "Sequence" => Exp::nil(),
            "SkipGroup" => Exp::nil(),
            "SkipTo" => Exp::nil(),
            "Synth" => Exp::nil(),
            "Token" => Exp::token(&map_get(tree, "exp", "token")?.value()),
            "Void" => Exp::nil(),
            _ => return Err(CompileError::UnknownExpressionType(typename)),
        };
        Ok(exp)
    }
}

impl Grammar {
    pub fn compile(tree: &Tree) -> CompileResult<Self> {
        let mut compiler = GrammarCompiler::new();
        compiler.compile_grammar(tree)
    }
}
