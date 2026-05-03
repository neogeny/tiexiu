// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde_json::{Map, Value};
use tiexiu::cfg::types::FlagMap;
use tiexiu::trees::{KeyValue, Tree, TreeMap};
use std::rc::Rc;

#[derive(Debug, thiserror::Error)]
pub enum TreeJsonError {
    #[error("expected {0} to be a JSON object")]
    ExpectedObject(&'static str),

    #[error("expected {0} to be a JSON array")]
    ExpectedArray(&'static str),

    #[error("expected {0} to be a JSON string")]
    ExpectedString(&'static str),

    #[error("expected {0} to be a JSON boolean")]
    ExpectedBool(&'static str),

    #[error("missing JSON field '{0}'")]
    MissingField(&'static str),

    #[error("unknown tree JSON variant '{0}'")]
    UnknownVariant(String),
}

impl Tree {
    pub fn _to_model_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.to_model_json())
    }

    pub fn _from_model_json(json: &str) -> Result<Self, TreeJsonError> {
        let value: Value = serde_json::from_str(json)
            .map_err(|_| TreeJsonError::ExpectedObject("tree JSON document"))?;
        Self::from_serde_json_value(&value)
    }

    pub fn to_model_json(&self) -> Value {
        match self {
            Tree::Nil => named("Nil", []),
            Tree::Bottom => named("Bottom", []),
            Tree::Text(text) => named("Text", [("text", Value::String(text.to_string()))]),
            Tree::Seq(items) | Tree::List(items) => named(
                "List",
                [(
                    "items",
                    Value::Array(items.iter().map(|t| t.to_model_json()).collect()),
                )],
            ),
            Tree::Named(keyval) => named_value("Named", keyval),
            Tree::NamedAsList(keyval) => named_value("NamedAsList", keyval),
            Tree::Override(tree) => named("Override", [("tree", tree.as_ref().to_model_json())]),
            Tree::OverrideAsList(tree) => {
                named("OverrideAsList", [("tree", tree.as_ref().to_model_json())])
            }
            Tree::Map(m) => named("Map", [("entries", map_entries_value(m))]),
            Tree::Node { typename, tree } => named(
                "Node",
                [
                    ("typename", Value::String(typename.to_string())),
                    ("tree", tree.as_ref().to_model_json()),
                ],
            ),
        }
    }

    fn from_serde_json_value(value: &Value) -> Result<Self, TreeJsonError> {
        let object = expect_object(value, "tree")?;
        let kind = expect_string(field(object, "type")?, "type")?;

        match kind {
            "Nil" => Ok(Tree::Nil),
            "Bottom" => Ok(Tree::Bottom),
            "Void" => Ok(Tree::Nil),
            "Text" => Ok(Tree::Text(
                expect_string(field(object, "text")?, "text")?.into(),
            )),
            "List" => {
                let items: Vec<Rc<Tree>> = expect_array(field(object, "items")?, "items")?
                    .iter()
                    .map(|v| Tree::from_serde_json_value(v).map(|t| t.into()))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Tree::Seq(items.into()))
            }
            "Named" => Ok(Tree::Named(named_keyval(object)?)),
            "NamedAsList" => Ok(Tree::NamedAsList(named_keyval(object)?)),
            "Override" => Ok(Tree::Override(
                Tree::from_serde_json_value(field(object, "tree")?)?.into(),
            )),
            "OverrideAsList" => Ok(Tree::OverrideAsList(
                Tree::from_serde_json_value(field(object, "tree")?)?.into(),
            )),
            "Map" => Ok(Tree::Map(
                map_from_entries(field(object, "entries")?)?.into(),
            )),
            "Node" => Ok(Tree::Node {
                typename: expect_string(field(object, "typename")?, "typename")?.into(),
                tree: Tree::from_serde_json_value(field(object, "tree")?)?.into(),
            }),
            other => Err(TreeJsonError::UnknownVariant(other.to_string())),
        }
    }
}

fn named<const N: usize>(kind: &str, fields: [(&str, Value); N]) -> Value {
    let mut object = Map::new();
    object.insert("type".into(), Value::String(kind.into()));
    for (key, value) in fields {
        object.insert(key.into(), value);
    }
    Value::Object(object)
}

fn named_value(kind: &str, keyval: &KeyValue) -> Value {
    let KeyValue(name, tree) = keyval;
    named(
        kind,
        [
            ("name", Value::String(name.to_string())),
            ("tree", tree.to_model_json()),
        ],
    )
}

fn map_entries_value(m: &TreeMap) -> Value {
    Value::Array(
        m.iter()
            .map(|(key, value)| {
                named(
                    "Entry",
                    [
                        ("key", Value::String(key.to_string())),
                        ("value", value.to_model_json()),
                    ],
                )
            })
            .collect(),
    )
}

fn _flag_entries_value(flags: &FlagMap) -> Value {
    Value::Array(
        flags
            .iter()
            .map(|(key, value)| {
                named(
                    "Flag",
                    [
                        ("key", Value::String(key.to_string())),
                        ("value", Value::Bool(*value)),
                    ],
                )
            })
            .collect(),
    )
}

fn named_keyval(object: &Map<String, Value>) -> Result<KeyValue, TreeJsonError> {
    let name = expect_string(field(object, "name")?, "name")?;
    let tree = Tree::from_serde_json_value(field(object, "tree")?)?;
    Ok(KeyValue(name.into(), tree.into()))
}

fn map_from_entries(value: &Value) -> Result<TreeMap, TreeJsonError> {
    let mut m = TreeMap::new();
    for entry in expect_array(value, "entries")? {
        let object = expect_object(entry, "entry")?;
        let key = expect_string(field(object, "key")?, "key")?;
        let tree = Tree::from_serde_json_value(field(object, "value")?)?;
        m.insert(key, tree);
    }
    Ok(m)
}

fn field<'a>(
    object: &'a Map<String, Value>,
    key: &'static str,
) -> Result<&'a Value, TreeJsonError> {
    object.get(key).ok_or(TreeJsonError::MissingField(key))
}

fn expect_object<'a>(
    value: &'a Value,
    expected: &'static str,
) -> Result<&'a Map<String, Value>, TreeJsonError> {
    value
        .as_object()
        .ok_or(TreeJsonError::ExpectedObject(expected))
}

fn expect_array<'a>(
    value: &'a Value,
    expected: &'static str,
) -> Result<&'a [Value], TreeJsonError> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or(TreeJsonError::ExpectedArray(expected))
}

fn expect_string<'a>(value: &'a Value, expected: &'static str) -> Result<&'a str, TreeJsonError> {
    value
        .as_str()
        .ok_or(TreeJsonError::ExpectedString(expected))
}

fn _expect_bool(value: &Value, expected: &'static str) -> Result<bool, TreeJsonError> {
    value.as_bool().ok_or(TreeJsonError::ExpectedBool(expected))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_json_roundtrip() {
        let mut m = TreeMap::new();
        m.insert("name", Tree::Text("value".into()));
        m.insert_as_list("items", Tree::Text("a".into()));
        m.insert_as_list("items", Tree::Text("b".into()));

        let tree = Tree::Node {
            typename: "Rule".into(),
            tree: Tree::Map(m.into()).into(),
        };

        let value = tree.to_model_json();
        let round_tripped = Tree::from_serde_json_value(&value).unwrap();
        assert_eq!(round_tripped, tree);
    }
}
