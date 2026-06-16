use super::{Tree, TreeBuild, TreeMap};
use crate::util::strtools::is_valid_identifier;
use serde_json::{json, Value};

pub const KEY_SEQ: &str = "*";
pub const KEY_AT: &str = "=";
pub const KEY_AT_LIST: &str = "=+";
pub const KEY_NAMED: &str = ":";
pub const KEY_NAMED_LIST: &str = ":+";
pub const KEY_NODE: &str = "{}";

impl TreeBuild {
    /// Creates a Seq tree node.
    pub fn seq(items: &[Tree]) -> Tree {
        json!({KEY_SEQ: items})
    }

    /// Creates a Seq tree node.
    pub fn named(name: &str, items: &[Tree]) -> Tree {
        if !is_valid_identifier(name) {
            return Self::nil()
        }
        let keyname = format!("{}{}", KEY_NAMED, name);
        json!({keyname: items})
    }

    /// Creates a Seq tree node.
    pub fn named_as_list(name: &str, items: &[Tree]) -> Tree {
        if !is_valid_identifier(name) {
            return Self::nil()
        }
        let keyname = format!("{}{}", KEY_NAMED_LIST, name);
        json!({keyname: items})
    }

    /// Creates an Override tree node for root merging.
    pub fn override_with(tree: Tree) -> Tree {
        json!({KEY_AT: tree})
    }

    /// Creates an OverrideAsList tree node for list root merging.
    pub fn override_as_list(tree: Tree) -> Tree {
        json!({KEY_AT_LIST: tree})
    }

    /// Creates a Node tree node from a rule call result.
    pub fn node(typename: &str, tree: Tree) -> Tree {
        if !is_valid_identifier(typename) {
            return Self::nil()
        }
        json!(
            {
                KEY_NODE: typename,
                "tree": tree
            }
        )
    }

    pub fn is_seq(tree: &Tree) -> bool {
        if let Value::Object(map) = tree {
            map.contains_key(KEY_SEQ)
        } else {
            false
        }

    }

    pub fn seq_items(tree: &Tree) -> Option<&Vec<Tree>> {
        if let Value::Object(map) = tree
            && let Some(value) = map.get(KEY_SEQ)
            && let Value::Array(items) = value {
            return Some(items)
        }
        None
    }

    pub fn tree_map(tree: &Tree) -> Option<&TreeMap> {
        if let Value::Object(map) = tree {
            return Some(map)
        }
        None
    }

}
