// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::tatsu::TatSuModel;
use crate::model::elements::{ERef, Element};
use crate::model::grammar::Grammar;
use crate::model::rule::{Rule, RuleMap};
use std::collections::HashMap;

impl From<TatSuModel> for ERef {
    fn from(model: TatSuModel) -> Self {
        ERef::new(Element::from(model))
    }
}

impl Grammar {
    pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // let value: serde_json::Value = serde_json::from_str(json).map_err(|e| {
        //     format!("Invalid JSON syntax at line {}, col {}: {}", e.line(), e.column(), e)
        // })?;

        // Debug: If you suspect the JSON structure is wrong, you can inspect 'value' here.
        // println!("DEBUG: Raw JSON Root Type: {:?}", value.as_object().map(|_| "Object").unwrap_or("Other"));
        // println!("DEBUG: Raw JSON Root Type: {:#}", value);

        // Use a Deserializer to track the path to the error
        let mut deserializer = serde_json::Deserializer::from_str(json);

        let model: TatSuModel =
            serde_path_to_error::deserialize(&mut deserializer).map_err(|err| {
                // This provides the path (e.g., "rules[0].name") and the error message
                format!("JSON error at {}: {}", err.path(), err)
            })?;

        // println!("{:?}", model);
        let grammar = Self::try_from(model)?;
        // let grammar = Self::default();
        Ok(grammar)
    }
}

impl TryFrom<TatSuModel> for Grammar {
    type Error = String;

    fn try_from(model: TatSuModel) -> Result<Self, Self::Error> {
        if let TatSuModel::Grammar {
            name,
            rules,
            directives,
            keywords,
            analyzed,
        } = model
        {
            let mut rule_vec: Vec<Rule> = vec![];
            for rule_model in rules {
                if let TatSuModel::Rule {
                    name,
                    params,
                    exp,
                    is_name,
                    is_tokn,
                    no_memo,

                    // NOTE:
                    //  these belong to lef-recursion analysis
                    //  they are here for legacy reasons
                    is_memo,
                    is_lrec,
                } = rule_model
                {
                    let rhs: Element = (*exp).into();
                    // let rule = Rule::new(&name, params, rhs);
                    let rule = Rule {
                        name,
                        params,
                        rhs,
                        is_name,
                        is_tokn,
                        no_memo,

                        is_memo: is_memo && !no_memo,
                        is_lrec,
                    };
                    rule_vec.push(rule);
                }
            }
            let str_directives: HashMap<String, String> = directives
                .iter()
                .map(|(k, v)| {
                    // 1. Clone the key to get an owned String
                    // 2. Use as_str() for strings to avoid double quotes,
                    //    otherwise fall back to to_string() for numbers/bools.
                    let val_str = v
                        .as_str()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| v.to_string());

                    (k.clone(), val_str)
                })
                .collect();
            let mut grammar = Grammar {
                name: name.as_str().into(),
                rules: rule_vec.as_slice().into(),
                analyzed,
                rulemap: RuleMap::new(),
                directives: str_directives,
                keywords,
            };
            grammar.initialize();
            Ok(grammar)
        } else {
            Err("Root node must be a Grammar".into())
        }
    }
}

impl From<TatSuModel> for Element {
    fn from(model: TatSuModel) -> Self {
        match model {
            TatSuModel::Grammar { .. } => {
                unreachable!("Container types (Rule/Grammar) cannot be nested inside expressions.");
            }
            TatSuModel::Rule { .. } => {
                unreachable!("Container types (Rule/Grammar) cannot be nested inside expressions.");
            }
            TatSuModel::RuleInclude { name, exp } => {
                // WARNING: NOT THE CORRECT IMPLEMENTATION
                if let Some(exp) = exp {
                    return Element::RuleInclude {
                        name: name.into(),
                        exp: (*exp).into(),
                    };
                }
                Element::RuleInclude {
                    name: name.into(),
                    exp: Element::Nil.into(),
                }
            }
            TatSuModel::LeftJoin { .. } => unreachable!("LeftJoin not implemented"),
            TatSuModel::RightJoin { .. } => unreachable!("RightJoin not implemented"),

            // --- Core Terminals ---
            TatSuModel::Cut => Element::Cut,
            TatSuModel::EOF => Element::Eof,
            TatSuModel::Void { .. } => Element::Void,

            // --- Calls and Tokens ---
            TatSuModel::Call { name, .. } => {
                // if let Some(exp) = exp {
                //     return Element::Call(name.into(), (*exp).into());
                // }
                Element::Call(name.into(), Element::Nil.into())
            }
            TatSuModel::Token { token } => Element::Token(token.into()),
            TatSuModel::Pattern { pattern } => Element::Pattern(pattern.into()),
            TatSuModel::Constant { literal } => Element::Constant(literal.to_string().into()),
            TatSuModel::Alert { literal, level } => {
                Element::Alert(literal.to_string().into(), level)
            }

            // --- Unary Operators ---
            TatSuModel::Group { exp } => Element::Group((*exp).into()),
            TatSuModel::Optional { exp } => Element::Optional((*exp).into()),
            TatSuModel::Option { exp } => Element::Alt((*exp).into()),
            TatSuModel::Closure { exp } => Element::Closure((*exp).into()),
            TatSuModel::PositiveClosure { exp } => Element::PositiveClosure((*exp).into()),

            // --- Lookahead ---
            TatSuModel::Lookahead { exp } => Element::Lookahead((*exp).into()),
            TatSuModel::NegativeLookahead { exp } => Element::NegativeLookahead((*exp).into()),
            TatSuModel::SkipTo { exp } => Element::SkipTo((*exp).into()),

            // --- N-ary Operators ---
            TatSuModel::Sequence { sequence } => {
                let exprs: Vec<Element> = sequence.into_iter().map(|m| m.into()).collect();
                Element::Sequence(exprs.as_slice().into())
            }
            TatSuModel::Choice { options } => {
                let exprs: Vec<Element> = options.into_iter().map(|m| m.into()).collect();
                Element::Choice(exprs.as_slice().into())
            }

            // --- Joins and Gathers ---
            TatSuModel::Join { exp, sep } => Element::Join {
                exp: (*exp).into(),
                sep: (*sep).into(),
            },
            TatSuModel::PositiveJoin { exp, sep } => Element::PositiveJoin {
                exp: (*exp).into(),
                sep: (*sep).into(),
            },
            TatSuModel::Gather { exp, sep } => Element::Gather {
                exp: (*exp).into(),
                sep: (*sep).into(),
            },
            TatSuModel::PositiveGather { exp, sep } => Element::PositiveGather {
                exp: (*exp).into(),
                sep: (*sep).into(),
            },
            TatSuModel::Named { name, exp } => Element::Named(name.into(), (*exp).into()),
            TatSuModel::NamedList { name, exp } => Element::NamedList(name.into(), (*exp).into()),
            TatSuModel::Override { exp } => Element::Override((*exp).into()),
            TatSuModel::OverrideList { exp } => Element::OverrideList((*exp).into()),
            TatSuModel::SkipGroup { exp } => Element::SkipGroup((*exp).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_from_json() {
        let json = std::fs::read_to_string("grammar/calc.json").expect("calc.json missing");
        println!("CALC FROM JSON");
        println!("{:#}", json);

        let _grammar = Grammar::from_json(&json).unwrap();
    }
}
