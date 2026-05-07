// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Imp - Direct Value to Grammar translator
//!
//! This module translates json::JsonValue directly to Grammar,
//! bypassing the TatSuModel deserializer which fails on modified JSON.

use crate::cfg::*;
use crate::json::error::JsonError;
use crate::peg::exp::Exp;
use crate::peg::grammar::{Grammar, GrammarDirectives};
use crate::peg::rule::Rule;
use crate::types::Str;
use json::JsonValue;

#[derive(Clone)]
pub struct JsonSerializationHelper {
    value: JsonValue,
    path: Vec<String>,
}

impl Grammar {
    pub fn from_json(json: &str) -> Result<Self, JsonError> {
        let value = json::parse(json)?;
        Self::from_json_value(&value)
    }

    pub fn from_json_value(value: &JsonValue) -> Result<Self, JsonError> {
        let path = JsonSerializationHelper::new(value.clone());
        let class = path.get_class()?;

        if class != "Grammar" {
            return Err(path.error("Expected Grammar root"));
        }

        let name = path.get_string("name")?;
        let _analyzed = path.opt_bool("analyzed", false);

        let rules = path.get_array("rules")?;
        let rule_vec: Result<Vec<_>, _> = rules
            .iter()
            .enumerate()
            .map(|(i, f)| {
                Rule::from_json_with_path(f.clone())
                    .map_err(|e| JsonError::InvalidField(format!("rules[{}]: {}", i, e)))
            })
            .collect();

        let directives =
            Self::parse_directives(path.get_obj().ok().and_then(|o| o.get("directives")))?;
        let keywords: Vec<Str> = if let Ok(obj) = path.get_obj() {
            if let Some(keywords_val) = obj.get("keywords") {
                if let JsonValue::Array(arr) = keywords_val {
                    arr.iter().map(|v| v.to_string().into()).collect()
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        let mut grammar = Grammar::new(
            &name,
            rule_vec?
                .into_iter()
                .map(|r| r.into())
                .collect::<Vec<_>>()
                .as_slice(),
        );
        grammar.set_directives(directives);
        grammar.set_keywords(keywords.as_slice());
        grammar.initialize()?;
        Ok(grammar)
    }

    fn parse_directives(directives: Option<&JsonValue>) -> Result<GrammarDirectives, JsonError> {
        if let Some(JsonValue::Object(obj)) = directives {
            let res: GrammarDirectives = obj
                .iter()
                .filter_map(|(k, v)| {
                    let val_str = match v {
                        JsonValue::String(s) => s.to_string(),
                        JsonValue::Boolean(b) => b.to_string(),
                        JsonValue::Number(n) => n.to_string(),
                        _ => v.to_string(),
                    };
                    Cfg::map(k, val_str.as_str())
                })
                .collect();
            return Ok(res);
        }
        Ok(GrammarDirectives::default())
    }
}

impl Rule {
    pub fn from_json_value(value: &JsonValue) -> Result<Self, JsonError> {
        let path = JsonSerializationHelper::new(value.clone());
        Self::from_json_with_path(path)
    }

    pub fn from_json_with_path(path: JsonSerializationHelper) -> Result<Self, JsonError> {
        let class = path.get_class()?;

        if class != "Rule" {
            return Err(path.error("Expected Rule"));
        }

        let name = path.get_string("name")?;
        let rhs = Exp::from_json_with_path(path.get_nested("exp")?)?;

        let params: Vec<String> = if let Ok(obj) = path.get_obj() {
            if let Some(params_val) = obj.get("params") {
                if let JsonValue::Array(arr) = params_val {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(String::from)
                        .collect()
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        let no_memo = path.opt_bool("no_memo", false);
        let no_stak = path.opt_bool("no_stak", false);
        let is_name = path.opt_bool("is_name", false);
        let is_tokn = path.opt_bool("is_tokn", false);
        let is_memo = path.opt_bool("is_memo", true);
        let is_lrec = path.opt_bool("is_lrec", false);

        Ok(Rule::from_parts(
            name, params, rhs, is_name, is_tokn, is_memo, is_lrec, no_memo, no_stak,
        ))
    }
}

impl Exp {
    pub fn from_json_value(value: &JsonValue) -> Result<Self, JsonError> {
        let path = JsonSerializationHelper::new(value.clone());
        Self::from_json_with_path(path)
    }

    pub fn from_json_with_path(path: JsonSerializationHelper) -> Result<Self, JsonError> {
        let class = path.get_class()?;

        match class.as_str() {
            "Sequence" => {
                let items = path.get_array("sequence")?;
                let exprs: Result<Vec<_>, _> = items
                    .iter()
                    .map(|f| Exp::from_json_with_path(f.clone()))
                    .collect();
                Ok(Exp::sequence(exprs?.as_slice().into()))
            }
            "Choice" => {
                let items = path.get_array("options")?;
                let exprs: Result<Vec<_>, _> = items
                    .iter()
                    .map(|f| Exp::from_json_with_path(f.clone()))
                    .collect();
                Ok(Exp::choice(exprs?.as_slice().into()))
            }
            "Option" => Ok(Exp::alt(Exp::from_json_with_path(path.get_nested("exp")?)?)),
            "Named" => Ok(Exp::named(
                &path.get_string("name")?,
                Exp::from_json_with_path(path.get_nested("exp")?)?,
            )),
            "NamedList" => Ok(Exp::named_list(
                &path.get_string("name")?,
                Exp::from_json_with_path(path.get_nested("exp")?)?,
            )),
            "Call" => Ok(Exp::call(&path.get_string("name")?)),
            "Token" => Ok(Exp::token(&path.get_string("token")?)),
            "Pattern" => Ok(Exp::pattern(&path.get_string("pattern")?)),
            "Constant" => Ok(Exp::constant(path.opt_str("literal").unwrap_or(""))),
            "Alert" => Ok(Exp::alert(
                path.opt_str("literal").unwrap_or(""),
                path.opt_u64("level").unwrap_or(0) as u8,
            )),
            "Group" => Ok(Exp::group(Exp::from_json_with_path(
                path.get_nested("exp")?,
            )?)),
            "Optional" => Ok(Exp::optional(Exp::from_json_with_path(
                path.get_nested("exp")?,
            )?)),
            "Closure" => Ok(Exp::closure(Exp::from_json_with_path(
                path.get_nested("exp")?,
            )?)),
            "PositiveClosure" => Ok(Exp::positive_closure(Exp::from_json_with_path(
                path.get_nested("exp")?,
            )?)),
            "Lookahead" => Ok(Exp::lookahead(Exp::from_json_with_path(
                path.get_nested("exp")?,
            )?)),
            "NegativeLookahead" => Ok(Exp::negative_lookahead(Exp::from_json_with_path(
                path.get_nested("exp")?,
            )?)),
            "SkipGroup" => Ok(Exp::skip_group(Exp::from_json_with_path(
                path.get_nested("exp")?,
            )?)),
            "SkipTo" => Ok(Exp::skip_to(Exp::from_json_with_path(
                path.get_nested("exp")?,
            )?)),
            "Override" => Ok(Exp::override_node(Exp::from_json_value(
                &path.get_nested("exp")?.value,
            )?)),
            "OverrideList" => Ok(Exp::override_list(Exp::from_json_value(
                &path.get_nested("exp")?.value,
            )?)),
            "Join" => Ok(Exp::join(
                Exp::from_json_with_path(path.get_nested("exp")?)?,
                Exp::from_json_with_path(path.get_nested("sep")?)?,
            )),
            "PositiveJoin" => Ok(Exp::positive_join(
                Exp::from_json_with_path(path.get_nested("exp")?)?,
                Exp::from_json_with_path(path.get_nested("sep")?)?,
            )),
            "Gather" => Ok(Exp::gather(
                Exp::from_json_with_path(path.get_nested("exp")?)?,
                Exp::from_json_with_path(path.get_nested("sep")?)?,
            )),
            "PositiveGather" => Ok(Exp::positive_gather(
                Exp::from_json_with_path(path.get_nested("exp")?)?,
                Exp::from_json_with_path(path.get_nested("sep")?)?,
            )),
            "RuleInclude" => {
                let name = path.get_string("name")?;
                Ok(Exp::rule_include(&name))
            }
            "Void" => Ok(Exp::void()),
            "Cut" => Ok(Exp::cut()),
            "EOF" => Ok(Exp::eof()),
            "EOL" => Ok(Exp::eol()),
            "EmptyClosure" => Ok(Exp::empty_closure()),
            _ => Err(path.error(&format!("Unsupported: {}", class))),
        }
    }
}

impl JsonSerializationHelper {
    fn new(value: JsonValue) -> Self {
        Self {
            value,
            path: Vec::new(),
        }
    }

    fn push(&self, class: &str) -> Self {
        let mut path = self.path.clone();
        path.push(class.to_string());
        Self {
            value: self.value.clone(),
            path,
        }
    }

    fn get_obj(&self) -> Result<&json::object::Object, JsonError> {
        match &self.value {
            JsonValue::Object(obj) => Ok(obj),
            _ => Err(self.error("Expected object")),
        }
    }

    fn error(&self, msg: &str) -> JsonError {
        let path_str = self.path.join(" -> ");
        if path_str.is_empty() {
            JsonError::Other(msg.into())
        } else {
            JsonError::Other(format!("{} at {}", msg, path_str))
        }
    }

    fn get_class(&self) -> Result<String, JsonError> {
        if let Ok(obj) = self.get_obj() {
            if let Some(class_val) = obj.get("__class__") {
                match class_val {
                    JsonValue::String(s) => return Ok(s.clone()),
                    JsonValue::Short(s) => return Ok(s.to_string()),
                    _ => {}
                }
            }
            Err(self.error("Missing __class__"))
        } else {
            Err(self.error("Missing __class__"))
        }
    }

    fn get_string(&self, field: &str) -> Result<String, JsonError> {
        if let Ok(obj) = self.get_obj() {
            if let Some(val) = obj.get(field) {
                return match val {
                    JsonValue::String(s) => Ok(s.clone()),
                    JsonValue::Short(s) => Ok(s.to_string()),
                    _ => Err(self.error(&format!("Missing field: {}", field))),
                };
            }
            Err(self.error(&format!("Missing field: {}", field)))
        } else {
            Err(self.error(&format!("Missing field: {}", field)))
        }
    }

    fn get_nested(&self, field: &str) -> Result<JsonSerializationHelper, JsonError> {
        let obj = self.get_obj()?;
        let value = obj
            .get(field)
            .ok_or_else(|| self.error(&format!("Missing field: {}", field)))?
            .clone();

        let nested_path = if let JsonValue::Object(child_obj) = &value {
            if let Some(class_val) = child_obj.get("__class__") {
                match class_val {
                    JsonValue::String(s) => format!("{}:{}", field, s),
                    JsonValue::Short(s) => format!("{}:{}", field, s),
                    _ => field.to_string(),
                }
            } else {
                field.to_string()
            }
        } else {
            field.to_string()
        };

        Ok(self.push(&nested_path).with_value(value))
    }

    fn with_value(&self, value: JsonValue) -> Self {
        Self {
            value,
            path: self.path.clone(),
        }
    }

    fn get_array(&self, field: &str) -> Result<Vec<JsonSerializationHelper>, JsonError> {
        if let Ok(obj) = self.get_obj() {
            if let Some(arr_val) = obj.get(field) {
                if let JsonValue::Array(arr) = arr_val {
                    return Ok(arr
                        .iter()
                        .enumerate()
                        .map(|(i, v)| {
                            let label = if let JsonValue::Object(child_obj) = v {
                                if let Some(class_val) = child_obj.get("__class__") {
                                    if let JsonValue::String(class) = class_val {
                                        format!("{}[{}]:{}", field, i, class)
                                    } else {
                                        format!("{}[{}]", field, i)
                                    }
                                } else {
                                    format!("{}[{}]", field, i)
                                }
                            } else {
                                format!("{}[{}]", field, i)
                            };
                            self.push(&label).with_value(v.clone())
                        })
                        .collect());
                }
            }
        }
        Err(self.error(&format!("Missing or not array: {}", field)))
    }

    fn opt_str(&self, field: &str) -> Option<&str> {
        if let Ok(obj) = self.get_obj() {
            if let Some(val) = obj.get(field) {
                if let JsonValue::String(s) = val {
                    return Some(s);
                }
            }
            None
        } else {
            None
        }
    }

    fn opt_bool(&self, field: &str, default: bool) -> bool {
        if let Ok(obj) = self.get_obj() {
            if let Some(val) = obj.get(field) {
                if let JsonValue::Boolean(b) = val {
                    return *b;
                }
            }
            default
        } else {
            default
        }
    }

    fn opt_u64(&self, field: &str) -> Option<u64> {
        if let Ok(obj) = self.get_obj() {
            if let Some(val) = obj.get(field) {
                if let JsonValue::Number(n) = val {
                    return u64::try_from(*n).ok();
                }
            }
            None
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_from_json_value_tatsu() {
        let json_str = std::fs::read_to_string("grammar/tatsu.json").expect("tatsu.json missing");
        let value = json::parse(&json_str).expect("Failed to parse JSON");
        let grammar = Grammar::from_json_value(&value).expect("Failed to convert");
        assert_eq!(grammar.name, "TatSu".into());
        let rule_count = grammar.rules().count();
        assert!(rule_count > 0, "Expected rules, got {}", rule_count);
    }

    #[test]
    fn test_grammar_from_json_value_calc_imported() {
        let json_str = std::fs::read_to_string("grammar/calc.json").expect("calc.json missing");
        let value = json::parse(&json_str).expect("Failed to parse JSON");
        let grammar = Grammar::from_json_value(&value).expect("Failed to convert");
        assert_eq!(grammar.name, "CALC".into());
        assert_eq!(grammar.rules().count(), 9);
        assert!(grammar.analyzed);
    }

    #[test]
    #[cfg_attr(not(feature = "grammars"), ignore)]
    fn test_grammar_from_json_value_java() {
        let path = std::path::Path::new("grammar/java.json");
        if !path.exists() {
            eprintln!("SKIP: grammar/java.json not found");
            return;
        }
        let json_str = std::fs::read_to_string("grammar/java.json").expect("java.json missing");
        let value = json::parse(&json_str).expect("Failed to parse JSON");
        let grammar = Grammar::from_json_value(&value).expect("Failed to convert");
        assert_eq!(grammar.name, "Java".into());

        let rule_count = grammar.rules().count();
        assert!(
            rule_count > 50,
            "Java grammar should have many rules, got {}",
            rule_count
        );

        let start_rule = grammar
            .start_rule()
            .expect("Java grammar should have a start rule");
        assert!(!start_rule.is_empty());
    }
}
