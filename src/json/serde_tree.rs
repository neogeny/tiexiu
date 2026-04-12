// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::trees::{FlagMap, KeyValue, NodeMeta, Tree, TreeMap};
use serde_json::{Map, Value};

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
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.to_serde_json_value())
    }

    pub fn from_serde_json_str(json: &str) -> Result<Self, TreeJsonError> {
        let value: Value = serde_json::from_str(json)
            .map_err(|_| TreeJsonError::ExpectedObject("tree JSON document"))?;
        Self::from_serde_json_value(&value)
    }
    
    pub fn to_serde_json_value(&self) -> Value {
        match self {
            Tree::Nil => tagged("Nil", []),
            Tree::Bottom => tagged("Bottom", []),
            Tree::Text(text) => tagged("Text", [("text", Value::String(text.to_string()))]),
            Tree::List(items) => tagged(
                "List",
                [(
                    "items",
                    Value::Array(items.iter().map(Tree::to_serde_json_value).collect()),
                )],
            ),
            Tree::Named(keyval) => named_value("Named", keyval),
            Tree::NamedAsList(keyval) => named_value("NamedAsList", keyval),
            Tree::Override(tree) => tagged("Override", [("tree", tree.as_ref().to_serde_json_value())]),
            Tree::OverrideAsList(tree) => {
                tagged("OverrideAsList", [("tree", tree.as_ref().to_serde_json_value())])
            }
            Tree::Map(m) => tagged("Map", [("entries", map_entries_value(m))]),
            Tree::Node { meta, tree } => tagged(
                "Node",
                [
                    ("meta", node_meta_value(meta)),
                    ("tree", tree.as_ref().to_serde_json_value()),
                ],
            ),
        }
    }
    
    pub fn from_serde_json_value(value: &Value) -> Result<Self, TreeJsonError> {
        let object = expect_object(value, "tree")?;
        let kind = expect_string(field(object, "type")?, "type")?;

        match kind {
            "Nil" => Ok(Tree::Nil),
            "Bottom" => Ok(Tree::Bottom),
            "Void" => Ok(Tree::Nil),
            "Text" => Ok(Tree::Text(
                expect_string(field(object, "text")?, "text")?.into(),
            )),
            "List" => Ok(Tree::List(
                expect_array(field(object, "items")?, "items")?
                    .iter()
                    .map(Tree::from_serde_json_value)
                    .collect::<Result<Vec<_>, _>>()?
                    .into(),
            )),
            "Named" => Ok(Tree::Named(named_keyval(object)?.into())),
            "NamedAsList" => Ok(Tree::NamedAsList(named_keyval(object)?.into())),
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
                meta: node_meta_from_value(field(object, "meta")?)?.into(),
                tree: Tree::from_serde_json_value(field(object, "tree")?)?.into(),
            }),
            other => Err(TreeJsonError::UnknownVariant(other.to_string())),
        }
    }
}

fn tagged<const N: usize>(kind: &str, fields: [(&str, Value); N]) -> Value {
    let mut object = Map::new();
    object.insert("type".into(), Value::String(kind.into()));
    for (key, value) in fields {
        object.insert(key.into(), value);
    }
    Value::Object(object)
}

fn named_value(kind: &str, keyval: &KeyValue) -> Value {
    let KeyValue(name, tree) = keyval;
    tagged(
        kind,
        [
            ("name", Value::String(name.to_string())),
            ("tree", tree.to_serde_json_value()),
        ],
    )
}

fn map_entries_value(m: &TreeMap) -> Value {
    Value::Array(
        m.entries
            .iter()
            .map(|(key, value)| {
                tagged(
                    "Entry",
                    [
                        ("key", Value::String(key.to_string())),
                        ("value", value.to_serde_json_value()),
                    ],
                )
            })
            .collect(),
    )
}

fn node_meta_value(meta: &NodeMeta) -> Value {
    tagged(
        "NodeMeta",
        [
            ("name", Value::String(meta.name.to_string())),
            (
                "params",
                Value::Array(
                    meta.params
                        .iter()
                        .map(|param| Value::String(param.to_string()))
                        .collect(),
                ),
            ),
            ("flags", flag_entries_value(&meta.flags)),
        ],
    )
}

fn flag_entries_value(flags: &FlagMap) -> Value {
    Value::Array(
        flags
            .iter()
            .map(|(key, value)| {
                tagged(
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
    Ok(KeyValue(name.into(), tree))
}

fn map_from_entries(value: &Value) -> Result<TreeMap, TreeJsonError> {
    let mut m = TreeMap::new();
    for entry in expect_array(value, "entries")? {
        let object = expect_object(entry, "entry")?;
        let key = expect_string(field(object, "key")?, "key")?;
        let tree = Tree::from_serde_json_value(field(object, "value")?)?;
        m.entries.insert(key.into(), tree);
    }
    Ok(m)
}

fn node_meta_from_value(value: &Value) -> Result<NodeMeta, TreeJsonError> {
    let object = expect_object(value, "meta")?;
    let name = expect_string(field(object, "name")?, "name")?;
    let params = expect_array(field(object, "params")?, "params")?
        .iter()
        .map(|param| expect_string(param, "param").map(Into::into))
        .collect::<Result<Vec<Box<str>>, _>>()?;
    let flags = flags_from_value(field(object, "flags")?)?;

    Ok(NodeMeta {
        name: name.into(),
        params: params.into(),
        flags,
    })
}

fn flags_from_value(value: &Value) -> Result<FlagMap, TreeJsonError> {
    let mut flags = FlagMap::new();
    for entry in expect_array(value, "flags")? {
        let object = expect_object(entry, "flag")?;
        let key = expect_string(field(object, "key")?, "key")?;
        let value = expect_bool(field(object, "value")?, "value")?;
        flags.insert(key.into(), value);
    }
    Ok(flags)
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

fn expect_bool(value: &Value, expected: &'static str) -> Result<bool, TreeJsonError> {
    value.as_bool().ok_or(TreeJsonError::ExpectedBool(expected))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_json_round_trip() {
        let mut m = TreeMap::new();
        m.insert("name", Tree::Text("value".into()));
        m.insert_as_list("items", Tree::Text("a".into()));
        m.insert_as_list("items", Tree::Text("b".into()));

        let tree = Tree::Node {
            meta: NodeMeta {
                name: "Rule".into(),
                params: ["p"].map(Into::into).into(),
                flags: [("is_name".into(), true), ("no_memo".into(), false)]
                    .into_iter()
                    .collect(),
            }
            .into(),
            tree: Tree::Map(m.into()).into(),
        };

        let value = tree.to_serde_json_value();
        let round_tripped = Tree::from_serde_json_value(&value).unwrap();
        assert_eq!(round_tripped, tree);
    }
}
