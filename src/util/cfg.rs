// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::error::{Error, Result};
use std::collections::BTreeMap;
use std::ops::Index;
use std::str::FromStr;
use std::{env, fmt};

/// Python-style falsy values for string-based configuration.
pub const FALSY_VALUES: &[&str] = &["false", "0", "no", "none", "False", "No"];

/// Helper to determine if a string is "falsy" in a Pythonic context.
pub fn is_falsy(v: &str) -> bool {
    v.is_empty() || FALSY_VALUES.contains(&v.to_lowercase().as_str())
}

/// Type aliases for external usage
pub type CfgA<'c> = &'c [(&'c str, &'c str)];
pub type CfgR<'c> = Box<[(&'c str, &'c str)]>;

/// An owned, optimized configuration container.
/// Invariants: Sorted by key, Unique keys, Latter wins on construction.
#[derive(Clone, Default)]
pub struct Cfg {
    pairs: Box<[(Box<str>, Box<str>)]>,
}

/// Has use for a Cfg
pub trait Configurable {
    fn configure(&mut self, cfg: &Cfg) {
        let _ = cfg;
    }
}

impl<K, V> FromIterator<(K, V)> for Cfg
where
    K: Into<Box<str>>,
    V: Into<Box<str>>,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let map: BTreeMap<Box<str>, Box<str>> = iter
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        Self::from_map(map)
    }
}

impl Cfg {
    /// Creates a new Cfg from borrowed slices ensuring all invariants.
    pub fn new(pairs: CfgA) -> Self {
        let map: BTreeMap<Box<str>, Box<str>> = pairs
            .iter()
            .map(|(k, v)| (Box::from(*k), Box::from(*v)))
            .collect();
        Self::from_map(map)
    }

    /// Internal helper to freeze a map into the optimized storage.
    fn from_map(map: BTreeMap<Box<str>, Box<str>>) -> Self {
        Self {
            pairs: map.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        }
    }

    /// Validates keys against a whitelist before creation.
    pub fn check_new(pairs: CfgA, valid: &[&str]) -> Result<Self> {
        for (name, _) in pairs {
            if !valid.contains(name) {
                return Err(Error::UnknownCfgOption((**name).into()));
            }
        }
        Ok(Self::new(pairs))
    }

    /// Pulls configuration from environment variables with a specific prefix.
    pub fn from_env(prefix: &str) -> Self {
        let map: BTreeMap<Box<str>, Box<str>> = env::vars()
            .filter(|(key, _)| key.starts_with(prefix))
            .map(|(key, val)| {
                let mut key = key[prefix.len()..].to_string();
                if key.starts_with('_') {
                    key.remove(0);
                }
                (key.to_lowercase().into_boxed_str(), val.into_boxed_str())
            })
            .collect();

        Self::from_map(map)
    }

    pub fn from_boxed_slice(pairs: Box<[(&'_ str, &'_ str)]>) -> Self {
        Self::new(pairs.as_ref())
    }

    pub fn as_boxed_slice(&self) -> Box<[(&str, &str)]> {
        self.pairs
            .iter()
            .map(|(k, v)| (k.as_ref(), v.as_ref()))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    /// Inserts or updates a value, re-freezing the internal slice.
    pub fn insert(&mut self, key: &str, value: &str) {
        let mut map: BTreeMap<Box<str>, Box<str>> = self.pairs.iter().cloned().collect();
        map.insert(Box::from(key), Box::from(value));
        *self = Self::from_map(map);
    }

    /// Merges two configurations. Values from 'other' win on key collisions.
    pub fn merge(&self, other: &Cfg) -> Self {
        let mut map: BTreeMap<Box<str>, Box<str>> = self.pairs.iter().cloned().collect();

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

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
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

    pub fn get_value(&self, key: &str) -> &str {
        self.get(key).unwrap_or("")
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.pairs.iter().map(|(k, v)| (k.as_ref(), v.as_ref()))
    }
}

impl Index<&str> for Cfg {
    type Output = str;
    fn index(&self, key: &str) -> &Self::Output {
        self.get_value(key)
    }
}

impl fmt::Debug for Cfg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invariants() {
        // Test Latter Wins and Sorting
        let cfg = Cfg::new(&[("trace", "0"), ("mode", "strict"), ("trace", "1")]);
        assert_eq!(cfg.pairs.len(), 2);
        assert_eq!(&cfg["trace"], "1");
        assert_eq!(cfg.pairs[0].0.as_ref(), "mode"); // Alphabetical
    }

    #[test]
    fn test_binary_search() {
        let cfg = Cfg::new(&[("z", "last"), ("a", "first"), ("m", "middle")]);
        assert_eq!(cfg.get("a"), Some("first"));
        assert_eq!(cfg.get("m"), Some("middle"));
        assert_eq!(cfg.get("z"), Some("last"));
    }

    #[test]
    fn test_merge_logic() {
        let base = Cfg::new(&[("trace", "0"), ("memoize", "true")]);
        let over = Cfg::new(&[("trace", "1"), ("new_opt", "yes")]);
        let merged = base.merge(&over);

        assert!(merged.is_enabled("trace"));
        assert!(merged.is_enabled("memoize"));
        assert_eq!(&merged["new_opt"], "yes");
    }

    #[test]
    fn test_insert_mutation() {
        let mut cfg = Cfg::new(&[("a", "1")]);
        cfg.insert("b", "2");
        cfg.insert("a", "3");
        assert_eq!(&cfg["a"], "3");
        assert_eq!(&cfg["b"], "2");
        assert_eq!(cfg.pairs.len(), 2);
    }

    #[test]
    fn test_get_or() {
        let cfg = Cfg::new(&[("depth", "100")]);
        assert_eq!(cfg.get_or("depth", 0), 100);
        assert_eq!(cfg.get_or("missing", 50), 50);
    }

    #[test]
    fn test_is_falsy() {
        assert!(is_falsy("0"));
        assert!(is_falsy("false"));
        assert!(is_falsy("None"));
        assert!(!is_falsy("true"));
        assert!(!is_falsy("1"));
    }
}
