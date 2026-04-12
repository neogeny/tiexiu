// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Exp - Grammar to serde_json::Value serializer
//!
//! This module serializes Grammar to serde_json::Value,
//! allowing easy tweaking of the output before final serialization.

use crate::peg::exp::{ERef, Exp, ExpKind};
use crate::peg::grammar::Grammar;
use crate::peg::rule::Rule;
use serde_json::{Map, Value};

impl Grammar {
    pub fn to_serde_value(&self) -> Value {
        let mut obj = Map::new();

        obj.insert("__class__".into(), Value::String("Grammar".into()));
        obj.insert("name".into(), Value::String(self.name.clone()));
        obj.insert("analyzed".into(), Value::Bool(self.analyzed));

        let rules: Vec<Value> = self.rules().map(|r| r.to_serde_value()).collect();
        obj.insert("rules".into(), Value::Array(rules));

        let directives: Map<String, Value> = self
            .directives
            .iter()
            .map(|(k, v)| (k.clone(), Value::String(v.clone())))
            .collect();
        obj.insert("directives".into(), Value::Object(directives));

        let keywords: Vec<Value> = self
            .keywords
            .iter()
            .map(|k| Value::String(k.clone()))
            .collect();
        obj.insert("keywords".into(), Value::Array(keywords));

        Value::Object(obj)
    }
}

impl Rule {
    pub fn to_serde_value(&self) -> Value {
        let mut obj = Map::new();

        obj.insert("__class__".into(), Value::String("Rule".into()));
        obj.insert("name".into(), Value::String(self.meta.name.to_string()));

        obj.insert("exp".into(), self.exp.to_serde_value());

        let params: Vec<Value> = self
            .meta
            .params
            .iter()
            .map(|p| Value::String(p.to_string()))
            .collect();
        obj.insert("params".into(), Value::Array(params));

        obj.insert("is_name".into(), Value::Bool(self.is_identifier()));
        obj.insert("is_tokn".into(), Value::Bool(self.has_token_flag()));
        obj.insert("no_memo".into(), Value::Bool(self.has_no_memo_flag()));
        obj.insert("is_memo".into(), Value::Bool(self.has_memo_flag()));
        obj.insert(
            "is_lrec".into(),
            Value::Bool(self.has_left_recursion_flag()),
        );

        Value::Object(obj)
    }
}

impl Exp {
    pub fn to_serde_value(&self) -> Value {
        self.kind.to_serde_value()
    }
}

