// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::json::error::ImportError;
use crate::json::tatsu::TatSuModel;
use crate::peg::exp::{Exp, ExpKind};
use crate::peg::grammar::Grammar;
use std::collections::HashMap;

// impl Grammar {
//     pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
//         let mut deserializer = serde_json::Deserializer::from_str(json);
//
//         let model: TatSuModel =
//             serde_path_to_error::deserialize(&mut deserializer).map_err(|err| {
//                 format!("JSON error at {}: {}", err.path(), err)
//             })?;
//
//         let grammar = Self::try_from(model)?;
//         Ok(grammar)
//     }
// }

impl TryFrom<Grammar> for TatSuModel {
    type Error = ImportError;

    fn try_from(grammar: Grammar) -> Result<Self, Self::Error> {
        let rules: Vec<TatSuModel> = grammar
            .rules()
            .map(|r| {
                let rule = r;
                TatSuModel::Rule {
                    name: rule.meta.name.clone().into(),
                    params: rule.meta.params.iter().map(|p| p.clone().into()).collect(),
                    exp: TatSuModel::from(rule.exp.clone()).into(),
                    is_name: rule.is_identifier(),
                    is_tokn: rule.has_token_flag(),
                    no_memo: rule.has_no_memo_flag(),
                    is_memo: rule.has_memo_flag(),
                    is_lrec: rule.has_left_recursion_flag(),
                }
            })
            .collect();

        let directives: HashMap<String, serde_json::Value> = grammar
            .directives
            .iter()
            .map(|(k, v)| (k.clone(), serde_json::Value::String(v.into())))
            .collect();

        Ok(TatSuModel::Grammar {
            name: grammar.name.as_str().into(),
            rules,
            directives,
            keywords: grammar.keywords,
            analyzed: grammar.analyzed,
        })
    }
}

impl From<Exp> for TatSuModel {
    fn from(exp: Exp) -> Self {
        match exp.kind {
            ExpKind::Nil => TatSuModel::Void,
            ExpKind::Cut => TatSuModel::Cut,
            ExpKind::Void => TatSuModel::Void,
            ExpKind::Eof => TatSuModel::EOF,
            ExpKind::Dot => TatSuModel::Pattern {
                pattern: ".".to_string(),
            },
            ExpKind::Call { name, .. } => TatSuModel::Call { name: name.into() },
            ExpKind::Token(s) => TatSuModel::Token { token: s.into() },
            ExpKind::Pattern(s) => TatSuModel::Pattern { pattern: s.into() },
            ExpKind::Constant(s) => TatSuModel::Constant {
                literal: serde_json::Value::String(s.into()),
            },
            ExpKind::Alert(s, level) => TatSuModel::Alert {
                literal: serde_json::Value::String(s.into()),
                level,
            },
            ExpKind::Named(name, exp) => TatSuModel::Named {
                name: name.into(),
                exp: TatSuModel::from(*exp).into(),
            },
            ExpKind::NamedList(name, exp) => TatSuModel::NamedList {
                name: name.into(),
                exp: TatSuModel::from(*exp).into(),
            },
            ExpKind::Override(exp) => TatSuModel::Override {
                exp: TatSuModel::from(*exp).into(),
            },
            ExpKind::OverrideList(exp) => TatSuModel::OverrideList {
                exp: TatSuModel::from(*exp).into(),
            },
            ExpKind::Group(exp) => TatSuModel::Group {
                exp: TatSuModel::from(*exp).into(),
            },
            ExpKind::SkipGroup(exp) => TatSuModel::SkipGroup {
                exp: TatSuModel::from(*exp).into(),
            },
            ExpKind::Lookahead(exp) => TatSuModel::Lookahead {
                exp: TatSuModel::from(*exp).into(),
            },
            ExpKind::NegativeLookahead(exp) => TatSuModel::NegativeLookahead {
                exp: TatSuModel::from(*exp).into(),
            },
            ExpKind::SkipTo(exp) => TatSuModel::SkipTo {
                exp: TatSuModel::from(*exp).into(),
            },
            ExpKind::Sequence(sequence) => TatSuModel::Sequence {
                sequence: sequence
                    .iter()
                    .map(|r| TatSuModel::from(r.clone()))
                    .collect(),
            },
            ExpKind::Choice(options) => TatSuModel::Choice {
                options: options
                    .iter()
                    .map(|r| TatSuModel::from(r.clone()))
                    .collect(),
            },
            ExpKind::Alt(exp) => TatSuModel::Option {
                exp: TatSuModel::from(*exp).into(),
            },
            ExpKind::Optional(exp) => TatSuModel::Optional {
                exp: TatSuModel::from(*exp).into(),
            },
            ExpKind::Closure(exp) => TatSuModel::Closure {
                exp: TatSuModel::from(*exp).into(),
            },
            ExpKind::PositiveClosure(exp) => TatSuModel::PositiveClosure {
                exp: TatSuModel::from(*exp).into(),
            },
            ExpKind::Join { exp, sep } => TatSuModel::Join {
                exp: TatSuModel::from(*exp).into(),
                sep: TatSuModel::from(*sep).into(),
            },
            ExpKind::PositiveJoin { exp, sep } => TatSuModel::PositiveJoin {
                exp: TatSuModel::from(*exp).into(),
                sep: TatSuModel::from(*sep).into(),
            },
            ExpKind::Gather { exp, sep } => TatSuModel::Gather {
                exp: TatSuModel::from(*exp).into(),
                sep: TatSuModel::from(*sep).into(),
            },
            ExpKind::PositiveGather { exp, sep } => TatSuModel::PositiveGather {
                exp: TatSuModel::from(*exp).into(),
                sep: TatSuModel::from(*sep).into(),
            },
            ExpKind::RuleInclude { name, exp } => TatSuModel::RuleInclude {
                name: name.into(),
                exp: exp
                    .map_or(TatSuModel::Void, |e| TatSuModel::from(*e))
                    .into(),
            },
            _ => unreachable!("Conversion for variant not implemented"),
        }
    }
}

