// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::ops::Deref;
use std::{env, fmt};

/// Type aliases for external usage
pub type CfgA<K> = [K];

/// An owned, optimized configuration container.
/// Invariants:
/// 1. Unique values.
/// 2. Immutable internals (value-based API).
#[derive(Clone, Default)]
pub struct Cfg<K: Clone + Default + Send + Sync> {
    cfga: Box<[K]>,
}

unsafe impl<K: Clone + Default + Send + Sync> Send for Cfg<K> {}
unsafe impl<K: Clone + Default + Send + Sync> Sync for Cfg<K> {}

impl<'a, K: Clone + Default + Send + Sync + PartialEq, const N: usize> From<&'a [K; N]> for Cfg<K> {
    fn from(options: &'a [K; N]) -> Self {
        Self::new(options.as_slice())
    }
}

/// Has use for a Cfg
pub trait Configurable<K: Clone + Default + Send + Sync> {
    fn configure(&mut self, cfg: &Cfg<K>) {
        let _ = cfg;
    }
}

/// Maps environment key-value pairs to a configuration type K.
pub trait CfgMapper<K: Clone + Default + Send + Sync + PartialEq> {
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

impl<K> CfgMapper<K> for Cfg<K>
where
    K: Clone + Default + Send + Sync + PartialEq + CfgMapper<K>,
{
    fn map(key: &str, value: &str) -> Option<K> {
        K::map(key, value)
    }

    fn unmap(value: &K) -> Option<(&str, &str)> {
        K::unmap(value)
    }
}

impl<K: Clone + Default + Send + Sync + PartialEq> FromIterator<K> for Cfg<K> {
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        let mut vec = Vec::new();
        for item in iter {
            if !vec.contains(&item) {
                vec.push(item);
            }
        }
        Self {
            cfga: vec.into_boxed_slice(),
        }
    }
}

impl<K: Clone + Default + Send + Sync + PartialEq> Cfg<K> {
    /// Creates a new Cfg ensuring all invariants (Unique).
    pub fn new(options: &CfgA<K>) -> Self {
        let mut vec = Vec::with_capacity(options.len());
        for opt in options.iter() {
            if !vec.contains(opt) {
                vec.push(opt.clone());
            }
        }
        Self {
            cfga: vec.into_boxed_slice(),
        }
    }

    #[allow(clippy::boxed_local)]
    pub fn from_boxed_slice(options: Box<[K]>) -> Self {
        let mut vec = Vec::with_capacity(options.len());
        for opt in options.iter() {
            if !vec.contains(opt) {
                vec.push(opt.clone());
            }
        }
        Self {
            cfga: vec.into_boxed_slice(),
        }
    }

    /// Merges two configurations (Set Union).
    pub fn merge(&self, other: &Cfg<K>) -> Self {
        let mut vec = self.cfga.to_vec();
        for opt in other.iter() {
            if !vec.contains(opt) {
                vec.push(opt.clone());
            }
        }
        Self {
            cfga: vec.into_boxed_slice(),
        }
    }

    pub fn add(&self, key: K) -> Self {
        if self.contains(&key) {
            self.clone()
        } else {
            let mut vec = self.cfga.to_vec();
            vec.push(key);
            Self {
                cfga: vec.into_boxed_slice(),
            }
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        self.cfga.iter().any(|k| k == key)
    }
}

impl<K: Clone + Default + Send + Sync> Cfg<K> {
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

impl<K: Clone + Default + Send + Sync> Deref for Cfg<K> {
    type Target = [K];
    fn deref(&self) -> &Self::Target {
        &self.cfga
    }
}

impl<K: Clone + Default + Send + Sync + fmt::Debug> fmt::Debug for Cfg<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[derive(Debug, Clone, PartialEq, Default)]
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

        // Verify insertion order preserved
        assert_eq!(cfg.cfga[0], TestOpt::Trace);
        assert_eq!(cfg.cfga[1], TestOpt::Strict);
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
        let cfg_from_a: Cfg<TestOpt> = (&options_a).into();
        assert!(cfg_from_a.contains(&TestOpt::Trace));

        let cfg_new = Cfg::new(&options_a);
        assert!(cfg_new.contains(&TestOpt::Debug));

        let slice_from_cfg: &[TestOpt] = &cfg_from_a;
        assert_eq!(slice_from_cfg.len(), 2);

        let slice_from_as_slice = cfg_new.as_slice();
        assert_eq!(slice_from_as_slice.len(), 2);

        let iter_from_cfg = cfg_from_a.iter();
        assert_eq!(iter_from_cfg.count(), 2);

        let vec_from_iter: Cfg<TestOpt> =
            vec![TestOpt::Trace, TestOpt::Memoize].into_iter().collect();
        assert!(vec_from_iter.contains(&TestOpt::Memoize));
        Ok(())
    }
}
