// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::tree::Tree;
use crate::cfg::types::Define;
use crate::trees::TreeRef;
use crate::types::{Ref, Str};

/// A reference-counted slice of key-tree entries for TreeMap.
pub type TreeEntrySlice = Ref<[(Str, TreeRef)]>;

/// An ordered map of named tree elements.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeMap(pub TreeEntrySlice);

impl From<Vec<(Str, TreeRef)>> for TreeMap {
    fn from(vec: Vec<(Str, TreeRef)>) -> Self {
        TreeMap(vec.into())
    }
}

impl From<Vec<(Str, Tree)>> for TreeMap {
    fn from(vec: Vec<(Str, Tree)>) -> Self {
        let converted: Vec<(Str, TreeRef)> = vec.into_iter().map(|(k, v)| (k, v.into())).collect();
        TreeMap(converted.into())
    }
}

impl From<TreeMap> for Vec<(Str, TreeRef)> {
    fn from(tree_map: TreeMap) -> Self {
        tree_map.0.as_ref().to_vec()
    }
}

impl TreeMap {
    /// Creates an empty TreeMap.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an iterator over (key, tree) entries.
    pub fn iter(&self) -> std::slice::Iter<'_, (Str, TreeRef)> {
        self.0.iter()
    }

    /// Returns true if the map contains no entries.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Looks up a key and returns a reference to its tree value.
    pub fn get(&self, key: &str) -> Option<TreeRef> {
        self.0
            .iter()
            .find(|&(k, _)| k.as_ref() == key)
            .cloned()
            .map(|(_, v)| v)
    }

    /// Merges entries from another TreeMap into this one.
    pub fn update(&mut self, other: &TreeMap) {
        for (key, value) in other.0.iter() {
            if let Tree::Seq(items) = value.as_ref() {
                for item in items.iter().cloned() {
                    self.insert_as_list(key, item);
                }
            } else if let Tree::List(items) = value.as_ref() {
                for item in items.iter().cloned() {
                    self.insert_as_list(key, item);
                }
            } else {
                self.insert(key, value.clone());
            }
        }
    }

    /// Ensures that the given definition keys exist in the map, inserting defaults if missing.
    pub fn define(&mut self, keys: &[Define]) {
        let mut entries: Vec<(Str, TreeRef)> = self.0.as_ref().to_vec();
        for (k, aslist) in keys {
            let key = self.safe_key(k);
            if !entries.iter().any(|(k, _)| k == &key) {
                let val: TreeRef = if *aslist {
                    Tree::seq(&[]).into()
                } else {
                    Tree::Nil.into()
                };
                entries.push((key, val));
            }
        }
        self.0 = entries.into();
    }

    /// Inserts a value, merging with any existing entry for the same key.
    pub fn insert(&mut self, key: &str, item: TreeRef) {
        let key = self.safe_key(key);
        let mut entries: Vec<(Str, TreeRef)> = self.0.as_ref().to_vec();

        let new_val = if let Some(current) = entries.iter().find(|(k, _)| k == &key) {
            Tree::append(&current.1, &item)
        } else {
            item
        };

        self.update_or_push(&mut entries, key, new_val);
        self.0 = entries.into();
    }

    /// Inserts a value into a list entry for this key.
    pub fn insert_as_list(&mut self, key: &str, item: TreeRef) {
        let key = self.safe_key(key);
        let mut entries: Vec<(Str, TreeRef)> = self.0.as_ref().to_vec();

        let new_val = if let Some(current) = entries.iter().find(|(k, _)| k == &key) {
            Tree::append_as_list(&current.1, &item)
        } else {
            Tree::Seq([item].into()).into()
        };

        self.update_or_push(&mut entries, key, new_val);
        self.0 = entries.into();
    }

    fn update_or_push(&self, entries: &mut Vec<(Str, TreeRef)>, key: Str, val: TreeRef) {
        if let Some(existing) = entries.iter_mut().find(|(k, _)| k == &key) {
            existing.1 = val;
        } else {
            entries.push((key, val));
        }
    }

    fn safe_key(&self, key: &str) -> Str {
        let mut k = key.to_string();
        while self.is_reserved(&k) {
            k.push('_');
        }
        k.into()
    }

    fn is_reserved(&self, key: &str) -> bool {
        matches!(
            key,
            "items" | "keys" | "values" | "get" | "update" | "pop" | "clear"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(s: &str) -> TreeRef {
        Tree::Text(s.into()).into()
    }

    fn seq(items: &[&str]) -> TreeRef {
        Tree::Seq(items.iter().map(|s| text(s)).collect()).into()
    }

    fn list(items: &[&str]) -> TreeRef {
        Tree::List(items.iter().map(|s| text(s)).collect()).into()
    }

    #[test]
    fn insert_zero_times() {
        let map = TreeMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn insert_once() {
        let mut map = TreeMap::new();
        map.insert("foo", text("bar"));
        assert_eq!(map.get("foo"), Some(text("bar")));
    }

    #[test]
    fn insert_twice() {
        let mut map = TreeMap::new();
        map.insert("foo", text("bar"));
        map.insert("foo", text("baz"));
        assert_eq!(
            map.get("foo"),
            Some(Tree::Seq([text("bar"), text("baz")].into()).into())
        );
    }

    #[test]
    fn insert_three_times() {
        let mut map = TreeMap::new();
        map.insert("foo", text("a"));
        map.insert("foo", text("b"));
        map.insert("foo", text("c"));
        assert_eq!(
            map.get("foo"),
            Some(Tree::Seq([text("a"), text("b"), text("c")].into()).into())
        );
    }

    #[test]
    fn insert_list_once() {
        let mut map = TreeMap::new();
        map.insert("foo", seq(&["a", "b"]));
        assert_eq!(map.get("foo"), Some(seq(&["a", "b"])));
    }

    #[test]
    fn insert_list_twice() {
        let mut map = TreeMap::new();
        map.insert("foo", seq(&["a", "b"]));
        map.insert("foo", seq(&["c", "d"]));
        assert_eq!(
            map.get("foo"),
            Some(Tree::Seq([text("a"), text("b"), seq(&["c", "d"])].into()).into())
        );
    }

    #[test]
    fn insert_as_list_zero_times() {
        let map = TreeMap::new();
        assert!(map.is_empty());
    }

    #[test]
    fn insert_as_list_once() {
        let mut map = TreeMap::new();
        map.insert_as_list("foo", text("bar"));
        assert_eq!(map.get("foo"), Some(Tree::Seq([text("bar")].into()).into()));
    }

    #[test]
    fn insert_as_list_twice() {
        let mut map = TreeMap::new();
        map.insert_as_list("foo", text("bar"));
        map.insert_as_list("foo", text("baz"));
        assert_eq!(
            map.get("foo"),
            Some(Tree::Seq([text("bar"), text("baz")].into()).into())
        );
    }

    #[test]
    fn insert_as_list_three_times() {
        let mut map = TreeMap::new();
        map.insert_as_list("foo", text("a"));
        map.insert_as_list("foo", text("b"));
        map.insert_as_list("foo", text("c"));
        assert_eq!(
            map.get("foo"),
            Some(Tree::Seq([text("a"), text("b"), text("c")].into()).into())
        );
    }

    #[test]
    fn insert_as_list_with_list_once() {
        let mut map = TreeMap::new();
        map.insert_as_list("foo", seq(&["a", "b"]));
        assert_eq!(
            map.get("foo"),
            Some(Tree::Seq([seq(&["a", "b"])].into()).into())
        );
    }

    #[test]
    fn insert_as_list_with_seq_twice() {
        let mut map = TreeMap::new();
        map.insert_as_list("foo", seq(&["a", "b"]));
        map.insert_as_list("foo", seq(&["c", "d"]));
        assert_eq!(
            map.get("foo"),
            Some(Tree::Seq([seq(&["a", "b"]), seq(&["c", "d"])].into()).into())
        );
    }

    #[test]
    fn insert_as_list_with_seq_once() {
        let mut map = TreeMap::new();
        map.insert_as_list("foo", list(&["a", "b"]));
        assert_eq!(
            map.get("foo"),
            Some(Tree::Seq([list(&["a", "b"])].into()).into())
        );
    }

    #[test]
    fn insert_as_list_with_list_twice() {
        let mut map = TreeMap::new();
        map.insert_as_list("foo", list(&["a", "b"]));
        map.insert_as_list("foo", list(&["c", "d"]));
        assert_eq!(
            map.get("foo"),
            Some(Tree::Seq([list(&["a", "b"]), list(&["c", "d"])].into()).into())
        );
    }

    #[test]
    fn insert_mixed_with_insert_as_list() {
        let mut map = TreeMap::new();
        map.insert("foo", text("a"));
        map.insert_as_list("foo", text("b"));
        map.insert("foo", text("c"));
        map.insert_as_list("foo", text("d"));

        let expected: TreeRef =
            Tree::Seq([text("a"), text("b"), text("c"), text("d")].into()).into();
        assert_eq!(map.get("foo"), Some(expected));
    }

    #[test]
    fn update_with_empty() {
        let mut map = TreeMap::new();
        map.insert("foo", text("bar"));
        let other = TreeMap::new();
        map.update(&other);
        assert_eq!(map.get("foo"), Some(text("bar")));
    }

    #[test]
    fn update_empty_with_values() {
        let mut map = TreeMap::new();
        let mut other = TreeMap::new();
        other.insert("foo", text("bar"));
        map.update(&other);
        assert_eq!(map.get("foo"), Some(text("bar")));
    }

    #[test]
    fn update_twice_same_key() {
        let mut map = TreeMap::new();
        map.insert("foo", text("a"));

        let mut other = TreeMap::new();
        other.insert("foo", text("b"));
        map.update(&other);

        assert_eq!(
            map.get("foo"),
            Some(Tree::Seq([text("a"), text("b")].into()).into())
        );
    }

    #[test]
    fn update_twice_different_keys() {
        let mut map = TreeMap::new();
        map.insert("foo", text("a"));

        let mut other = TreeMap::new();
        other.insert("bar", text("b"));
        map.update(&other);

        assert_eq!(map.get("foo"), Some(text("a")));
        assert_eq!(map.get("bar"), Some(text("b")));
    }

    #[test]
    fn update_as_list_with_empty() {
        let mut map = TreeMap::new();
        map.insert_as_list("foo", text("a"));

        let other = TreeMap::new();
        map.update(&other);

        assert_eq!(map.get("foo"), Some(Tree::Seq([text("a")].into()).into()));
    }

    #[test]
    fn update_as_list_with_values() {
        let mut map = TreeMap::new();
        map.insert_as_list("foo", text("a"));

        let mut other = TreeMap::new();
        other.insert_as_list("foo", seq(&["b", "c"]));
        map.update(&other);

        assert_eq!(
            map.get("foo"),
            Some(Tree::Seq([text("a"), seq(&["b", "c"])].into()).into())
        );
    }
}
