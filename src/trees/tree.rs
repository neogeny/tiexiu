// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::map::TreeMap;
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq)]
pub struct KeyValue(pub Box<str>, pub Box<Tree>);

pub fn keyval(name: &str, tree: Tree) -> KeyValue {
    KeyValue(name.into(), tree.into())
}

pub type FlagMap = IndexMap<Box<str>, bool>;

#[derive(Debug, Clone, PartialEq)]
pub enum Tree {
    Text(Box<str>),      // Tokens or patterns
    List(Box<[Tree]>),   // Sequences of expressions
    Closed(Box<[Tree]>), // Sequences of expressions
    Map(Box<TreeMap>),   // A mapping of named elements

    Node {
        // The result of parsing a rule call
        typename: Box<str>,
        tree: Box<Tree>, // The result of parsing a rule
    },

    // INTERNAL
    // The folowing variants do not appear in final trees
    Nil,                   // Parsing that doesn't consume any input
    Named(KeyValue),       // Named elements
    NamedAsList(KeyValue), // Named elements forced into a list
    Override(Box<Tree>),   // Sets the value of the whole expression
    OverrideAsList(Box<Tree>),

    Bottom, // The marker for failure used in memoization
}

impl From<Vec<Tree>> for Tree {
    fn from(v: Vec<Tree>) -> Self {
        let clean: Vec<Tree> = v
            .into_iter()
            .filter(|item| !matches!(item, Tree::Nil))
            .collect();
        Tree::List(clean.into_boxed_slice())
    }
}

impl<const N: usize> From<[Tree; N]> for Tree {
    fn from(arr: [Tree; N]) -> Self {
        let clean: Vec<Tree> = arr
            .into_iter()
            .filter(|item| !matches!(item, Tree::Nil))
            .collect();
        Tree::List(clean.into_boxed_slice())
    }
}

impl Tree {
    pub fn value(&self) -> Box<str> {
        match self {
            Tree::Text(text) => text.clone(),
            _ => format!("{:#?}", self).into(),
        }
    }

    pub fn value_list(&self) -> Box<[Tree]> {
        match self {
            Tree::List(items) | Tree::Closed(items) => items.clone(),
            _ => [].into(),
        }
    }

    pub fn value_str_list(&self) -> Box<[Box<str>]> {
        self.value_list().iter().map(|t| t.value()).collect()
    }

    pub fn value_map(&self) -> Option<&TreeMap> {
        match self {
            Tree::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&Tree> {
        match self {
            Tree::Map(map) => map.get(key),
            _ => None,
        }
    }

    pub fn get_value(&self, key: &str) -> Box<str> {
        self.get(key)
            .map(|n| n.value())
            .unwrap_or_else(|| "".into())
    }

    pub fn get_list(&self, key: &str) -> Box<[Tree]> {
        self.get(key)
            .map(|n| n.value_list().clone())
            .unwrap_or_else(|| [].into())
    }

    pub fn get_str_list(&self, key: &str) -> Box<[Box<str>]> {
        self.get_list(key).iter().map(|t| t.value()).collect()
    }

    pub fn closed(self) -> Self {
        match self {
            Tree::List(items) => {
                Tree::Closed(items.into_iter().map(|item| item.normalized()).collect())
            }
            _ => self,
        }
    }

    pub fn aslist(self) -> Self {
        let norm = self.normalized();
        match &norm {
            Tree::List(_) | Tree::Closed(_) => norm,
            _ => Tree::List([norm].into()),
        }
    }

    pub fn append(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => n,
            (s, Self::Nil) => s,
            (Self::List(list), node) => {
                let mut v = list.into_vec();
                v.push(node);
                Self::List(v.into_boxed_slice())
            }
            (s, n) => Self::List(vec![s, n].into_boxed_slice()),
        }
    }

    pub fn append_as_list(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => Self::List(vec![n].into_boxed_slice()),
            (s, Self::Nil) => s,
            (Self::List(list), n) => {
                let mut v = list.into_vec();
                v.push(n);
                Self::List(v.into_boxed_slice())
            }
            (s, n) => Self::List(vec![s, n].into_boxed_slice()),
        }
    }

