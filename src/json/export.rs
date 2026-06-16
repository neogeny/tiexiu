// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Exp - Grammar to serde_json::Value serializer
//!
//! This module serializes Grammar to serde_json::Value,
//! allowing easy tweaking of the output before final serialization.

use crate::cfg::constants::*;
use crate::cfg::*;
use crate::json::error::Result;
use crate::peg::exp::{Exp, ExpKind};
use crate::peg::grammar::Grammar;
use crate::peg::rule::Rule;
use serde_json::{Map, Value};

impl Grammar {
    /// Serializes this grammar into a `Value`.
    pub fn to_json(&self) -> Value {
        let mut obj = Map::new();

        obj.insert("__class__".into(), Value::String("Grammar".into()));
        obj.insert("name".into(), Value::String(self.name.to_string()));

        let dirs = self.get_directives();
        let mut directives = Map::new();
        for opt in dirs.iter() {
            match opt {
                CfgKey::Grammar(name) => {
                    directives.insert(STR_GRAMMAR_NAME.into(), Value::String(name.to_string()));
                }
                CfgKey::Wsp(p) => {
                    directives.insert(STR_WHITESPACE.into(), Value::String(p.to_string()));
                }
                CfgKey::Cmt(p) => {
                    directives.insert(STR_COMMENTS.into(), Value::String(p.to_string()));
                }
                CfgKey::Eol(p) => {
                    directives.insert(STR_EOL_COMMENTS.into(), Value::String(p.to_string()));
                }
                CfgKey::NameChars(p) => {
                    directives.insert(STR_NAMECHARS.into(), Value::String(p.to_string()));
                }
                CfgKey::IgnoreCase => {
                    directives.insert(STR_IGNORECASE.into(), Value::Bool(true));
                }
                CfgKey::NoIgnoreCase => {
                    directives.insert(STR_IGNORECASE.into(), Value::Bool(false));
                }
                CfgKey::NameGuard(v) => {
                    directives.insert(STR_NAMEGUARD.into(), Value::Bool(*v));
                }
                CfgKey::NoLeftRecursion => {
                    directives.insert(STR_LEFTREC.into(), Value::Bool(false));
                }
                CfgKey::NoParseInfo => {
                    directives.insert(STR_PARSEINFO.into(), Value::Bool(false));
                }
                CfgKey::NoMemoization => {
                    directives.insert(STR_MEMOIZATION.into(), Value::Bool(false));
                }
                _ => {}
            };
        }
        obj.insert("directives".into(), Value::Object(directives));

        let keywords: Vec<Value> = self
            .keywords
            .iter()
            .map(|k| Value::String(k.to_string()))
            .collect();
        obj.insert("keywords".into(), Value::Array(keywords));

        let rules: Vec<Value> = self.rules().map(|r| r.to_json()).collect();
        obj.insert("rules".into(), Value::Array(rules));

        Value::Object(obj)
    }

    /// Serializes the grammar to a compact JSON string.
    pub fn to_json_str(&self) -> Result<Box<str>> {
        Ok(serde_json::to_string(&self.to_json())?.into())
    }

    /// Serializes the grammar to a pretty-printed JSON string.
    pub fn to_json_string(&self) -> Result<String> {
        let mut s = serde_json::to_string_pretty(&self.to_json())?;
        s.push('\n');
        Ok(s)
    }
}

