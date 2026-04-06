// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::tatsu::TatSuModel;
use crate::peg::exp::{ERef, Exp};
use crate::peg::grammar::Grammar;
use crate::peg::rule::{Rule, RuleMap};
use std::collections::HashMap;

impl From<TatSuModel> for ERef {
    fn from(model: TatSuModel) -> Self {
        ERef::new(Exp::from(model))
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

        // println!("{:?}", peg);
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
                    let rhs: Exp = (*exp).into();
                    // let rule = Rule::new(&name, params, rhs);
                    let rule = Rule {
                        name,
                        params,
                        exp: rhs,
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

impl From<TatSuModel> for Exp {
    fn from(model: TatSuModel) -> Self {
        match model {
            TatSuModel::Grammar { .. } => {
                unreachable!("Container types (Rule/Grammar) cannot be nested inside expressions.");
            }
            TatSuModel::Rule { .. } => {
                unreachable!("Container types (Rule/Grammar) cannot be nested inside expressions.");
            }
            TatSuModel::RuleInclude { name, exp } => Exp::rule_include(&name, (*exp).into()),
            TatSuModel::LeftJoin { .. } => unreachable!("LeftJoin not implemented"),
            TatSuModel::RightJoin { .. } => unreachable!("RightJoin not implemented"),

            // --- Core Terminals ---
            TatSuModel::Cut => Exp::cut(),
            TatSuModel::EOF => Exp::eof(),
            TatSuModel::Void { .. } => Exp::void(),

            // --- Calls and Tokens ---
            TatSuModel::Call { name, .. } => Exp::call(name.as_str(), Exp::nil()),
            TatSuModel::Token { token } => Exp::token(token.as_str()),
            TatSuModel::Pattern { pattern } => Exp::pattern(pattern.as_str()),
            TatSuModel::Constant { literal } => Exp::constant(literal.to_string().as_str()),
            TatSuModel::Alert { literal, level } => Exp::alert(literal.as_str().unwrap(), level),

            // --- Unary Operators ---
            TatSuModel::Group { exp } => Exp::group((*exp).into()),
            TatSuModel::Optional { exp } => Exp::optional((*exp).into()),
            TatSuModel::Option { exp } => Exp::alt((*exp).into()),
            TatSuModel::Closure { exp } => Exp::closure((*exp).into()),
            TatSuModel::PositiveClosure { exp } => Exp::positive_closure((*exp).into()),

            // --- Lookahead ---
            TatSuModel::Lookahead { exp } => Exp::lookahead((*exp).into()),
            TatSuModel::NegativeLookahead { exp } => Exp::negative_lookahead((*exp).into()),
            TatSuModel::SkipTo { exp } => Exp::skip_to((*exp).into()),

            // --- N-ary Operators ---
            TatSuModel::Sequence { sequence } => {
                let exprs: Vec<Exp> = sequence.into_iter().map(|m| m.into()).collect();
                Exp::sequence(exprs.as_slice().into())
            }
            TatSuModel::Choice { options } => {
                let exprs: Vec<Exp> = options.into_iter().map(|m| m.into()).collect();
                Exp::choice(exprs.as_slice().into())
            }

            // --- Joins and Gathers ---
            TatSuModel::Join { exp, sep } => Exp::join((*exp).into(), (*sep).into()),
            TatSuModel::PositiveJoin { exp, sep } => {
                Exp::positive_join((*exp).into(), (*sep).into())
            }
            TatSuModel::Gather { exp, sep } => Exp::gather((*exp).into(), (*sep).into()),
            TatSuModel::PositiveGather { exp, sep } => {
                Exp::positive_gather((*exp).into(), (*sep).into())
            }
            TatSuModel::Named { name, exp } => Exp::named(name.as_str(), (*exp).into()),
            TatSuModel::NamedList { name, exp } => Exp::named_list(name.as_str(), (*exp).into()),
            TatSuModel::Override { exp } => Exp::override_node((*exp).into()),
            TatSuModel::OverrideList { exp } => Exp::override_list((*exp).into()),
            TatSuModel::SkipGroup { exp } => Exp::skip_group((*exp).into()),
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
