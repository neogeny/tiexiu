// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeMap;
use std::ops::Index;
use std::str::FromStr;
use std::{env, fmt};

/// Python-style falsy values for string-based configuration.
/// Note: Sorted for potential binary search optimization.
pub const FALSY_VALUES: &[&str] = &["0", "False", "No", "false", "no", "none"];

/// Helper to determine if a string is "falsy" in a Pythonic context.
pub fn is_falsy(v: &str) -> bool {
    v.is_empty() || FALSY_VALUES.iter().any(|&f| f.eq_ignore_ascii_case(v))
}

/// Type aliases for external usage
pub type CfgA<'c> = &'c [(&'c str, &'c str)];

/// An owned, optimized configuration container.
/// Generic over K to allow Enums for checked keys. Defaults to Box<str>.
#[derive(Clone, Default)]
pub struct Cfg<K = Box<str>>
where
    K: AsRef<str> + Ord,
{
    pairs: Box<[(K, Box<str>)]>,
}

/// Has use for a Cfg
pub trait Configurable<K = Box<str>>
where
    K: AsRef<str> + Ord,
{
    fn configure(&mut self, cfg: &Cfg<K>) {
        let _ = cfg;
    }
}

impl<K> Cfg<K>
where
    K: AsRef<str> + Ord + Clone,
{
    /// Creates a new Cfg from borrowed slices ensuring all invariants.
    pub fn new(pairs: &[(&str, &str)]) -> Self
    where
        K: From<Box<str>>,
    {
        let map: BTreeMap<K, Box<str>> = pairs
            .iter()
            .map(|(k, v)| (K::from(Box::from(*k)), Box::from(*v)))
            .collect();
        Self::from_map(map)
    }

    /// Internal helper to freeze a map into the optimized storage.
    fn from_map(map: BTreeMap<K, Box<str>>) -> Self {
        Self {
            pairs: map.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        }
    }

    /// Pulls configuration from environment variables.
    /// Returns string-based Cfg which can then be converted to typed.
    pub fn from_env(prefix: &str) -> Cfg<Box<str>> {
        let map: BTreeMap<Box<str>, Box<str>> = env::vars()
            .filter(|(key, _)| key.starts_with(prefix))
            .map(|(key, val)| {
                let mut key_str = key[prefix.len()..].to_string();
                if key_str.starts_with('_') {
                    key_str.remove(0);
                }
                (
                    key_str.to_lowercase().into_boxed_str(),
                    val.into_boxed_str(),
                )
            })
            .collect();

        Cfg::<Box<str>>::from_map(map)
    }

    /// Transition from a string-based Cfg to a Typed Enum Cfg, ignoring invalid keys.
    pub fn into_typed<KT>(self) -> Cfg<KT>
    where
        KT: AsRef<str> + Ord + FromStr + Clone,
    {
        let map: BTreeMap<KT, Box<str>> = self
            .pairs
            .into_vec()
            .into_iter()
            .filter_map(|(k, v)| {
                // We use .ok() to ignore Result errors and skip the key
                KT::from_str(k.as_ref())
                    .ok()
                    .map(|typed_key| (typed_key, v))
            })
            .collect();
        Cfg::<KT>::from_map(map)
    }

    pub fn insert(&mut self, key: K, value: &str) {
        let mut map: BTreeMap<K, Box<str>> = self.pairs.iter().cloned().collect();
        map.insert(key, Box::from(value));
        *self = Self::from_map(map);
    }

    pub fn merge(&self, other: &Self) -> Self {
        let mut map: BTreeMap<K, Box<str>> = self.pairs.iter().cloned().collect();

        for (k, v) in other.pairs.iter() {
            if let Some(u) = map.get(k)
                && !is_falsy(u)
            {
                continue;
            }
            map.insert(k.clone(), v.clone());
        }

        Self::from_map(map)
    }

    /// Optimized $O(\log n)$ lookup.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.pairs
            .binary_search_by(|(k, _)| k.as_ref().cmp(key))
            .ok()
            .map(|idx| self.pairs[idx].1.as_ref())
    }

    pub fn is_enabled(&self, key: &str) -> bool {
        self.get(key).map(|v| !is_falsy(v)).unwrap_or(false)
    }

    pub fn get_or<T: FromStr>(&self, key: &str, default: T) -> T {
        self.get(key)
            .and_then(|v| v.parse::<T>().ok())
            .unwrap_or(default)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.pairs.iter().map(|(k, v)| (k.as_ref(), v.as_ref()))
    }
}

/// Allows indexing by string slice regardless of the internal key type K.
impl<K> Index<&str> for Cfg<K>
where
    K: AsRef<str> + Ord + Clone,
{
    type Output = str;
    fn index(&self, key: &str) -> &Self::Output {
        self.get(key).unwrap_or("")
    }
}

impl<K> fmt::Debug for Cfg<K>
where
    K: AsRef<str> + Ord + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum TestKeys {
        Trace,
        Mode,
    }

    impl AsRef<str> for TestKeys {
        fn as_ref(&self) -> &str {
            match self {
                Self::Trace => "trace",
                Self::Mode => "mode",
            }
        }
    }

    impl FromStr for TestKeys {
        type Err = ();
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            match s {
                "trace" => Ok(Self::Trace),
                "mode" => Ok(Self::Mode),
                _ => Err(()),
            }
        }
    }

    #[test]
    fn test_typed_conversion() {
        // Start with raw string config
        let raw = Cfg::<Box<str>>::new(&[
            ("trace", "1"),
            ("mode", "fast"),
            ("unknown_key", "ignore_me"),
        ]);

        // Convert to typed
        let typed = raw.into_typed::<TestKeys>();

        assert_eq!(typed.get("trace"), Some("1"));
        assert_eq!(typed.get("mode"), Some("fast"));
        // The unknown key should be gone
        assert_eq!(typed.pairs.len(), 2);
    }

    #[test]
    fn test_binary_search_logic() {
        let cfg = Cfg::<Box<str>>::new(&[("z", "last"), ("a", "first"), ("m", "middle")]);
        assert_eq!(cfg.get("a"), Some("first"));
        assert_eq!(cfg.get("m"), Some("middle"));
    }

    #[test]
    fn test_is_falsy_variants() {
        assert!(is_falsy("0"));
        assert!(is_falsy("No"));
        assert!(is_falsy("none"));
        assert!(!is_falsy("true"));
    }
}