impl ExpKind {
    pub fn to_serde_value(&self) -> Value {
        let mut obj = Map::new();

        match self {
            ExpKind::Nil | ExpKind::Void | ExpKind::Fail | ExpKind::Dot => {
                obj.insert("__class__".into(), Value::String("Void".into()));
            }
            ExpKind::Call { name, .. } => {
                obj.insert("__class__".into(), Value::String("Call".into()));
                obj.insert("name".into(), Value::String(name.as_ref().to_string()));
            }
            ExpKind::Token(s) => {
                obj.insert("__class__".into(), Value::String("Token".into()));
                obj.insert("token".into(), Value::String(s.as_ref().to_string()));
            }
            ExpKind::Pattern(s) => {
                obj.insert("__class__".into(), Value::String("Pattern".into()));
                obj.insert("pattern".into(), Value::String(s.as_ref().to_string()));
            }
            ExpKind::Constant(s) => {
                obj.insert("__class__".into(), Value::String("Constant".into()));
                obj.insert("literal".into(), Value::String(s.as_ref().to_string()));
            }
            ExpKind::Alert(s, level) => {
                obj.insert("__class__".into(), Value::String("Alert".into()));
                obj.insert("literal".into(), Value::String(s.as_ref().to_string()));
                obj.insert("level".into(), Value::Number((*level).into()));
            }
            ExpKind::Named(name, inner) => {
                obj.insert("__class__".into(), Value::String("Named".into()));
                obj.insert("name".into(), Value::String(name.as_ref().to_string()));
                obj.insert("exp".into(), inner.to_serde_value());
            }
            ExpKind::NamedList(name, inner) => {
                obj.insert("__class__".into(), Value::String("NamedList".into()));
                obj.insert("name".into(), Value::String(name.as_ref().to_string()));
                obj.insert("exp".into(), inner.to_serde_value());
            }
            ExpKind::Override(inner) => {
                obj.insert("__class__".into(), Value::String("Override".into()));
                obj.insert("exp".into(), inner.to_serde_value());
            }
            ExpKind::OverrideList(inner) => {
                obj.insert("__class__".into(), Value::String("OverrideList".into()));
                obj.insert("exp".into(), inner.to_serde_value());
            }
            ExpKind::Group(inner) => {
                obj.insert("__class__".into(), Value::String("Group".into()));
                obj.insert("exp".into(), inner.to_serde_value());
            }
            ExpKind::SkipGroup(inner) => {
                obj.insert("__class__".into(), Value::String("SkipGroup".into()));
                obj.insert("exp".into(), inner.to_serde_value());
            }
            ExpKind::Lookahead(inner) => {
                obj.insert("__class__".into(), Value::String("Lookahead".into()));
                obj.insert("exp".into(), inner.to_serde_value());
            }
            ExpKind::NegativeLookahead(inner) => {
                obj.insert(
                    "__class__".into(),
                    Value::String("NegativeLookahead".into()),
                );
                obj.insert("exp".into(), inner.to_serde_value());
            }
            ExpKind::SkipTo(inner) => {
                obj.insert("__class__".into(), Value::String("SkipTo".into()));
                obj.insert("exp".into(), inner.to_serde_value());
            }
            ExpKind::Sequence(arr) => {
                obj.insert("__class__".into(), Value::String("Sequence".into()));
                let seq: Vec<Value> = arr.iter().map(|e| e.to_serde_value()).collect();
                obj.insert("sequence".into(), Value::Array(seq));
            }
            ExpKind::Choice(arr) => {
                obj.insert("__class__".into(), Value::String("Choice".into()));
                let opts: Vec<Value> = arr.iter().map(|e| e.to_serde_value()).collect();
                obj.insert("options".into(), Value::Array(opts));
            }
            ExpKind::Alt(inner) => {
                obj.insert("__class__".into(), Value::String("Option".into()));
                obj.insert("exp".into(), inner.to_serde_value());
            }
            ExpKind::Optional(inner) => {
                obj.insert("__class__".into(), Value::String("Optional".into()));
                obj.insert("exp".into(), inner.to_serde_value());
            }
            ExpKind::Closure(inner) => {
                obj.insert("__class__".into(), Value::String("Closure".into()));
                obj.insert("exp".into(), inner.to_serde_value());
            }
            ExpKind::PositiveClosure(inner) => {
                obj.insert("__class__".into(), Value::String("PositiveClosure".into()));
                obj.insert("exp".into(), inner.to_serde_value());
            }
            ExpKind::Join { exp, sep } => {
                obj.insert("__class__".into(), Value::String("Join".into()));
                obj.insert("exp".into(), exp.to_serde_value());
                obj.insert("sep".into(), sep.to_serde_value());
            }
            ExpKind::PositiveJoin { exp, sep } => {
                obj.insert("__class__".into(), Value::String("PositiveJoin".into()));
                obj.insert("exp".into(), exp.to_serde_value());
                obj.insert("sep".into(), sep.to_serde_value());
            }
            ExpKind::Gather { exp, sep } => {
                obj.insert("__class__".into(), Value::String("Gather".into()));
                obj.insert("exp".into(), exp.to_serde_value());
                obj.insert("sep".into(), sep.to_serde_value());
            }
            ExpKind::PositiveGather { exp, sep } => {
                obj.insert("__class__".into(), Value::String("PositiveGather".into()));
                obj.insert("exp".into(), exp.to_serde_value());
                obj.insert("sep".into(), sep.to_serde_value());
            }
            ExpKind::RuleInclude { name, exp } => {
                obj.insert("__class__".into(), Value::String("RuleInclude".into()));
                obj.insert("name".into(), Value::String(name.as_ref().to_string()));
                if let Some(inner) = exp {
                    obj.insert("exp".into(), inner.to_serde_value());
                }
            }
            ExpKind::Eof => {
                obj.insert("__class__".into(), Value::String("EOF".into()));
            }
            ExpKind::Cut => {
                obj.insert("__class__".into(), Value::String("Cut".into()));
            }
        }

        Value::Object(obj)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_to_serde_value() {
        let json_str = std::fs::read_to_string("grammar/tatsu.json").expect("tatsu.json missing");
        let value: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");
        let grammar = Grammar::from_serde_value(&value).expect("Failed to convert");
        let output = grammar.to_serde_value();

        assert!(output.is_object());
        let obj = output.as_object().unwrap();
        assert_eq!(obj.get("__class__").unwrap().as_str(), Some("Grammar"));
        assert_eq!(obj.get("name").unwrap().as_str(), Some("TatSu"));
        assert!(obj.contains_key("rules"));
    }
}
