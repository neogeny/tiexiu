// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Rule;
use crate::trees::tree::Tree;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Key {
    pub mark: usize,
    pub name: String,
    pub memo: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Memo {
    pub tree: Tree,
    pub mark: usize,
}

#[derive(Clone, Debug)]
pub struct MemoCache {
    memos: HashMap<Key, Memo>,
}

impl Default for MemoCache {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoCache {
    pub fn new() -> Self {
        Self {
            memos: HashMap::new(),
        }
    }

    pub fn clear_error_memos(&mut self) {
        self.memos.retain(|_, memo| memo.tree != Tree::Bottom);
    }
}

impl MemoCache {
    pub fn key(mark: usize, name: &str, memo: bool) -> Key {
        Key {
            mark,
            name: name.to_string(),
            memo,
        }
    }

    pub fn rule_key(mark: usize, rule: &Rule) -> Key {
        Key {
            mark,
            name: rule.name.to_string(),
            memo: rule.is_memoizable(),
        }
    }

    pub fn memo(&mut self, key: &Key) -> Option<Memo> {
        self.memos.get(key).cloned()
    }

    pub fn memoize(&mut self, key: &Key, tree: &Tree, mark: usize) {
        if !key.memo {
            return;
        }
        let memo = Memo {
            tree: tree.clone(),
            mark,
        };
        self.memos.insert(key.clone(), memo);
    }

    pub fn prune(&mut self, cutpoint: usize) {
        self.memos.retain(|key, _| key.mark >= cutpoint);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trees::Tree;

    #[test]
    fn new_cache_is_empty() {
        let mut cache = MemoCache::new();
        let key = MemoCache::key(0, "rule", true);
        assert!(cache.memo(&key).is_none());
    }

    #[test]
    fn memoize_and_retrieve() {
        let mut cache = MemoCache::new();
        let key = MemoCache::key(0, "rule", true);
        let tree = Tree::Text("test".into());

        cache.memoize(&key, &tree, 5);

        let result = cache.memo(&key);
        assert!(result.is_some());
        assert_eq!(result.unwrap().tree, tree);
    }

    #[test]
    fn memoize_multiple_rules() {
        let mut cache = MemoCache::new();
        let key1 = MemoCache::key(0, "rule1", true);
        let key2 = MemoCache::key(0, "rule2", true);
        let tree1 = Tree::Text("a".into());
        let tree2 = Tree::Text("b".into());

        cache.memoize(&key1, &tree1, 1);
        cache.memoize(&key2, &tree2, 2);

        assert_eq!(cache.memo(&key1).unwrap().tree, tree1);
        assert_eq!(cache.memo(&key2).unwrap().tree, tree2);
    }

    #[test]
    fn prune_keeps_after_cutpoint() {
        let mut cache = MemoCache::new();
        let key = MemoCache::key(5, "rule", true);
        let tree = Tree::Text("test".into());

        cache.memoize(&key, &tree, 5);
        cache.prune(5);

        assert!(cache.memo(&key).is_some());
    }

    #[test]
    fn prune_removes_before_cutpoint() {
        let mut cache = MemoCache::new();
        let key = MemoCache::key(3, "rule", true);
        let tree = Tree::Text("test".into());

        cache.memoize(&key, &tree, 3);
        cache.prune(5);

        assert!(cache.memo(&key).is_none());
    }

    #[test]
    fn key_equality() {
        let key1 = MemoCache::key(0, "rule", true);
        let key2 = MemoCache::key(0, "rule", true);
        let key3 = MemoCache::key(1, "rule", true);

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }
}
