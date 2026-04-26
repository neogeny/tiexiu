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
pub struct Cfg<K: Ord + Clone + Default> {
    cfga: Box<CfgA<K>>,
}

// impl<K: Ord + Clone + Default> From<&CfgA<K>> for CfgBox<K> {
//     fn from(cfg: &CfgA<K>) -> Self {
//         Self::new(cfg)
//     }
// }

impl<'a, K: Ord + Clone + Default, const N: usize> From<&'a [K; N]> for Cfg<K> {
    fn from(options: &'a [K; N]) -> Self {
        Self::new(options.as_slice())
    }
}

/// Has use for a Cfg
pub trait Configurable<K: Ord + Clone + Default> {
    fn configure(&mut self, cfg: &Cfg<K>) {
        let _ = cfg;
    }
}

/// Maps environment key-value pairs to a configuration type K.
pub trait CfgMapper<K: Ord + Clone + Default> {
    fn map(key: &str, value: &str) -> Option<K>;
    fn unmap(_value: &K) -> Option<(&str, &str)> {
        None
    }

    /// Loads configuration from environment variables using this mapper.
    fn load_from_env(prefix: &str) -> Cfg<K> {
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

impl<K: Ord + Clone + Default> CfgMapper<K> for Cfg<K>
where
    K: CfgMapper<K>,
{
    fn map(key: &str, value: &str) -> Option<K> {
        K::map(key, value)
    }

    fn unmap(value: &K) -> Option<(&str, &str)> {
        K::unmap(value)
    }
}

impl<K: Ord + Clone + Default> FromIterator<K> for Cfg<K> {
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        let set: BTreeSet<K> = iter.into_iter().collect();
        Self {
            cfga: set.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        }
    }
}

impl<K: Ord + Clone + Default> Cfg<K> {
    /// Creates a new Cfg ensuring all invariants (Sorted, Unique).
    pub fn new(options: &CfgA<K>) -> Self {
        let set: BTreeSet<K> = options.iter().cloned().collect();
        Self {
            cfga: set.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        }
    }

    pub fn from_boxed_slice(options: Box<[K]>) -> Self {
        let mut vec = options.into_vec();
        vec.sort();
        vec.dedup();
        Self {
            cfga: vec.into_boxed_slice(),
        }
    }

    /// Merges two configurations (Set Union).
    pub fn merge(&self, other: &Cfg<K>) -> Self {
        let mut set: BTreeSet<K> = self.cfga.iter().cloned().collect();
        set.extend(other.iter().cloned());

        Self {
            cfga: set.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        }
    }

    pub fn add(&self, key: K) -> Self {
        if self.contains(&key) {
            self.clone()
        } else {
            self.merge(&Cfg::new(&[key]))
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        self.cfga.binary_search(key).is_ok()
    }

    pub fn is_empty(&self) -> bool {
        self.cfga.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &K> {
        self.cfga.iter()
    }

    pub fn as_slice(&self) -> &[K] {
        &self.cfga
    }
}

impl<K: Ord + Clone + Default> Deref for Cfg<K> {
    type Target = [K];
    fn deref(&self) -> &Self::Target {
        &self.cfga
    }
}

impl<K: Ord + Clone + Default + fmt::Debug> fmt::Debug for Cfg<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
    enum TestOpt {
        #[default]
        None,
        Trace,
        Debug,
        Memoize,
        Strict,
    }

    impl CfgMapper<TestOpt> for TestOpt {
        fn map(key: &str, value: &str) -> Option<TestOpt> {
            match (key.to_lowercase().as_str(), value) {
                ("trace", "1") => Some(TestOpt::Trace),
                ("memoize", "1") => Some(TestOpt::Memoize),
                ("strict", "1") => Some(TestOpt::Strict),
                _ => None,
            }
        }

        fn unmap(value: &TestOpt) -> Option<(&str, &str)> {
            let _ = value;
            todo!()
        }
    }

    #[test]
    fn test_invariants() -> Result<()> {
        let options = [TestOpt::Trace, TestOpt::Strict, TestOpt::Trace];
        let cfg = Cfg::new(&options);

        assert_eq!(cfg.cfga.len(), 2); // Unique keys
        assert!(cfg.contains(&TestOpt::Trace));
        assert!(cfg.contains(&TestOpt::Strict));
        assert!(!cfg.contains(&TestOpt::Memoize));

        // Verify sorting
        assert!(cfg.cfga[0] < cfg.cfga[1]);
        Ok(())
    }

    #[test]
    fn test_merge_logic() -> Result<()> {
        let base = Cfg::new(&[TestOpt::Trace]);
        let over = Cfg::new(&[TestOpt::Memoize]);
        let merged = base.merge(&over);

        assert!(merged.contains(&TestOpt::Trace));
        assert!(merged.contains(&TestOpt::Memoize));
        assert_eq!(merged.cfga.len(), 2);
        Ok(())
    }

    #[test]
    fn test_from_env() -> Result<()> {
        unsafe {
            env::set_var("TEST_TRACE", "1");
            env::set_var("TEST_STRICT", "1");
            env::set_var("TEST_OTHER", "skip");
        }

        let cfg: Cfg<TestOpt> = TestOpt::load_from_env("TEST_");
        assert!(cfg.contains(&TestOpt::Trace));
        assert!(cfg.contains(&TestOpt::Strict));
        assert_eq!(cfg.cfga.len(), 2);
        Ok(())
    }

    #[test]
    fn test_cfg_is_mapper() -> Result<()> {
        let cfg = Cfg::<TestOpt>::load_from_env("EMPTY_");
        assert!(cfg.is_empty());
        Ok(())
    }

    #[test]
    fn test_conversions() -> Result<()> {
        let options_a = [TestOpt::Trace, TestOpt::Debug];
        let cfg_from_a: Cfg<TestOpt> = (&options_a).into(); // CfgA -> Cfg
        assert!(cfg_from_a.contains(&TestOpt::Trace));

        let cfg_new = Cfg::new(&options_a); // CfgA -> Cfg via new
        assert!(cfg_new.contains(&TestOpt::Debug));

        let slice_from_cfg: &[TestOpt] = &cfg_from_a; // Cfg -> &[K] via Deref
        assert_eq!(slice_from_cfg.len(), 2);

        let slice_from_as_slice = cfg_new.as_slice(); // Cfg -> &[K] via as_slice
        assert_eq!(slice_from_as_slice.len(), 2);

        let iter_from_cfg = cfg_from_a.iter(); // Cfg -> Iterator
        assert_eq!(iter_from_cfg.count(), 2);

        let vec_from_iter: Cfg<TestOpt> =
            vec![TestOpt::Trace, TestOpt::Memoize].into_iter().collect(); // Vec<K> -> Cfg via FromIterator
        assert!(vec_from_iter.contains(&TestOpt::Memoize));
        Ok(())
    }
}
