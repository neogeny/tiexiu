// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Exp - Grammar to json::JsonValue serializer
//!
//! This module serializes Grammar to json::JsonValue,
//! allowing easy tweaking of the output before final serialization.

use crate::cfg::constants::*;
use crate::cfg::*;
use crate::json::error::Result;
use crate::peg::exp::{Exp, ExpKind};
use crate::peg::grammar::Grammar;
use crate::peg::rule::Rule;
use json::JsonValue;

impl Grammar {
    /// Serializes the grammar to a `serde_json::Value` (requires `serde_json` feature).
    #[cfg(feature = "serde_json")]
    pub fn to_json_serde(&self) -> serde_json::Value {
        let json_str = self.to_json().dump();
        serde_json::from_str(&json_str).unwrap_or(serde_json::Value::Null)
    }

    /// Serializes this grammar into a `JsonValue`.
    pub fn to_json(&self) -> JsonValue {
        let mut obj = JsonValue::new_object();

        obj["__class__"] = JsonValue::String("Grammar".into());
        obj["name"] = JsonValue::String(self.name.to_string());
        obj["analyzed"] = JsonValue::Boolean(self.analyzed);

        let dirs = self.get_directives();
        let mut directives = JsonValue::new_object();
        for opt in dirs.iter() {
            match opt {
                CfgKey::Grammar(name) => directives[STR_GRAMMAR_NAME] = name.to_string().into(),
                CfgKey::Wsp(p) => directives[STR_WHITESPACE] = p.to_string().into(),
                CfgKey::Cmt(p) => directives[STR_COMMENTS] = p.to_string().into(),
                CfgKey::Eol(p) => directives[STR_EOL_COMMENTS] = p.to_string().into(),
                CfgKey::NameChars(p) => directives[STR_NAMECHARS] = p.to_string().into(),
                _ => {}
            };
        }
        directives[STR_IGNORECASE] = dirs.contains(&CfgKey::IgnoreCase).into();
        directives[STR_NAMEGUARD] = dirs.contains(&CfgKey::NameGuard).into();
        directives[STR_LEFTREC] = (!dirs.contains(&CfgKey::NoLeftRecursion)).into();
        directives[STR_PARSEINFO] = (!dirs.contains(&CfgKey::NoParseInfo)).into();
        directives[STR_MEMOIZATION] = (!dirs.contains(&CfgKey::NoMemoization)).into();
        obj["directives"] = directives;

        let keywords: Vec<JsonValue> = self
            .keywords
            .iter()
            .map(|k| JsonValue::String(k.to_string()))
            .collect();
        obj["keywords"] = JsonValue::Array(keywords);

        let rules: Vec<JsonValue> = self.rules().map(|r| r.to_json()).collect();
        obj["rules"] = JsonValue::Array(rules);

        obj
    }

    /// Serializes the grammar to a compact JSON string.
    pub fn to_json_str(&self) -> Result<Box<str>> {
        Ok(self.to_json().dump().into())
    }

    /// Serializes the grammar to a pretty-printed JSON string.
    pub fn to_json_string(&self) -> Result<String> {
        Ok(self.to_json().pretty(2))
    }
}

impl Rule {
    /// Serializes this rule into a `JsonValue`.
    pub fn to_json(&self) -> JsonValue {
        let mut obj = JsonValue::new_object();

        obj["__class__"] = JsonValue::String("Rule".into());
        obj["name"] = JsonValue::String(self.name.to_string());

        let params: Vec<JsonValue> = self
            .params
            .iter()
            .map(|p| JsonValue::String(p.to_string()))
            .collect();
        obj["params"] = JsonValue::Array(params);

        obj["no_memo"] = JsonValue::Boolean(self.has_no_memo_flag());
        obj["no_stak"] = JsonValue::Boolean(self.has_no_stak_flag());
        obj["is_name"] = JsonValue::Boolean(self.is_name());
        obj["is_tokn"] = JsonValue::Boolean(self.has_is_tokn_flag());
        obj["is_memo"] = JsonValue::Boolean(self.has_is_memo_flag());
        obj["is_lrec"] = JsonValue::Boolean(self.has_is_lrec_flag());

        obj["exp"] = self.exp.to_json();

        obj
    }
}

impl Exp {
    /// Serializes this expression into a `JsonValue`.
    pub fn to_json(&self) -> JsonValue {
        self.kind.to_json_value()
    }
}

