// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::trees::short::BOTTOM;
use crate::trees::tree::Tree;
use crate::types::{FastIndexSet, Str};
use ahash::{AHashMap, RandomState};
use std::rc::Rc;

/// Key for memoizing a parse result at a specific position for a named rule.
#[derive(Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct MemoKey {
    /// The cursor position (mark) where the rule was evaluated.
    pub mark: usize,
    /// The name of the rule being memoized.
    pub name: Str,
    /// Whether memoization is enabled for this key.
    pub can_memo: bool,
}

/// A memoized parse result containing the produced tree and end position.
#[derive(Clone, Debug, PartialEq)]
pub struct Memo {
    /// The resulting parse tree.
    pub tree: Rc<Tree>,
    /// The cursor position after the matched rule.
    pub mark: usize,
}

/// A cache for memoized parse results, keyed by position and rule name.
#[derive(Clone, Debug)]
pub struct MemoCache {
    strings: FastIndexSet<Str>,
    memos: AHashMap<MemoKey, Memo>,
}

/// Tracks the recursion depth of a memo key for detecting left-recursion loops.
#[derive(Clone, Default, Debug)]
pub struct KeyTrack {
    /// The key being tracked.
    pub key: MemoKey,
    /// The current recursion depth.
    pub depth: usize,
}

impl Default for MemoCache {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyTrack {
    /// Increments depth if `key` matches the current tracked key, otherwise resets.
    pub fn track(&mut self, key: &MemoKey) -> usize {
        if *key == self.key {
            self.depth += 1;
        } else {
            self.key = key.clone();
            self.depth = 1;
        }
        self.depth
    }

    /// Decrements depth if `key` matches the current tracked key.
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
    /// Creates a new empty `MemoCache`.
    pub fn new() -> Self {
        Self {
            strings: FastIndexSet::with_hasher(RandomState::new()),
            memos: AHashMap::new(),
        }
    }

    /// Removes all memo entries whose result is `Tree::Bottom` (failed parse markers).
    pub fn clear_error_memos(&mut self) {
        self.memos.retain(|_, memo| *memo.tree != Tree::Bottom);
    }

    /// Interns a string, returning a reference-counted copy from the cache.
    pub fn intern(&mut self, s: &str) -> Str {
        if let Some(existing) = self.strings.get(s) {
            return existing.clone();
        }

        let new: Str = s.into();
        self.strings.insert(new.clone());
        new
    }

    /// Interns a string and returns its index in the string table.
    pub fn intern_index(&mut self, s: Str) -> usize {
        if let Some(index) = self.strings.get_index_of(&s) {
            return index;
        }

        let (index, _) = self.strings.insert_full(s);
        index
    }
}

impl MemoCache {
    /// Creates a `MemoKey` from position, rule name, and memoization flag.
    pub fn key(&mut self, mark: usize, name: Str, can_memo: bool) -> MemoKey {
        MemoKey {
            mark,
            name,
            can_memo,
        }
    }

    /// Looks up a memoized result for the given key.
    pub fn memo(&mut self, key: &MemoKey) -> Option<Memo> {
        self.memos.get(key).cloned()
    }

    /// Stores a memoized parse result, respecting the key's `can_memo` flag.
    pub fn memoize(&mut self, key: &MemoKey, tree: &Rc<Tree>, mark: usize) {
        if !key.can_memo {
            return;
        }
        let memo = Memo {
            tree: tree.clone(),
            mark,
        };
        self.memos.insert(key.clone(), memo);
    }

    /// Removes all memo entries whose start position is before `cutpoint`,
    /// except for `Tree::Bottom` sentinels.
    pub fn prune(&mut self, cutpoint: usize) {
        self.memos
            .retain(|key, memo| key.mark >= cutpoint && *memo.tree != BOTTOM);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trees::Tree;

    #[test]
    fn new_cache_is_empty() {
        let mut cache = MemoCache::new();
        let key = cache.key(0, "rule".into(), true);
        assert!(cache.memo(&key).is_none());
    }

    #[test]
    fn memoize_and_retrieve() {
        let mut cache = MemoCache::new();
        let key = cache.key(0, "rule".into(), true);
        let tree: Rc<Tree> = Tree::Text("test".into()).into();

        cache.memoize(&key, &tree, 5);

        let result = cache.memo(&key);
        assert!(result.is_some());
        assert_eq!(result.unwrap().tree, tree);
    }

    #[test]
    fn memoize_multiple_rules() {
        let mut cache = MemoCache::new();
        let key1 = cache.key(0, "rule1".into(), true);
        let key2 = cache.key(0, "rule2".into(), true);
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
        let key = cache.key(5, "rule".into(), true);
        let tree: Rc<Tree> = Tree::Text("test".into()).into();

        cache.memoize(&key, &tree, 5);
        cache.prune(5);

        assert!(cache.memo(&key).is_some());
    }

    #[test]
    fn prune_removes_before_cutpoint() {
        let mut cache = MemoCache::new();
        let key = cache.key(3, "rule".into(), true);
        let tree: Rc<Tree> = Tree::Text("test".into()).into();

        cache.memoize(&key, &tree, 3);
        cache.prune(5);

        assert!(cache.memo(&key).is_none());
    }

    #[test]
    fn key_equality() {
        let mut cache = MemoCache::new();
        let key1 = cache.key(0, "rule".into(), true);
        let key2 = cache.key(0, "rule".into(), true);
        let key3 = cache.key(1, "rule".into(), true);

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }
}
