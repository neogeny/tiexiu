use super::tatsu::TatSuModel;
use crate::model::elements::{E, ERef};
use crate::model::grammar::Grammar;
use crate::model::rule::{Rule, RuleMap};

impl From<TatSuModel> for ERef {
    fn from(model: TatSuModel) -> Self {
        // Wrap the converted E into your smart pointer (Arc/Box/etc)
        ERef::new(E::from(model))
    }
}

impl Grammar {
    pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let model: TatSuModel = serde_json::from_str(json)?;
        let grammar = Self::try_from(model)?;
        Ok(grammar)
    }
}

impl TryFrom<TatSuModel> for Grammar {
    type Error = String;

    fn try_from(model: TatSuModel) -> Result<Self, Self::Error> {
        if let TatSuModel::Grammar { name, rules, .. } = model {
            let mut registry = RuleMap::new();

            for rule_node in rules {
                if let TatSuModel::Rule {
                    name: r_name, exp, ..
                } = rule_node
                {
                    // Convert the 'dumb' expression to a 'smart' E
                    let engine_exp: E = (*exp).into();

                    let rule = Rule::new(&r_name, engine_exp);
                    registry.insert(r_name, rule);
                }
            }

            Ok(Grammar {
                name,
                rulemap: registry,
            })
        } else {
            Err("Root node must be a Grammar".to_string())
        }
    }
}

impl From<TatSuModel> for E {
    fn from(model: TatSuModel) -> Self {
        match model {
            // If we encounter a Rule here, something is wrong!
            TatSuModel::Rule { .. } | TatSuModel::Grammar { .. } => {
                unreachable!("Container types (Rule/Grammar) cannot be nested inside expressions.");
            }
            TatSuModel::LeftJoin { .. } => unreachable!("LeftJoin not implemented"),
            TatSuModel::RightJoin { .. } => unreachable!("LeftJoin not implemented"),

            // --- Core Terminals ---
            TatSuModel::Cut => E::Cut,
            TatSuModel::EOF => E::Eof,
            TatSuModel::Void { .. } => E::Void, // TatSu Void usually wraps an exp, check TieXiu semantics

            // --- Calls and Tokens ---
            TatSuModel::Call { name } => E::Call(name.into()),
            TatSuModel::Token { token } => E::Token(token.into()),
            TatSuModel::Pattern { pattern } => E::Pattern(pattern.into()),
            TatSuModel::Constant { literal } => E::Constant(literal.into()),
            TatSuModel::Alert { literal, level } => E::Alert(literal.into(), level),

            // --- Unary Operators ---
            TatSuModel::Group { exp } => E::Group((*exp).into()),
            TatSuModel::Option { exp } | TatSuModel::Optional { exp } => E::Optional((*exp).into()),
            TatSuModel::Closure { exp } => E::Closure((*exp).into()),
            TatSuModel::PositiveClosure { exp } => E::PositiveClosure((*exp).into()),

            // --- Lookahead ---
            TatSuModel::PositiveLookahead { exp } => E::Lookahead((*exp).into()),
            TatSuModel::NegativeLookahead { exp } => E::NegativeLookahead((*exp).into()),
            TatSuModel::SkipTo { exp } => E::SkipTo((*exp).into()),

            // --- N-ary Operators ---
            TatSuModel::Sequence { sequence } => {
                let exprs: Vec<E> = sequence.into_iter().map(|m| m.into()).collect();
                E::Sequence(exprs.as_slice().into())
            }
            TatSuModel::Choice { options } => {
                let exprs: Vec<E> = options.into_iter().map(|m| m.into()).collect();
                E::Choice(exprs.as_slice().into())
            }

            // --- Joins and Gathers ---
            TatSuModel::Join { exp, sep } => E::Join {
                exp: (*exp).into(),
                sep: (*sep).into(),
            },
            TatSuModel::PositiveJoin { exp, sep } => E::PositiveJoin {
                exp: (*exp).into(),
                sep: (*sep).into(),
            },
            TatSuModel::Gather { exp, sep } => E::Gather {
                exp: (*exp).into(),
                sep: (*sep).into(),
            },
            TatSuModel::PositiveGather { exp, sep } => E::PositiveGather {
                exp: (*exp).into(),
                sep: (*sep).into(),
            },
            TatSuModel::Named { name, exp } => E::Named(name.into(), (*exp).into()),
            TatSuModel::NamedList { name, exp } => E::NamedList(name.into(), (*exp).into()),
            TatSuModel::Override { exp } => E::Override((*exp).into()),
            TatSuModel::OverrideList { exp } => E::OverrideList((*exp).into()),
            TatSuModel::SkipGroup { exp } => E::SkipGroup((*exp).into()),
        }
    }
}
