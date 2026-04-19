// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeSet;
use std::ops::Deref;
use std::{env, fmt};

/// Type aliases for external usage
pub type CfgA<K> = [K];

/// An owned, optimized configuration container.
/// Invariants:
/// 1. Sorted by value (for binary search).
/// 2. Unique values.
#[derive(Clone, Default)]
pub struct CfgBox<K: Ord + Clone + Default> {
    options: Box<[K]>,
}

impl<K: Ord + Clone + Default> From<&CfgA<K>> for CfgBox<K> {
    fn from(cfg: &CfgA<K>) -> Self {
        Self::new(cfg)
    }
}

impl<'a, K: Ord + Clone + Default, const N: usize> From<&'a [K; N]> for CfgBox<K> {
    fn from(options: &'a [K; N]) -> Self {
        Self::new(options.as_slice())
    }
}

/// Has use for a Cfg
pub trait Configurable<K: Ord + Clone + Default> {
    fn configure(&mut self, cfg: &CfgBox<K>) {
        let _ = cfg;
    }
}

/// Maps environment key-value pairs to a configuration type K.
pub trait CfgMapper<K: Ord + Clone + Default> {
    fn map(key: &str, value: &str) -> Option<K>;

    /// Loads configuration from environment variables using this mapper.
    fn load_from_env(prefix: &str) -> CfgBox<K> {
        env::vars()
            .filter(|(key, _)| key.starts_with(prefix))
            .filter_map(|(key, val)| {
                let mut key = key[prefix.len()..].to_string();
                if key.starts_with('_') {
                    key.remove(0);
                }
                Self::map(&key, &val)
            })
            .collect()
    }
}

impl<K: Ord + Clone + Default> CfgMapper<K> for CfgBox<K> {
    fn map(_key: &str, _value: &str) -> Option<K> {
        None
    }
}

impl<K: Ord + Clone + Default> FromIterator<K> for CfgBox<K> {
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        let set: BTreeSet<K> = iter.into_iter().collect();
        Self {
            options: set.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        }
    }
}

impl<K: Ord + Clone + Default> CfgBox<K> {
    /// Creates a new Cfg ensuring all invariants (Sorted, Unique).
    pub fn new(options: &CfgA<K>) -> Self {
        let set: BTreeSet<K> = options.iter().cloned().collect();
        Self {
            options: set.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        }
    }

    pub fn from_boxed_slice(options: Box<[K]>) -> Self {
        let mut vec = options.into_vec();
        vec.sort();
        vec.dedup();
        Self {
            options: vec.into_boxed_slice(),
        }
    }

    /// Merges two configurations (Set Union).
    pub fn merge(&self, other: &CfgBox<K>) -> Self {
        let mut set: BTreeSet<K> = self.options.iter().cloned().collect();
        set.extend(other.options.iter().cloned());

        Self {
            options: set.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        self.options.binary_search(key).is_ok()
    }

    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &K> {
        self.options.iter()
    }

    pub fn as_slice(&self) -> &[K] {
        &self.options
    }
}

impl<K: Ord + Clone + Default> Deref for CfgBox<K> {
    type Target = [K];
    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

impl<K: Ord + Clone + Default + fmt::Debug> fmt::Debug for CfgBox<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
    enum TestOpt {
        #[default]
        None,
        Trace,
        Debug,
        Memoize,
        Strict,
    }

    struct TestMapper;
    impl CfgMapper<TestOpt> for TestMapper {
        fn map(key: &str, value: &str) -> Option<TestOpt> {
            match (key.to_lowercase().as_str(), value) {
                ("trace", "1") => Some(TestOpt::Trace),
                ("memoize", "1") => Some(TestOpt::Memoize),
                ("strict", "1") => Some(TestOpt::Strict),
                _ => None,
            }
        }
    }

    #[test]
    fn test_invariants() {
        let options = [TestOpt::Trace, TestOpt::Strict, TestOpt::Trace];
        let cfg = CfgBox::new(&options);

        assert_eq!(cfg.options.len(), 2); // Unique keys
        assert!(cfg.contains(&TestOpt::Trace));
        assert!(cfg.contains(&TestOpt::Strict));
        assert!(!cfg.contains(&TestOpt::Memoize));

        // Verify sorting
        assert!(cfg.options[0] < cfg.options[1]);
    }

    #[test]
    fn test_merge_logic() {
        let base = CfgBox::new(&[TestOpt::Trace]);
        let over = CfgBox::new(&[TestOpt::Memoize]);
        let merged = base.merge(&over);

        assert!(merged.contains(&TestOpt::Trace));
        assert!(merged.contains(&TestOpt::Memoize));
        assert_eq!(merged.options.len(), 2);
    }

    #[test]
    fn test_from_env() {
        unsafe {
            env::set_var("TEST_TRACE", "1");
            env::set_var("TEST_STRICT", "1");
            env::set_var("TEST_OTHER", "skip");
        }

        let cfg = TestMapper::load_from_env("TEST_");
        assert!(cfg.contains(&TestOpt::Trace));
        assert!(cfg.contains(&TestOpt::Strict));
        assert_eq!(cfg.options.len(), 2);
    }

    #[test]
    fn test_cfg_is_mapper() {
        let cfg = CfgBox::<TestOpt>::load_from_env("EMPTY_");
        assert!(cfg.is_empty());
    }

    #[test]
    fn test_conversions() {
        let options_a = [TestOpt::Trace, TestOpt::Debug];
        let cfg_from_a: CfgBox<TestOpt> = (&options_a).into(); // CfgA -> Cfg
        assert!(cfg_from_a.contains(&TestOpt::Trace));

        let cfg_new = CfgBox::new(&options_a); // CfgA -> Cfg via new
        assert!(cfg_new.contains(&TestOpt::Debug));

        let slice_from_cfg: &[TestOpt] = &cfg_from_a; // Cfg -> &[K] via Deref
        assert_eq!(slice_from_cfg.len(), 2);

        let slice_from_as_slice = cfg_new.as_slice(); // Cfg -> &[K] via as_slice
        assert_eq!(slice_from_as_slice.len(), 2);

        let iter_from_cfg = cfg_from_a.iter(); // Cfg -> Iterator
        assert_eq!(iter_from_cfg.count(), 2);

        let vec_from_iter: CfgBox<TestOpt> =
            vec![TestOpt::Trace, TestOpt::Memoize].into_iter().collect(); // Vec<K> -> Cfg via FromIterator
        assert!(vec_from_iter.contains(&TestOpt::Memoize));
    }
}