impl Rule {
    /// Serializes this rule into a `Value`.
    pub fn to_json(&self) -> Value {
        let mut obj = Map::new();

        obj.insert("__class__".into(), Value::String("Rule".into()));
        obj.insert("name".into(), Value::String(self.name.to_string()));

        obj.insert("exp".into(), self.exp.to_json());

        let params: Vec<Value> = self
            .params
            .iter()
            .map(|p| Value::String(p.to_string()))
            .collect();
        obj.insert("params".into(), Value::Array(params));

        obj.insert("kwparams".into(), Value::Object(Map::new()));
        obj.insert(
            "decorators".into(),
            Value::Array(
                self.decorators
                    .iter()
                    .map(|d| Value::String(d.to_string()))
                    .collect(),
            ),
        );
        obj.insert("base".into(), Value::Null);

        obj.insert("is_name".into(), Value::Bool(self.is_name()));
        obj.insert("is_tokn".into(), Value::Bool(self.has_is_tokn_flag()));
        obj.insert("no_memo".into(), Value::Bool(self.has_no_memo_flag()));
        obj.insert("no_stak".into(), Value::Bool(self.has_no_stak_flag()));
        obj.insert("is_memo".into(), Value::Bool(self.has_is_memo_flag()));
        obj.insert("is_lrec".into(), Value::Bool(self.has_is_lrec_flag()));

        Value::Object(obj)
    }
}

impl Exp {
    /// Serializes this expression into a `Value`.
    pub fn to_json(&self) -> Value {
        self.kind.to_json_value()
    }
}

