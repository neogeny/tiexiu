// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::ExpKind;
use crate::api::error::{CompileError, CompileResult};
use crate::cfg::*;
use crate::peg::grammar::KeywordRef;
use crate::peg::rule::{RuleMap, RuleRef};
use crate::peg::{Exp, Grammar, Rule};
use crate::trees::{Tree, TreeMap, TreeRef};
use crate::types::Str;

/// Compiles a parse tree (from the PEG grammar) into a compiled `Grammar`.
#[derive(Debug, Default)]
pub(crate) struct GrammarCompiler {}

fn parse_node(node: &TreeRef) -> CompileResult<(Str, TreeRef)> {
    let Tree::Node { typename, tree } = node.as_ref() else {
        return Err(CompileError::ExpectedNode(format!("{:?}", node)));
    };
    Ok((typename.clone(), tree.clone()))
}

fn parse_node_check(node: &TreeRef, typename: &'static str) -> CompileResult<TreeRef> {
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

fn _parse_list(node: &TreeRef) -> CompileResult<&[TreeRef]> {
    match node.as_ref() {
        Tree::Seq(list) | Tree::List(list) => Ok(list),
        _ => Err(CompileError::ExpectedList(format!("{:?}", node))),
    }
}

fn map_get<'m>(map: &'m TreeRef, context: &'m str, key: &'static str) -> CompileResult<TreeRef> {
    match map.get(key) {
        Some(node) => Ok(node),
        None => Err(CompileError::MissingKey {
            context: context.into(),
            key,
            tree: map.clone(),
        }),
    }
}

fn map_get_default(map: &Tree, key: &'static str, default: &'static str) -> Str {
    match map.get(key) {
        Some(node) => node.value(),
        None => default.into(),
    }
}

impl Grammar {
    /// Compiles a parse tree into a `Grammar`, running analysis and initialization.
    pub fn compile(tree: &TreeRef, cfga: &CfgA) -> CompileResult<Self> {
        let mut compiler = GrammarCompiler::new();
        compiler.compile_grammar(tree, cfga)
    }
}

impl GrammarCompiler {
    /// Creates a new `GrammarCompiler`.
    pub fn new() -> GrammarCompiler {
        GrammarCompiler {}
    }