    pub fn merge(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => n,
            (s, Self::Nil) => s,
            (Self::List(l1), Self::List(l2)) => {
                let mut v = l1.into_vec();
                v.extend(l2.into_vec());
                Self::List(v.into_boxed_slice())
            }
            (Self::List(l1), n) => {
                let mut v = l1.into_vec();
                v.push(n);
                Self::List(v.into_boxed_slice())
            }
            (s, Self::List(l2)) => {
                let mut v = vec![s];
                v.extend(l2.into_vec());
                Self::List(v.into_boxed_slice())
            }
            (s, n) => s.append(n),
        }
    }

    pub fn normalized(self) -> Tree {
        let (map, root, tree) = self._normalize();

        if root != Tree::Nil {
            root
        } else if !map.is_empty() {
            Tree::Map(map.into())
        } else {
            tree
        }
    }

    fn _normalize(self) -> (TreeMap, Tree, Tree) {
        let mut map = TreeMap::new();
        let mut root = Tree::Nil;
        let mut tree = Tree::Nil;

        match self {
            Tree::List(elements) => {
                for node in elements {
                    let (child_map, child_root, child_cst) = node._normalize();

                    map.update(&child_map);
                    root = root.merge(child_root);
                    tree = tree.merge(child_cst);
                }
            }
            Tree::Named(keyval) => {
                let KeyValue(name, val) = keyval;
                map.insert(&name, *val);
            }
            Tree::NamedAsList(keyval) => {
                let KeyValue(name, val) = keyval;
                map.insert_as_list(&name, *val);
            }
            Tree::Override(val) => root = root.append(*val),
            Tree::OverrideAsList(val) => root = root.append_as_list(*val),
            Tree::Nil => {}
            other => tree = tree.merge(other),
        }

        (map, root, tree)
    }

    pub fn width(&self) -> usize {
        match self {
            Tree::Text(text) => text.len(),
            Tree::Override(inner) | Tree::OverrideAsList(inner) => inner.width(),
            Tree::Nil | Tree::Bottom => 0,
            Tree::List(items) | Tree::Closed(items) => items.iter().map(|item| item.width()).sum(),
            Tree::Map(map) => map.entries.values().map(|node| node.width()).sum(),
            Tree::Named(pair) | Tree::NamedAsList(pair) => {
                let KeyValue(_, val) = pair;
                val.width()
            }
            Tree::Node { typename: _, tree } => tree.width(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET: usize = 32;

    #[test]
    fn test_tree_size() {
        let size = size_of::<Tree>();
        // 24 bytes: Box (8) + Rc (8) + bool/padding (8)
        assert!(size <= TARGET, "Cst size is {} > {} bytes", size, TARGET);
    }
    #[test]
    fn test_keyval_size() {
        let size = size_of::<KeyValue>();
        assert!(size <= TARGET, "KeyVal size is {} > {} bytes", size, TARGET);
    }

    #[test]
    fn test_node_nil_removal() {
        let raw = Tree::List([Tree::Nil, Tree::Bottom, Tree::Nil].into());
        let result = raw.normalized();

        // result = tree_closed(Bottom)
        assert_eq!(result, Tree::Bottom.normalized());
    }

    #[test]
    fn test_node_nil_removal_to_bottom() {
        let raw = Tree::List([Tree::Nil, Tree::Bottom, Tree::Nil].into());
        let result = raw.normalized();

        // If tree_closed is identity for non-lists, this is just Bottom
        assert_eq!(result, Tree::Bottom);
    }

    #[test]
    fn test_node_nil_removal_to_list() {
        let raw = Tree::List([Tree::Bottom, Tree::Nil, Tree::Bottom].into());
        let result = raw.normalized(); // normalize doesn't close

        if let Tree::List(v) = result {
            assert_eq!(v.len(), 2); // Nil is gone, only the two Bottoms remain
            assert_eq!(v[0], Tree::Bottom);
            assert_eq!(v[1], Tree::Bottom);
        } else {
            panic!("Expected Closure, got {:?}", result);
        }
    }

    #[test]
    fn test_node_nil_purging_preserves_count() {
        // Input: List([Nil, Bottom, Nil])
        let raw = Tree::List([Tree::Nil, Tree::Bottom, Tree::Nil].into());
        let result = raw.normalized();

        // Since it's effectively Bottom, and Bottom isn't a list,
        // it doesn't become a Closure of len 1. It just stays Bottom.
        assert_eq!(result, Tree::Bottom);
    }
}
