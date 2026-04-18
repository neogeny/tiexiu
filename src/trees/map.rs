// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::tree::Tree;
use indexmap::IndexMap;

pub type MapEntries = IndexMap<Box<str>, Tree>;

/// A structured mapping for AST nodes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeMap {
    pub entries: MapEntries,
}

impl TreeMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&Tree> {
        self.entries.get(key)
    }

    pub fn update(&mut self, other: &TreeMap) {
        for (key, value) in &other.entries {
            if let Tree::List(items) = value {
                for item in items.iter() {
                    self.insert_as_list(key, item.clone());
                }
            } else if let Tree::Closed(items) = value {
                for item in items.iter() {
                    self.insert_as_list(key, item.clone());
                }
            } else {
                self.insert(key, value.clone());
            }
        }
    }

    pub fn define(&mut self, keys: &[&str], list_keys: &[&str]) {
        for &k in keys {
            let key = self.safe_key(k);
            self.entries.entry(key).or_insert(Tree::Nil);
        }

        for &k in list_keys {
            let key = self.safe_key(k);
            self.entries.entry(key).or_insert(Tree::List([].into()));
        }
    }

    pub fn insert(&mut self, key: &str, item: Tree) {
        let key = self.safe_key(key);
        let mut new = item;
        if let Some(current) = self.entries.get(&key) {
            new = current.clone().append(new);
        }
        self.entries.insert(key, new);
    }

    pub fn insert_as_list(&mut self, key: &str, item: Tree) {
        let key = self.safe_key(key);
        let mut new = item;
        if let Some(current) = self.entries.get(&key) {
            new = current.clone().append_as_list(new);
        } else {
            new = Tree::List([new].into());
        }
        self.entries.insert(key, new);
    }

    fn safe_key(&self, key: &str) -> Box<str> {
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

    fn text(s: &str) -> Tree {
        Tree::Text(s.into())
    }

    fn list(items: &[&str]) -> Tree {
        Tree::List(items.iter().map(|s| text(s)).collect())
    }

    fn closed(items: &[&str]) -> Tree {
        Tree::Closed(items.iter().map(|s| text(s)).collect())
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
        assert_eq!(map.get("foo"), Some(&text("bar")));
    }

    #[test]
    fn insert_twice() {
        let mut map = TreeMap::new();
        map.insert("foo", text("bar"));
        map.insert("foo", text("baz"));
        assert_eq!(
            map.get("foo"),
            Some(&Tree::List([text("bar"), text("baz")].into()))
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
            Some(&Tree::List([text("a"), text("b"), text("c")].into()))
        );
    }

    #[test]
    fn insert_list_once() {
        let mut map = TreeMap::new();
        map.insert("foo", list(&["a", "b"]));
        assert_eq!(map.get("foo"), Some(&list(&["a", "b"])));
    }

    #[test]
    fn insert_list_twice() {
        let mut map = TreeMap::new();
        map.insert("foo", list(&["a", "b"]));
        map.insert("foo", list(&["c", "d"]));
        assert_eq!(
            map.get("foo"),
            Some(&Tree::List(
                [text("a"), text("b"), list(&["c", "d"])].into()
            ))
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
        assert_eq!(map.get("foo"), Some(&Tree::List([text("bar")].into())));
    }

    #[test]
    fn insert_as_list_twice() {
        let mut map = TreeMap::new();
        map.insert_as_list("foo", text("bar"));
        map.insert_as_list("foo", text("baz"));
        assert_eq!(
            map.get("foo"),
            Some(&Tree::List([text("bar"), text("baz")].into()))
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
            Some(&Tree::List([text("a"), text("b"), text("c")].into()))
        );
    }

    #[test]
    fn insert_as_list_with_list_once() {
        let mut map = TreeMap::new();
        map.insert_as_list("foo", list(&["a", "b"]));
        assert_eq!(
            map.get("foo"),
            Some(&Tree::List([list(&["a", "b"])].into()))
        );
    }

    #[test]
    fn insert_as_list_with_list_twice() {
        let mut map = TreeMap::new();
        map.insert_as_list("foo", list(&["a", "b"]));
        map.insert_as_list("foo", list(&["c", "d"]));
        assert_eq!(
            map.get("foo"),
            Some(&Tree::List([list(&["a", "b"]), list(&["c", "d"])].into()))
        );
    }

    #[test]
    fn insert_as_list_with_closed_once() {
        let mut map = TreeMap::new();
        map.insert_as_list("foo", closed(&["a", "b"]));
        assert_eq!(
            map.get("foo"),
            Some(&Tree::List([closed(&["a", "b"])].into()))
        );
    }

    #[test]
    fn insert_as_list_with_closed_twice() {
        let mut map = TreeMap::new();
        map.insert_as_list("foo", closed(&["a", "b"]));
        map.insert_as_list("foo", closed(&["c", "d"]));
        assert_eq!(
            map.get("foo"),
            Some(&Tree::List(
                [closed(&["a", "b"]), closed(&["c", "d"])].into()
            ))
        );
    }

    #[test]
    fn insert_mixed_with_insert_as_list() {
        let mut map = TreeMap::new();
        map.insert("foo", text("a"));
        map.insert_as_list("foo", text("b"));
        map.insert("foo", text("c"));
        map.insert_as_list("foo", text("d"));

        let expected = Tree::List([text("a"), text("b"), text("c"), text("d")].into());
        assert_eq!(map.get("foo"), Some(&expected));
    }

    #[test]
    fn update_with_empty() {
        let mut map = TreeMap::new();
        map.insert("foo", text("bar"));
        let other = TreeMap::new();
        map.update(&other);
        assert_eq!(map.get("foo"), Some(&text("bar")));
    }

    #[test]
    fn update_empty_with_values() {
        let mut map = TreeMap::new();
        let mut other = TreeMap::new();
        other.insert("foo", text("bar"));
        map.update(&other);
        assert_eq!(map.get("foo"), Some(&text("bar")));
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
            Some(&Tree::List([text("a"), text("b")].into()))
        );
    }

    #[test]
    fn update_twice_different_keys() {
        let mut map = TreeMap::new();
        map.insert("foo", text("a"));

        let mut other = TreeMap::new();
        other.insert("bar", text("b"));
        map.update(&other);

        assert_eq!(map.get("foo"), Some(&text("a")));
        assert_eq!(map.get("bar"), Some(&text("b")));
    }

    #[test]
    fn update_as_list_with_empty() {
        let mut map = TreeMap::new();
        map.insert_as_list("foo", text("a"));

        let other = TreeMap::new();
        map.update(&other);

        assert_eq!(map.get("foo"), Some(&Tree::List([text("a")].into())));
    }

    #[test]
    fn update_as_list_with_values() {
        let mut map = TreeMap::new();
        map.insert_as_list("foo", text("a"));

        let mut other = TreeMap::new();
        other.insert_as_list("foo", list(&["b", "c"]));
        map.update(&other);

        assert_eq!(
            map.get("foo"),
            Some(&Tree::List([text("a"), list(&["b", "c"])].into()))
        );
    }
}