    /// Compiles a grammar parse tree into a `Grammar`, running full initialization.
    pub fn compile_grammar(&mut self, tree: &TreeRef, cfga: &CfgA) -> CompileResult<Grammar> {
        let cfg = config(cfga);
        let _debug = cfg.contains(&CfgKey::Debug);
        let map = parse_node_check(tree, "Grammar")?;

        let mut rulemap: RuleMap = RuleMap::new();
        let rule_trees = map_get(&map, "Grammar", "rules")?.list_value();
        for rtree in rule_trees.iter() {
            let rule = self.compile_rule(rtree)?;
            rulemap.insert(rule.name.clone(), rule.into());
        }

        let rules: Vec<RuleRef> = rulemap.into_iter().map(|(_, r)| r).collect();
        let name = map_get_default(&map, "name", "__COMPILED__");

        let mut directives = Cfg::default();
        if let Ok(directives_tree) = map_get(&map, "Grammar", "directives") {
            let directives_list = _parse_list(&directives_tree)?;
            let mut str_directives = Vec::new();
            for d in directives_list.iter() {
                let dm = _parse_map(d)?;
                let name = dm
                    .get("name")
                    .ok_or_else(|| CompileError::MissingKey {
                        context: "Directive".into(),
                        key: "name",
                        tree: d.clone(),
                    })?
                    .value();
                let value = dm
                    .get("value")
                    .ok_or_else(|| CompileError::MissingKey {
                        context: "Directive".into(),
                        key: "value",
                        tree: d.clone(),
                    })?
                    .value();
                str_directives.push((name.to_string(), value.to_string()));
            }
            directives = str_directives
                .iter()
                .filter_map(|(k, v)| Cfg::map(k, v))
                .collect();
        }
        let keywords: Vec<KeywordRef> =
            if let Ok(keywords_tree) = map_get(&map, "Grammar", "keywords") {
                let keywords_nested = _parse_list(&keywords_tree)?;
                let mut keywords = Vec::new();
                for nested_list in keywords_nested.iter() {
                    let inner_list = _parse_list(nested_list)?;
                    for kw in inner_list.iter() {
                        let value: Str = match kw.as_ref() {
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
        grammar.initialize()?;
        Ok(grammar)
    }

    /// Compiles a single rule from its parse tree representation.
    pub fn compile_rule(&self, tree: &TreeRef) -> CompileResult<Rule> {
        let ctx = "Rule";
        let map = parse_node_check(tree, ctx)?;
        let name = map_get(&map, ctx, "name")?.value().to_string();

        let exp = self.parse_exp(&map_get(&map, ctx, "exp")?)?;
        let params: Vec<String> = match map_get(&map, ctx, "params") {
            Err(_) => Vec::new(),
            Ok(p) => p.str_list_value().iter().map(|s| s.to_string()).collect(),
        };
        let decorators: Vec<String> = match map_get(&map, ctx, "decorators") {
            Err(_) => Vec::new(),
            Ok(d) => d.str_list_value().iter().map(|s| s.to_string()).collect(),
        };

        // TatSu decorator → flag mapping (see _tatsu.ebnf:86,
        // tatsu/contexts/decorator/basic.py)
        let is_name = decorators.iter().any(|d| d == "name" || d == "isname");
        let no_memo = decorators.iter().any(|d| d == "nomemo");
        let no_stak = decorators.iter().any(|d| d == "nostak");
        // TatSu heuristic: uppercase rule name → is_tokn (model.py:295)
        let is_tokn = name
            .trim_start_matches('_')
            .chars()
            .next()
            .is_some_and(|c| c.is_uppercase());

        Ok(Rule::from_parts(
            name, params, decorators, exp, is_name, is_tokn, true,  // is_memo
            false, // is_lrec
            no_memo, no_stak,
        ))
    }

    /// Recursively compiles an expression from its parse tree node.
    pub fn parse_exp(&self, tree: &TreeRef) -> CompileResult<Exp> {
        let (typename, tree) = parse_node(tree)?;
        let typename = typename.to_string();
        let exp: Exp = match typename.as_str() {
            "Alert" => {
                let msgtree = map_get(&tree, &typename, "message")?;
                let msgexp = self.parse_exp(&msgtree)?;
                if let ExpKind::Constant(m) = msgexp.kind {
                    Exp::alert(&m, 0)
                } else {
                    Exp::alert(&msgexp.to_string(), 0)
                }
            }
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
            "Option" => Exp::alt(self.parse_exp(&tree)?),
            "Closure" => Exp::closure(self.parse_exp(&tree)?),
            "Comment" => Exp::nil(),
            "Constant" => Exp::constant(&tree.value()),
            "Cut" => Exp::cut(),
            "Dot" => Exp::dot(),
            "EOF" | "Eof" => Exp::eof(),
            "EOL" | "Eol" => Exp::eol(),
            "EOLComment" => Exp::nil(),
            "EmptyClosure" => Exp::empty_closure(),
            "Fail" => Exp::fail(),
            "Gather" => {
                let exp = map_get(&tree, &typename, "exp")?;
                let sep = map_get(&tree, &typename, "sep")?;
                Exp::gather(self.parse_exp(&exp)?, self.parse_exp(&sep)?)
            }
            "Grammar" => Exp::nil(),
            "Group" => Exp::group(self.parse_exp(&tree)?),
            "Join" => {
                let exp = map_get(&tree, &typename, "exp")?;
                let sep = map_get(&tree, &typename, "sep")?;
                Exp::join(self.parse_exp(&exp)?, self.parse_exp(&sep)?)
            }
            "Lookahead" => Exp::lookahead(self.parse_exp(&tree)?),
            "Model" => Exp::nil(),
            "ModelContext" => Exp::nil(),
            "NULL" => Exp::nil(),
            "Named" => {
                let name = map_get(&tree, &typename, "name")?.value();
                let inner = map_get(&tree, &typename, "exp")?;
                Exp::named(&name, self.parse_exp(&inner)?)
            }
            "NamedBox" => Exp::nil(),
            "NamedList" => {
                let name = map_get(&tree, &typename, "name")?.value();
                let inner = map_get(&tree, &typename, "exp")?;
                Exp::named_list(&name, self.parse_exp(&inner)?)
            }
            "NegativeLookahead" => Exp::negative_lookahead(self.parse_exp(&tree)?),
            "Optional" => Exp::optional(self.parse_exp(&tree)?),
            "Override" => Exp::override_node(self.parse_exp(&tree)?),
            "OverrideList" => Exp::override_list(self.parse_exp(&tree)?),
            "Pattern" => Exp::pattern(&tree.value())?,
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
            "PositiveClosure" => Exp::positive_closure(self.parse_exp(&tree)?),
            "PositiveGather" => {
                let exp = map_get(&tree, &typename, "exp")?;
                let sep = map_get(&tree, &typename, "sep")?;
                Exp::positive_gather(self.parse_exp(&exp)?, self.parse_exp(&sep)?)
            }
            "PositiveJoin" | "RightJoin" | "LeftJoin" => {
                let exp = map_get(&tree, &typename, "exp")?;
                let sep = map_get(&tree, &typename, "sep")?;
                Exp::positive_join(self.parse_exp(&exp)?, self.parse_exp(&sep)?)
            }
            "Rule" => Exp::nil(),
            "RuleInclude" => {
                let name = tree.value();
                Exp::rule_include(name.as_ref())
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
                if exps.len() == 1 {
                    exps.into_iter().next().unwrap()
                } else {
                    Exp::sequence(exps)
                }
            }
            "SkipGroup" => Exp::skip_group(self.parse_exp(&tree)?),
            "SkipTo" => Exp::skip_to(self.parse_exp(&tree)?),
            "Synth" => Exp::nil(),
            "NameMeta" => Exp::name_meta(),
            "IntMeta" => Exp::int_meta(),
            "UIntMeta" => Exp::uint_meta(),
            "FloatMeta" => Exp::float_meta(),
            "BoolMeta" => Exp::bool_meta(),

            "Token" => Exp::token(&tree.value()),
            "Void" => Exp::void(),
            _ => return Err(CompileError::UnknownExpressionType(typename.into())),
        };
        Ok(exp)
    }
}