impl ExpKind {
    /// Serializes this expression kind into a `JsonValue`.
    pub fn to_json_value(&self) -> JsonValue {
        let mut obj = JsonValue::new_object();
        let tag = TATSU_TYPE_TAG.to_string();

        match self {
            Self::EmptyClosure => {
                obj[&tag] = JsonValue::String("EmptyClosure".into());
            }
            ExpKind::Nil | ExpKind::Void => {
                obj[&tag] = JsonValue::String("Void".into());
                obj["ast"] = JsonValue::String("()".into());
            }
            ExpKind::Fail => {
                obj[&tag] = JsonValue::String("Fail".into());
            }
            ExpKind::Dot => {
                obj[&tag] = JsonValue::String("Dot".into());
            }
            ExpKind::Call { name, .. } => {
                obj[&tag] = JsonValue::String("Call".into());
                obj["name"] = JsonValue::String(name.as_ref().to_string());
            }
            ExpKind::Token(s) => {
                obj[&tag] = JsonValue::String("Token".into());
                obj["token"] = JsonValue::String(s.as_ref().to_string());
            }
            ExpKind::Pattern(s) => {
                obj[&tag] = JsonValue::String("Pattern".into());
                obj["pattern"] = JsonValue::String(s.as_ref().to_string());
            }
            ExpKind::Constant(s) => {
                obj[&tag] = JsonValue::String("Constant".into());
                obj["literal"] = JsonValue::String(s.as_ref().to_string());
            }
            ExpKind::Alert(s, level) => {
                obj[&tag] = JsonValue::String("Alert".into());
                obj["literal"] = JsonValue::String(s.as_ref().to_string());
                obj["level"] = JsonValue::Number((*level).into());
            }
            ExpKind::Named(name, inner) => {
                obj[&tag] = JsonValue::String("Named".into());
                obj["name"] = JsonValue::String(name.as_ref().to_string());
                obj["exp"] = inner.to_json();
            }
            ExpKind::NamedList(name, inner) => {
                obj[&tag] = JsonValue::String("NamedList".into());
                obj["name"] = JsonValue::String(name.as_ref().to_string());
                obj["exp"] = inner.to_json();
            }
            ExpKind::Override(inner) => {
                obj[&tag] = JsonValue::String("Override".into());
                obj["exp"] = inner.to_json();
            }
            ExpKind::OverrideList(inner) => {
                obj[&tag] = JsonValue::String("OverrideList".into());
                obj["exp"] = inner.to_json();
            }
            ExpKind::Group(inner) => {
                obj[&tag] = JsonValue::String("Group".into());
                obj["exp"] = inner.to_json();
            }
            ExpKind::SkipGroup(inner) => {
                obj[&tag] = JsonValue::String("SkipGroup".into());
                obj["exp"] = inner.to_json();
            }
            ExpKind::Lookahead(inner) => {
                obj[&tag] = JsonValue::String("Lookahead".into());
                obj["exp"] = inner.to_json();
            }
            ExpKind::NegativeLookahead(inner) => {
                obj[&tag] = JsonValue::String("NegativeLookahead".into());
                obj["exp"] = inner.to_json();
            }
            ExpKind::SkipTo(inner) => {
                obj[&tag] = JsonValue::String("SkipTo".into());
                obj["exp"] = inner.to_json();
            }
            ExpKind::Sequence(arr) => {
                obj[&tag] = JsonValue::String("Sequence".into());
                let seq: Vec<JsonValue> = arr.iter().map(|e| e.to_json()).collect();
                obj["sequence"] = JsonValue::Array(seq);
            }
            ExpKind::Choice(arr) => {
                obj[&tag] = JsonValue::String("Choice".into());
                let opts: Vec<JsonValue> = arr.iter().map(|e| e.to_json()).collect();
                obj["options"] = JsonValue::Array(opts);
            }
            ExpKind::Alt(inner) => {
                obj[&tag] = JsonValue::String("Option".into());
                obj["exp"] = inner.to_json();
            }
            ExpKind::Optional(inner) => {
                obj[&tag] = JsonValue::String("Optional".into());
                obj["exp"] = inner.to_json();
            }
            ExpKind::Closure(inner) => {
                obj[&tag] = JsonValue::String("Closure".into());
                obj["exp"] = inner.to_json();
            }
            ExpKind::PositiveClosure(inner) => {
                obj[&tag] = JsonValue::String("PositiveClosure".into());
                obj["exp"] = inner.to_json();
            }
            ExpKind::Join { exp, sep } => {
                obj[&tag] = JsonValue::String("Join".into());
                obj["exp"] = exp.to_json();
                obj["sep"] = sep.to_json();
            }
            ExpKind::PositiveJoin { exp, sep } => {
                obj[&tag] = JsonValue::String("PositiveJoin".into());
                obj["exp"] = exp.to_json();
                obj["sep"] = sep.to_json();
            }
            ExpKind::Gather { exp, sep } => {
                obj[&tag] = JsonValue::String("Gather".into());
                obj["exp"] = exp.to_json();
                obj["sep"] = sep.to_json();
            }
            ExpKind::PositiveGather { exp, sep } => {
                obj[&tag] = JsonValue::String("PositiveGather".into());
                obj["exp"] = exp.to_json();
                obj["sep"] = sep.to_json();
            }
            ExpKind::RuleInclude { name, exp: _ } => {
                obj[&tag] = JsonValue::String("RuleInclude".into());
                obj["name"] = JsonValue::String(name.as_ref().to_string());
            }
            ExpKind::Eof => {
                obj[&tag] = JsonValue::String("EOF".into());
            }
            ExpKind::Eol => {
                obj[&tag] = JsonValue::String("EOL".into());
            }
            ExpKind::Cut => {
                obj[&tag] = JsonValue::String("Cut".into());
            }
        }

        obj
    }
}

#[cfg(test)]
mod tests {
    use crate::Grammar;

    #[test]
    fn test_grammar_to_json_value() {
        let json_str = include_str!("../../grammar/tatsu.json");
        let value = json::parse(json_str).expect("Failed to parse JSON");
        let grammar = Grammar::from_json_value(&value).expect("Failed to convert");
        let output = grammar.to_json();

        assert!(output.is_object());
        assert!(output["__class__"].is_string());
        assert!(output["name"].is_string());
        assert!(output["rules"].is_array());
    }
}
