// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Imp - Direct Value to Grammar translator
//!
//! This module translates serde_json::Value directly to Grammar,
//! bypassing the TatSuModel deserializer which fails on modified JSON.

use crate::json::error::ImportError;
use crate::peg::exp::Exp;
use crate::peg::grammar::Grammar;
use crate::peg::rule::Rule;
use serde_json::Value;

#[derive(Clone)]
pub struct JsonSerializationHelper {
    value: Value,
    path: Vec<String>,
}

impl JsonSerializationHelper {
    fn new(value: Value) -> Self {
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

    fn get_obj(&self) -> Result<&serde_json::Map<String, Value>, ImportError> {
        self.value
            .as_object()
            .ok_or_else(|| self.error("Expected object"))
    }

    fn error(&self, msg: &str) -> ImportError {
        let path_str = self.path.join(" -> ");
        if path_str.is_empty() {
            ImportError::Other(msg.into())
        } else {
            ImportError::Other(format!("{} at {}", msg, path_str))
        }
    }

    fn get_class(&self) -> Result<String, ImportError> {
        if let Ok(obj) = self.get_obj() {
            obj.get("__class__")
                .and_then(|v| v.as_str())
                .map(String::from)
                .ok_or_else(|| self.error("Missing __class__"))
        } else {
            Err(self.error("Missing __class__"))
        }
    }

    fn get_string(&self, field: &str) -> Result<String, ImportError> {
        if let Ok(obj) = self.get_obj() {
            obj.get(field)
                .and_then(|v| v.as_str())
                .map(String::from)
                .ok_or_else(|| self.error(&format!("Missing field: {}", field)))
        } else {
            Err(self.error(&format!("Missing field: {}", field)))
        }
    }

    fn get_nested(&self, field: &str) -> Result<JsonSerializationHelper, ImportError> {
        let obj = self.get_obj()?;
        let value = obj
            .get(field)
            .ok_or_else(|| self.error(&format!("Missing field: {}", field)))?;

        // Push field name and __class__ for better error reporting
        let nested_path = if let Some(child_obj) = value.as_object() {
            if let Some(class) = child_obj.get("__class__").and_then(|v: &Value| v.as_str()) {
                format!("{}:{}", field, class)
            } else {
                field.to_string()
            }
        } else {
            field.to_string()
        };

        Ok(self.push(&nested_path).with_value(value.clone()))
    }

    fn with_value(&self, value: Value) -> Self {
        Self {
            value,
            path: self.path.clone(),
        }
    }

    fn get_array(&self, field: &str) -> Result<Vec<JsonSerializationHelper>, ImportError> {
        if let Ok(obj) = self.get_obj()
            && let Some(arr) = obj.get(field).and_then(|v: &Value| v.as_array()) 
        {
            return Ok(arr
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let label = if let Some(child_obj) = v.as_object() {
                        if let Some(class) = child_obj.get("__class__").and_then(|v: &Value| v.as_str()) {
                            format!("{}[{}]:{}", field, i, class)
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
        Err(self.error(&format!("Missing or not array: {}", field)))
    }

    fn opt_str(&self, field: &str) -> Option<&str> {
        if let Ok(obj) = self.get_obj() {
            obj.get(field).and_then(|v| v.as_str())
        } else {
            None
        }
    }

    fn opt_bool(&self, field: &str, default: bool) -> bool {
        if let Ok(obj) = self.get_obj() {
            obj.get(field).and_then(|v| v.as_bool()).unwrap_or(default)
        } else {
            default
        }
    }

    fn opt_u64(&self, field: &str) -> Option<u64> {
        if let Ok(obj) = self.get_obj() {
            obj.get(field).and_then(|v| v.as_u64())
        } else {
            None
        }
    }
}

impl Grammar {
    pub fn from_serde_value(value: &Value) -> Result<Self, ImportError> {
        let path = JsonSerializationHelper::new(value.clone());
        let class = path.get_class()?;

        if class != "Grammar" {
            return Err(path.error("Expected Grammar root"));
        }

        let name = path.get_string("name")?;
        let analyzed = path.opt_bool("analyzed", false);

        let rules = path.get_array("rules")?;
        let rule_vec: Result<Vec<_>, _> = rules
            .iter()
            .enumerate()
            .map(|(i, f)| {
                Rule::from_serde_value_with_path(f.clone())
                    .map_err(|e| ImportError::InvalidField(format!("rules[{}]: {}", i, e)))
            })
            .collect();

        let directives =
            Self::parse_directives(path.get_obj().ok().and_then(|o| o.get("directives")))?;
        let keywords: std::collections::HashSet<String> = path
            .get_obj()
            .ok()
            .and_then(|o| o.get("keywords"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();

        let mut grammar = Grammar::new(&name, &rule_vec?);
        grammar.analyzed = analyzed;
        grammar.directives = directives;
        grammar.keywords = keywords;
        grammar.initialize();
        Ok(grammar)
    }

    fn parse_directives(
        directives: Option<&Value>,
    ) -> Result<std::collections::HashMap<String, String>, ImportError> {
        let mut result = std::collections::HashMap::new();
        if let Some(Value::Object(obj)) = directives {
            for (k, v) in obj {
                let val_str = match v {
                    Value::String(s) => s.clone(),
                    Value::Bool(b) => b.to_string(),
                    Value::Number(n) => n.to_string(),
                    _ => v.to_string(),
                };
                result.insert(k.clone(), val_str);
            }
        }
        Ok(result)
    }
}

impl Rule {
    pub fn from_serde_value(value: &Value) -> Result<Self, ImportError> {
        let path = JsonSerializationHelper::new(value.clone());
        Self::from_serde_value_with_path(path)
    }

    pub fn from_serde_value_with_path(path: JsonSerializationHelper) -> Result<Self, ImportError> {
        let class = path.get_class()?;

        if class != "Rule" {
            return Err(path.error("Expected Rule"));
        }

        let name = path.get_string("name")?;
        let rhs = Exp::from_serde_value_with_path(path.get_nested("exp")?)?;

        let params = path
            .get_obj()
            .ok()
            .and_then(|o| o.get("params"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();

        let is_name = path.opt_bool("is_name", false);
        let is_tokn = path.opt_bool("is_tokn", false);
        let no_memo = path.opt_bool("no_memo", false);
        let is_memo = path.opt_bool("is_memo", true);
        let is_lrec = path.opt_bool("is_lrec", false);

        Ok(Rule::from_parts(
            name, params, rhs, is_name, is_tokn, no_memo, is_memo, is_lrec,
        ))
    }
}

impl Exp {
    pub fn from_serde_value(value: &Value) -> Result<Self, ImportError> {
        let path = JsonSerializationHelper::new(value.clone());
        Self::from_serde_value_with_path(path)
    }

    pub fn from_serde_value_with_path(path: JsonSerializationHelper) -> Result<Self, ImportError> {
        let class = path.get_class()?;

        match class.as_str() {
            "Sequence" => {
                let items = path.get_array("sequence")?;
                let exprs: Result<Vec<_>, _> = items
                    .iter()
                    .map(|f| Exp::from_serde_value_with_path(f.clone()))
                    .collect();
                Ok(Exp::sequence(exprs?.as_slice().into()))
            }
            "Choice" => {
                let items = path.get_array("options")?;
                let exprs: Result<Vec<_>, _> = items
                    .iter()
                    .map(|f| Exp::from_serde_value_with_path(f.clone()))
                    .collect();
                Ok(Exp::choice(exprs?.as_slice().into()))
            }
            "Option" => Ok(Exp::alt(Exp::from_serde_value_with_path(path.get_nested("exp")?)?)),
            "Named" => Ok(Exp::named(
                &path.get_string("name")?,
                Exp::from_serde_value_with_path(path.get_nested("exp")?)?,
            )),
            "NamedList" => Ok(Exp::named_list(
                &path.get_string("name")?,
                Exp::from_serde_value_with_path(path.get_nested("exp")?)?,
            )),
            "Call" => Ok(Exp::call(&path.get_string("name")?)),
            "Token" => Ok(Exp::token(&path.get_string("token")?)),
            "Pattern" => Ok(Exp::pattern(&path.get_string("pattern")?)),
            "Constant" => Ok(Exp::constant(path.opt_str("literal").unwrap_or(""))),
            "Alert" => Ok(Exp::alert(
                path.opt_str("literal").unwrap_or(""),
                path.opt_u64("level").unwrap_or(0) as u8,
            )),
            "Group" => Ok(Exp::group(Exp::from_serde_value_with_path(path.get_nested("exp")?)?)),
            "Optional" => Ok(Exp::optional(Exp::from_serde_value_with_path(path.get_nested("exp")?)?)),
            "Closure" => Ok(Exp::closure(Exp::from_serde_value_with_path(path.get_nested("exp")?)?)),
            "PositiveClosure" => Ok(Exp::positive_closure(Exp::from_serde_value_with_path(
                path.get_nested("exp")?,
            )?)),
            "Lookahead" => Ok(Exp::lookahead(Exp::from_serde_value_with_path(path.get_nested("exp")?)?)),
            "NegativeLookahead" => Ok(Exp::negative_lookahead(Exp::from_serde_value_with_path(
                path.get_nested("exp")?,
            )?)),
            "SkipGroup" => Ok(Exp::skip_group(Exp::from_serde_value_with_path(path.get_nested("exp")?)?)),
            "SkipTo" => Ok(Exp::skip_to(Exp::from_serde_value_with_path(path.get_nested("exp")?)?)),
            "Override" => Ok(Exp::override_node(Exp::from_serde_value(
                &path.get_nested("exp")?.value,
            )?)),
            "OverrideList" => Ok(Exp::override_list(Exp::from_serde_value(
                &path.get_nested("exp")?.value,
            )?)),
            "Join" => Ok(Exp::join(
                Exp::from_serde_value_with_path(path.get_nested("exp")?)?,
                Exp::from_serde_value_with_path(path.get_nested("sep")?)?,
            )),
            "PositiveJoin" => Ok(Exp::positive_join(
                Exp::from_serde_value_with_path(path.get_nested("exp")?)?,
                Exp::from_serde_value_with_path(path.get_nested("sep")?)?,
            )),
            "Gather" => Ok(Exp::gather(
                Exp::from_serde_value_with_path(path.get_nested("exp")?)?,
                Exp::from_serde_value_with_path(path.get_nested("sep")?)?,
            )),
            "PositiveGather" => Ok(Exp::positive_gather(
                Exp::from_serde_value_with_path(path.get_nested("exp")?)?,
                Exp::from_serde_value_with_path(path.get_nested("sep")?)?,
            )),
            "RuleInclude" => {
                let name = path.get_string("name")?;
                let exp = path
                    .get_obj()
                    .ok()
                    .and_then(|o| o.get("exp"))
                    .and_then(|v| {
                        if v.is_null() {
                            None
                        } else {
                            Some(Exp::from_serde_value(v))
                        }
                    });
                match exp {
                    Some(Ok(inner)) => Ok(Exp::rule_include_with(&name, inner)),
                    Some(Err(e)) => Err(e),
                    None => Ok(Exp::rule_include(&name)),
                }
            }
            "Void" => Ok(Exp::void()),
            "Cut" => Ok(Exp::cut()),
            "EOF" => Ok(Exp::eof()),
            _ => Err(path.error(&format!("Unsupported: {}", class))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_from_serde_value_tatsu() {
        let json_str = std::fs::read_to_string("grammar/tatsu.json").expect("tatsu.json missing");
        let value: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");
        let grammar = Grammar::from_serde_value(&value).expect("Failed to convert");
        assert_eq!(grammar.name, "TatSu");
        let rule_count = grammar.rules().count();
        assert!(rule_count > 0, "Expected rules, got {}", rule_count);
    }

    #[test]
    fn test_grammar_from_serde_value_calc() {
        let json_str = std::fs::read_to_string("grammar/calc.json").expect("calc.json missing");
        let value: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");
        let grammar = Grammar::from_serde_value(&value).expect("Failed to convert");
        assert_eq!(grammar.name, "CALC");
    }
}
