// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::json::error::ImportError;
use crate::json::tatsu::TatSuModel;
use crate::peg::exp::{ERef, Exp};
use crate::peg::grammar::Grammar;
use crate::peg::rule::Rule;

impl TryFrom<TatSuModel> for ERef {
    type Error = ImportError;

    fn try_from(model: TatSuModel) -> Result<Self, Self::Error> {
        Ok(ERef::new(Exp::try_from(model)?))
    }
}

impl Grammar {
    pub fn from_json(json: &str) -> Result<Self, ImportError> {
        // #[cfg(debug_assertions)]
        {
            let value: serde_json::Value = serde_json::from_str(json).map_err(ImportError::from)?;

            // Debug: If you suspect the JSON structure is wrong, you can inspect 'value' here.
            assert!(value.is_object());
            // println!(
            //     "DEBUG: Raw JSON Root Type: {:?}",
            //     value.as_object().map(|_| "Object").unwrap_or("Other")
            // );
            println!("DEBUG: Raw JSON Root Type: {:#}", value);
        }

        // Use a Deserializer to track the path to the error
        let mut deserializer = serde_json::Deserializer::from_str(json);

        let model: TatSuModel = serde_path_to_error::deserialize(&mut deserializer)
            .map_err(|err| ImportError::JsonPath(err.path().to_string(), err.into_inner()))?;

        #[cfg(debug_assertions)]
        println!("{:?}", model);

        let grammar = Self::try_from(model)?;
        Ok(grammar)
    }
}

impl TryFrom<TatSuModel> for Grammar {
    type Error = ImportError;

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
                    let rhs = Exp::try_from(*exp)?;
                    let rule = Rule::from_parts(
                        name, params, rhs, is_name, is_tokn, no_memo, is_memo, is_lrec,
                    );
                    rule_vec.push(rule);
                }
            }
            let str_directives: std::collections::HashMap<String, String> = directives
                .iter()
                .map(|(k, v)| {
                    let val_str = v.as_str().map(|s| s.to_string()).unwrap_or(v.to_string());

                    (k.clone(), val_str)
                })
                .collect();
            let mut grammar = Grammar::new(name.as_str(), &rule_vec);
            grammar.analyzed = analyzed;
            grammar.directives = str_directives;
            grammar.keywords = keywords;
            grammar.initialize();
            Ok(grammar)
        } else {
            Err(ImportError::InvalidRoot)
        }
    }
}

impl TryFrom<TatSuModel> for Exp {
    type Error = ImportError;

    fn try_from(model: TatSuModel) -> Result<Self, Self::Error> {
        match model {
            TatSuModel::Grammar { .. } | TatSuModel::Rule { .. } => {
                Err(ImportError::UnsupportedModel(format!("{:?}", model)))
            }
            TatSuModel::RuleInclude { name, exp } => {
                let inner_exp = Exp::try_from(*exp)?;
                Ok(Exp::rule_include_with(&name, inner_exp))
            }
            TatSuModel::LeftJoin { .. } => Err(ImportError::UnsupportedModel("LeftJoin".into())),
            TatSuModel::RightJoin { .. } => Err(ImportError::UnsupportedModel("RightJoin".into())),

            // --- Core Terminals ---
            TatSuModel::Cut => Ok(Exp::cut()),
            TatSuModel::EOF => Ok(Exp::eof()),
            TatSuModel::Void => Ok(Exp::void()),

            // --- Calls and Tokens ---
            TatSuModel::Call { name, .. } => Ok(Exp::call(name.as_str())),
            TatSuModel::Token { token } => Ok(Exp::token(token.as_str())),
            TatSuModel::Pattern { pattern } => Ok(Exp::pattern(pattern.as_str())),
            TatSuModel::Constant { literal } => Ok(Exp::constant(literal.as_str().unwrap_or(""))),
            TatSuModel::Alert { literal, level } => {
                Ok(Exp::alert(literal.as_str().unwrap(), level))
            }

            // --- Unary Operators ---
            TatSuModel::Group { exp } => Ok(Exp::group(Exp::try_from(*exp)?)),
            TatSuModel::Optional { exp } => Ok(Exp::optional(Exp::try_from(*exp)?)),
            TatSuModel::Option { exp } => Ok(Exp::alt(Exp::try_from(*exp)?)),
            TatSuModel::Closure { exp } => Ok(Exp::closure(Exp::try_from(*exp)?)),
            TatSuModel::PositiveClosure { exp } => Ok(Exp::positive_closure(Exp::try_from(*exp)?)),

            // --- Lookahead ---
            TatSuModel::Lookahead { exp } => Ok(Exp::lookahead(Exp::try_from(*exp)?)),
            TatSuModel::NegativeLookahead { exp } => {
                Ok(Exp::negative_lookahead(Exp::try_from(*exp)?))
            }
            TatSuModel::SkipTo { exp } => Ok(Exp::skip_to(Exp::try_from(*exp)?)),

            // --- N-ary Operators ---
            TatSuModel::Sequence { sequence } => {
                let exprs: Result<Vec<Exp>, ImportError> =
                    sequence.into_iter().map(Exp::try_from).collect();
                Ok(Exp::sequence(exprs?.as_slice().into()))
            }
            TatSuModel::Choice { options } => {
                let exprs: Result<Vec<Exp>, ImportError> =
                    options.into_iter().map(Exp::try_from).collect();
                Ok(Exp::choice(exprs?.as_slice().into()))
            }

            // --- Joins and Gathers ---
            TatSuModel::Join { exp, sep } => {
                Ok(Exp::join(Exp::try_from(*exp)?, Exp::try_from(*sep)?))
            }
            TatSuModel::PositiveJoin { exp, sep } => Ok(Exp::positive_join(
                Exp::try_from(*exp)?,
                Exp::try_from(*sep)?,
            )),
            TatSuModel::Gather { exp, sep } => {
                Ok(Exp::gather(Exp::try_from(*exp)?, Exp::try_from(*sep)?))
            }
            TatSuModel::PositiveGather { exp, sep } => Ok(Exp::positive_gather(
                Exp::try_from(*exp)?,
                Exp::try_from(*sep)?,
            )),
            TatSuModel::Named { name, exp } => Ok(Exp::named(name.as_str(), Exp::try_from(*exp)?)),
            TatSuModel::NamedList { name, exp } => {
                Ok(Exp::named_list(name.as_str(), Exp::try_from(*exp)?))
            }
            TatSuModel::Override { exp } => Ok(Exp::override_node(Exp::try_from(*exp)?)),
            TatSuModel::OverrideList { exp } => Ok(Exp::override_list(Exp::try_from(*exp)?)),
            TatSuModel::SkipGroup { exp } => Ok(Exp::skip_group(Exp::try_from(*exp)?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_from_json() {
        let filename = "grammar/calc.json";
        let json = std::fs::read_to_string(filename).expect("calc.json missing");
        println!("CALC FROM JSON");
        println!("{:#}", json);

        let _grammar = Grammar::from_json(&json).unwrap();
    }
}