impl ExpKind {
    /// Serializes this expression kind into a `Value`.
    pub fn to_json_value(&self) -> Value {
        let mut obj = Map::new();
        let tag = TATSU_TYPE_TAG.to_string();

        match self {
            Self::EmptyClosure => {
                obj.insert(tag, Value::String("EmptyClosure".into()));
                obj.insert("ast".into(), Value::Array(vec![]));
            }
            ExpKind::Nil | ExpKind::Void => {
                obj.insert(tag, Value::String("Void".into()));
                obj.insert("ast".into(), Value::String("()".into()));
            }
            ExpKind::Fail => {
                obj.insert(tag, Value::String("Fail".into()));
            }
            ExpKind::Dot => {
                obj.insert(tag, Value::String("Dot".into()));
            }
            ExpKind::Call { name, .. } => {
                obj.insert(tag, Value::String("Call".into()));
                obj.insert("name".into(), Value::String(name.to_string()));
            }
            ExpKind::Token(s) => {
                obj.insert(tag, Value::String("Token".into()));
                obj.insert("token".into(), Value::String(s.to_string()));
            }
            ExpKind::Pattern(s) => {
                obj.insert(tag, Value::String("Pattern".into()));
                obj.insert("pattern".into(), Value::String(s.to_string()));
            }
            ExpKind::Constant(s) => {
                obj.insert(tag, Value::String("Constant".into()));
                obj.insert("literal".into(), Value::String(s.to_string()));
            }
            ExpKind::Alert(s, level) => {
                obj.insert(tag, Value::String("Alert".into()));
                obj.insert("literal".into(), Value::String(s.to_string()));
                obj.insert("level".into(), Value::Number((*level).into()));
            }
            ExpKind::Named(name, inner) => {
                obj.insert(tag, Value::String("Named".into()));
                obj.insert("name".into(), Value::String(name.to_string()));
                obj.insert("exp".into(), inner.to_json());
            }
            ExpKind::NamedList(name, inner) => {
                obj.insert(tag, Value::String("NamedList".into()));
                obj.insert("name".into(), Value::String(name.to_string()));
                obj.insert("exp".into(), inner.to_json());
            }
            ExpKind::Override(inner) => {
                obj.insert(tag, Value::String("Override".into()));
                obj.insert("exp".into(), inner.to_json());
            }
            ExpKind::OverrideList(inner) => {
                obj.insert(tag, Value::String("OverrideList".into()));
                obj.insert("exp".into(), inner.to_json());
            }
            ExpKind::Group(inner) => {
                obj.insert(tag, Value::String("Group".into()));
                obj.insert("exp".into(), inner.to_json());
            }
            ExpKind::SkipGroup(inner) => {
                obj.insert(tag, Value::String("SkipGroup".into()));
                obj.insert("exp".into(), inner.to_json());
            }
            ExpKind::Lookahead(inner) => {
                obj.insert(tag, Value::String("Lookahead".into()));
                obj.insert("exp".into(), inner.to_json());
            }
            ExpKind::NegativeLookahead(inner) => {
                obj.insert(tag, Value::String("NegativeLookahead".into()));
                obj.insert("exp".into(), inner.to_json());
            }
            ExpKind::SkipTo(inner) => {
                obj.insert(tag, Value::String("SkipTo".into()));
                obj.insert("exp".into(), inner.to_json());
            }
            ExpKind::Sequence(arr) => {
                obj.insert(tag, Value::String("Sequence".into()));
                let seq: Vec<Value> = arr.iter().map(|e| e.to_json()).collect();
                obj.insert("sequence".into(), Value::Array(seq));
            }
            ExpKind::Choice(arr) => {
                obj.insert(tag, Value::String("Choice".into()));
                let opts: Vec<Value> = arr.iter().map(|e| e.to_json()).collect();
                obj.insert("options".into(), Value::Array(opts));
            }
            ExpKind::Alt(inner) => {
                obj.insert(tag, Value::String("Option".into()));
                obj.insert("exp".into(), inner.to_json());
            }
            ExpKind::Optional(inner) => {
                obj.insert(tag, Value::String("Optional".into()));
                obj.insert("exp".into(), inner.to_json());
            }
            ExpKind::Closure(inner) => {
                obj.insert(tag, Value::String("Closure".into()));
                obj.insert("exp".into(), inner.to_json());
            }
            ExpKind::PositiveClosure(inner) => {
                obj.insert(tag, Value::String("PositiveClosure".into()));
                obj.insert("exp".into(), inner.to_json());
            }
            ExpKind::Join { exp, sep } => {
                obj.insert(tag, Value::String("Join".into()));
                obj.insert("exp".into(), exp.to_json());
                obj.insert("sep".into(), sep.to_json());
            }
            ExpKind::PositiveJoin { exp, sep } => {
                obj.insert(tag, Value::String("PositiveJoin".into()));
                obj.insert("exp".into(), exp.to_json());
                obj.insert("sep".into(), sep.to_json());
            }
            ExpKind::Gather { exp, sep } => {
                obj.insert(tag, Value::String("Gather".into()));
                obj.insert("exp".into(), exp.to_json());
                obj.insert("sep".into(), sep.to_json());
            }
            ExpKind::PositiveGather { exp, sep } => {
                obj.insert(tag, Value::String("PositiveGather".into()));
                obj.insert("exp".into(), exp.to_json());
                obj.insert("sep".into(), sep.to_json());
            }
            ExpKind::RuleInclude { name, exp: _ } => {
                obj.insert(tag, Value::String("RuleInclude".into()));
                obj.insert("name".into(), Value::String(name.to_string()));
            }
            ExpKind::Eof => {
                obj.insert(tag, Value::String("EOF".into()));
            }
            ExpKind::Eol => {
                obj.insert(tag, Value::String("EOL".into()));
            }
            ExpKind::Cut => {
                obj.insert(tag, Value::String("Cut".into()));
            }
            ExpKind::NameMeta => {
                obj.insert(tag, Value::String("NameMeta".into()));
            }
            ExpKind::IntMeta => {
                obj.insert(tag, Value::String("IntMeta".into()));
            }
            ExpKind::UIntMeta => {
                obj.insert(tag, Value::String("UIntMeta".into()));
            }
            ExpKind::FloatMeta => {
                obj.insert(tag, Value::String("FloatMeta".into()));
            }
            ExpKind::BoolMeta => {
                obj.insert(tag, Value::String("BoolMeta".into()));
            }
        }

        Value::Object(obj)
    }
}

#[cfg(test)]
mod tests {
    use crate::Grammar;

    #[test]
    fn test_grammar_to_json_value() {
        let json_str = include_str!("../../grammar/tatsu.json");
        let value: serde_json::Value =
            serde_json::from_str(json_str).expect("Failed to parse JSON");
        let grammar = Grammar::from_json_value(&value).expect("Failed to convert");
        let output = grammar.to_json();

        assert!(output.is_object());
        assert!(output["__class__"].is_string());
        assert!(output["name"].is_string());
        assert!(output["rules"].is_array());
    }
}