// impl From<TatSuModel> for Exp {
//     fn from(model: TatSuModel) -> Self {
//         match model {
//             TatSuModel::Grammar { .. } | TatSuModel::Rule { .. } => {
//                 unreachable!("Container types cannot be nested inside expressions.");
//             }
//             TatSuModel::RuleInclude { name, exp } => Exp::rule_include(&name, (*exp).into()),
//             TatSuModel::LeftJoin { .. } | TatSuModel::RightJoin { .. } => {
//                 unreachable!("Join types not implemented")
//             }
//             TatSuModel::Cut => Exp::cut(),
//             TatSuModel::EOF => Exp::eof(),
//             TatSuModel::Void { .. } => Exp::void(),
//             TatSuModel::Call { name, .. } => Exp::call(name.as_str()),
//             TatSuModel::Token { token } => Exp::token(token.as_str()),
//             TatSuModel::Pattern { pattern } => Exp::pattern(pattern.as_str()),
//             TatSuModel::Constant { literal } => Exp::constant(literal.as_str().unwrap_or("")),
//             TatSuModel::Alert { literal, level } => Exp::alert(literal.as_str().unwrap(), level),
//             TatSuModel::Group { exp } => Exp::group((*exp).into()),
//             TatSuModel::Optional { exp } => Exp::optional((*exp).into()),
//             TatSuModel::Option { exp } => Exp::alt((*exp).into()),
//             TatSuModel::Closure { exp } => Exp::closure((*exp).into()),
//             TatSuModel::PositiveClosure { exp } => Exp::positive_closure((*exp).into()),
//             TatSuModel::Lookahead { exp } => Exp::lookahead((*exp).into()),
//             TatSuModel::NegativeLookahead { exp } => Exp::negative_lookahead((*exp).into()),
//             TatSuModel::SkipTo { exp } => Exp::skip_to((*exp).into()),
//             TatSuModel::Sequence { sequence } => {
//                 let exprs: Vec<Exp> = sequence.into_iter().map(|m| m.into()).collect();
//                 Exp::sequence(exprs.as_slice().into())
//             }
//             TatSuModel::Choice { options } => {
//                 let exprs: Vec<Exp> = options.into_iter().map(|m| m.into()).collect();
//                 Exp::choice(exprs.as_slice().into())
//             }
//             TatSuModel::Join { exp, sep } => Exp::join((*exp).into(), (*sep).into()),
//             TatSuModel::PositiveJoin { exp, sep } => {
//                 Exp::positive_join((*exp).into(), (*sep).into())
//             }
//             TatSuModel::Gather { exp, sep } => Exp::gather((*exp).into(), (*sep).into()),
//             TatSuModel::PositiveGather { exp, sep } => {
//                 Exp::positive_gather((*exp).into(), (*sep).into())
//             }
//             TatSuModel::Named { name, exp } => Exp::named(name.as_str(), (*exp).into()),
//             TatSuModel::NamedList { name, exp } => Exp::named_list(name.as_str(), (*exp).into()),
//             TatSuModel::Override { exp } => Exp::override_node((*exp).into()),
//             TatSuModel::OverrideList { exp } => Exp::override_list((*exp).into()),
//             TatSuModel::SkipGroup { exp } => Exp::skip_group((*exp).into()),
//         }
//     }
// }
