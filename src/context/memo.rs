// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::trees::tree::Tree;
use crate::types::Str;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct MemoKey {
    pub mark: usize,
    pub name: Str,
    pub memo: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Memo {
    pub tree: Rc<Tree>,
    pub mark: usize,
}

#[derive(Clone, Debug)]
pub struct MemoCache {
    memos: HashMap<MemoKey, Memo>,
}

#[derive(Clone, Default, Debug)]
pub struct KeyTrack {
    pub key: MemoKey,
    pub depth: usize,
}

impl Default for MemoCache {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyTrack {
    pub fn track(&mut self, key: &MemoKey) -> usize {
        if *key == self.key {
            self.depth += 1;
        } else {
            self.key = key.clone();
            self.depth = 1;
        }
        self.depth
    }

    pub fn untrack(&mut self, key: &MemoKey) -> usize {
        if *key == self.key {
            self.depth = self.depth.saturating_sub(1);
            if self.depth == 0 {
                self.key = MemoKey::default();
            }
            self.depth
        } else {
            0
        }
    }
}

impl MemoCache {
    pub fn new() -> Self {
        Self {
            memos: HashMap::new(),
        }
    }

    pub fn clear_error_memos(&mut self) {
        self.memos.retain(|_, memo| *memo.tree != Tree::Bottom);
    }
}

impl MemoCache {
    pub fn key(mark: usize, name: Str, memo: bool) -> MemoKey {
        MemoKey {
            mark,
            name: name.clone(),
            memo,
        }
    }

    pub fn memo(&mut self, key: &MemoKey) -> Option<Memo> {
        self.memos.get(key).cloned()
    }

    pub fn memoize(&mut self, key: &MemoKey, tree: &Rc<Tree>, mark: usize) {
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
        let key = MemoCache::key(0, "rule".into(), true);
        assert!(cache.memo(&key).is_none());
    }

    #[test]
    fn memoize_and_retrieve() {
        let mut cache = MemoCache::new();
        let key = MemoCache::key(0, "rule".into(), true);
        let tree: Rc<Tree> = Tree::Text("test".into()).into();

        cache.memoize(&key, &tree, 5);

        let result = cache.memo(&key);
        assert!(result.is_some());
        assert_eq!(result.unwrap().tree, tree);
    }

    #[test]
    fn memoize_multiple_rules() {
        let mut cache = MemoCache::new();
        let key1 = MemoCache::key(0, "rule1".into(), true);
        let key2 = MemoCache::key(0, "rule2".into(), true);
        let tree1: Rc<Tree> = Tree::Text("a".into()).into();
        let tree2: Rc<Tree> = Tree::Text("b".into()).into();

        cache.memoize(&key1, &tree1, 1);
        cache.memoize(&key2, &tree2, 2);

        assert_eq!(cache.memo(&key1).unwrap().tree, tree1);
        assert_eq!(cache.memo(&key2).unwrap().tree, tree2);
    }

    #[test]
    fn prune_keeps_after_cutpoint() {
        let mut cache = MemoCache::new();
        let key = MemoCache::key(5, "rule".into(), true);
        let tree: Rc<Tree> = Tree::Text("test".into()).into();

        cache.memoize(&key, &tree, 5);
        cache.prune(5);

        assert!(cache.memo(&key).is_some());
    }

    #[test]
    fn prune_removes_before_cutpoint() {
        let mut cache = MemoCache::new();
        let key = MemoCache::key(3, "rule".into(), true);
        let tree: Rc<Tree> = Tree::Text("test".into()).into();

        cache.memoize(&key, &tree, 3);
        cache.prune(5);

        assert!(cache.memo(&key).is_none());
    }

    #[test]
    fn key_equality() {
        let key1 = MemoCache::key(0, "rule".into(), true);
        let key2 = MemoCache::key(0, "rule".into(), true);
        let key3 = MemoCache::key(1, "rule".into(), true);

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }
}
