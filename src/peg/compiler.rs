// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::error::{CompileError, CompileResult};
use super::{Exp, Grammar, Rule};
use crate::peg::grammar::{GrammarDirectives, KeywordRef};
use crate::peg::rule::{RuleMap, RuleRef};
use crate::trees::{FlagMap, Tree, TreeMap};

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
        Tree::Seq(list) | Tree::Closed(list) => Ok(list),
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
        let map = parse_node_check(tree, "Grammar")?;

        let mut rulemap: RuleMap = RuleMap::new();
        let rule_trees = map_get(map, "Grammar", "rules")?.list_value();
        for rtree in rule_trees {
            let rule = self.compile_rule(&rtree)?;
            rulemap.insert(rule.name.clone(), rule.into());
        }

        let rules: Vec<RuleRef> = rulemap.into_iter().map(|(_, r)| r).collect();
        let name = map_get_default(map, "name", "__COMPILED__");

        let mut directives: GrammarDirectives = GrammarDirectives::new(&[]);
        if let Ok(directives_tree) = map_get(map, "Grammar", "directives") {
            let directives_list = _parse_list(directives_tree)?;
            directives = GrammarDirectives::from_iter(directives_list.iter().map(|d| {
                let dm = _parse_map(d).expect("directive should be a Map");
                let name = dm.get("name").expect("name key").value();
                let value = dm.get("value").expect("value key").value();
                (name.to_string(), value.to_string())
            }));
        }
        let keywords: Vec<KeywordRef> =
            if let Ok(keywords_tree) = map_get(map, "Grammar", "keywords") {
                let keywords_nested = _parse_list(keywords_tree)?;
                let mut keywords = Vec::new();
                for nested_list in keywords_nested.iter() {
                    let inner_list = _parse_list(nested_list)?;
                    for kw in inner_list.iter() {
                        let value: Box<str> = match kw {
                            Tree::Text(t) => t.clone(),
                            Tree::Node { typename, tree } if typename.as_ref() == "Word" => {
                                tree.value()
                            }
                            _ => continue,
                        };
                        keywords.push(value);
                    }
                }
                keywords
            } else {
                Vec::new()
            };

        let mut grammar = Grammar::new(&name, rules.as_slice());
        grammar.set_directives(directives);
        grammar.set_keywords(keywords.as_slice());
        Ok(grammar)
    }

    pub fn compile_rule(&self, tree: &Tree) -> CompileResult<Rule> {
        let ctx = "Rule";
        let map = parse_node_check(tree, ctx)?;
        let name = map_get(map, ctx, "name")?.value();

        let _flags = FlagMap::new();
        let exp = self.parse_exp(map_get(map, ctx, "exp")?)?;
        let params = match map_get(map, ctx, "params") {
            Err(_) => [].into(),
            Ok(p) => p.str_list_value(),
        };
        Ok(Rule::new(&name, &params, exp))
    }

    pub fn parse_exp(&self, tree: &Tree) -> CompileResult<Exp> {
        let (typename, tree) = parse_node(tree)?;
        let exp: Exp = match &*typename {
            "Alert" => Exp::alert(&map_get(tree, "exp", "msg")?.value(), 0),
            "BasedRule" => Exp::nil(),
            "Box" => Exp::nil(),
            "Call" => Exp::call(&tree.value()),
            "Choice" => {
                let items = tree.list_value();
                let exps: Vec<Exp> = items
                    .iter()
                    .map(|t| self.parse_exp(t))
                    .collect::<CompileResult<_>>()?;
                Exp::choice(exps)
            }
            "Option" => self.parse_exp(tree)?,
            "Closure" => {
                let inner = map_get(tree, "exp", "exp")?;
                Exp::closure(self.parse_exp(inner)?)
            }
            "Comment" => Exp::nil(),
            "Constant" => Exp::constant(&tree.value()),
            "Cut" => Exp::cut(),
            "Dot" => Exp::dot(),
            "EOF" | "Eof" => Exp::eof(),
            "EOL" | "Eol" => Exp::eol(),
            "EOLComment" => Exp::nil(),
            "EmptyClosure" => Exp::closure(Exp::nil()),
            "Fail" => Exp::fail(),
            "Gather" => {
                let exp = map_get(tree, "exp", "exp")?;
                let sep = map_get(tree, "exp", "sep")?;
                Exp::gather(self.parse_exp(exp)?, self.parse_exp(sep)?)
            }
            "Grammar" => Exp::nil(),
            "GrammarSemantics" => Exp::nil(),
            "Group" => {
                let inner = map_get(tree, "exp", "exp")?;
                Exp::group(self.parse_exp(inner)?)
            }
            "Join" => {
                let exp = map_get(tree, "exp", "exp")?;
                let sep = map_get(tree, "exp", "sep")?;
                Exp::join(self.parse_exp(exp)?, self.parse_exp(sep)?)
            }
            "LeftJoin" => Exp::nil(),
            "Lookahead" => {
                let inner = map_get(tree, "exp", "exp")?;
                Exp::lookahead(self.parse_exp(inner)?)
            }
            "Model" => Exp::nil(),
            "ModelContext" => Exp::nil(),
            "NULL" => Exp::nil(),
            "Named" => {
                let name = map_get(tree, "exp", "name")?.value();
                let inner = map_get(tree, "exp", "exp")?;
                Exp::named(&name, self.parse_exp(inner)?)
            }
            "NamedBox" => Exp::nil(),
            "NamedList" => {
                let name = map_get(tree, "exp", "name")?.value();
                let inner = map_get(tree, "exp", "exp")?;
                Exp::named_list(&name, self.parse_exp(inner)?)
            }
            "NegativeLookahead" => {
                let inner = map_get(tree, "exp", "exp")?;
                Exp::negative_lookahead(self.parse_exp(inner)?)
            }
            "Optional" => {
                let inner = map_get(tree, "exp", "exp")?;
                Exp::optional(self.parse_exp(inner)?)
            }
            "Override" => Exp::nil(),
            "OverrideList" => Exp::nil(),
            "Pattern" => Exp::pattern(&tree.value()),
            "Patterns" => {
                let items = tree.get_list("tree");
                let exps: Vec<Exp> = items
                    .iter()
                    .map(|t| self.parse_exp(t))
                    .collect::<CompileResult<_>>()?;
                if exps.len() == 1 {
                    exps.into_iter().next().unwrap()
                } else {
                    Exp::choice(exps)
                }
            }
            "PositiveClosure" => {
                let inner = map_get(tree, "exp", "exp")?;
                Exp::positive_closure(self.parse_exp(inner)?)
            }
            "PositiveGather" => {
                let exp = map_get(tree, "exp", "exp")?;
                let sep = map_get(tree, "exp", "sep")?;
                Exp::positive_gather(self.parse_exp(exp)?, self.parse_exp(sep)?)
            }
            "PositiveJoin" => {
                let exp = map_get(tree, "exp", "exp")?;
                let sep = map_get(tree, "exp", "sep")?;
                Exp::positive_join(self.parse_exp(exp)?, self.parse_exp(sep)?)
            }
            "RightJoin" => Exp::nil(),
            "Rule" => Exp::nil(),
            "RuleInclude" => {
                let name = map_get(tree, "exp", "name")?.value();
                Exp::rule_include(&name)
            }
            "Sequence" => {
                let tree_inner = tree
                    .get("tree")
                    .map(|t| t.list_value())
                    .unwrap_or_else(|| tree.list_value());
                let exps: Vec<Exp> = tree_inner
                    .iter()
                    .map(|t| self.parse_exp(t))
                    .collect::<CompileResult<_>>()?;
                Exp::sequence(exps)
            }
            "SkipGroup" => {
                let inner = map_get(tree, "exp", "exp")?;
                Exp::skip_group(self.parse_exp(inner)?)
            }
            "SkipTo" => {
                let inner = map_get(tree, "exp", "exp")?;
                Exp::skip_to(self.parse_exp(inner)?)
            }
            "Synth" => Exp::nil(),
            "Token" => Exp::token(&tree.value()),
            "Void" => Exp::void(),
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
