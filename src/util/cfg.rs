use std::fmt;
use std::ops::Index;
use std::str::FromStr;

/// Python-style falsy values for string-based configuration.
pub const FALSY_VALUES: &[&str] = &["false", "0", "no", "none", ""];

pub type CfgA<'c> = &'c [(&'c str, &'c str)];

/// A lightweight, read-only configuration view (Transient).
/// Use this for per-call options where the caller owns the data.
#[derive(Copy, Clone)]
pub struct Cfg<'a> {
    pairs: CfgA<'a>,
}

impl<'a> Cfg<'a> {
    pub fn new(pairs: &'a [(&'a str, &'a str)]) -> Self {
        Self { pairs }
    }

    /// Consumes the Cfg view and promotes the data to a strictly optimized,
    /// heap-allocated collection. This 'kills' the transient view by transferring
    /// ownership to the new boxed slice.
    pub fn into_boxed_slice(self) -> Box<[(Box<str>, Box<str>)]> {
        self.pairs
            .iter()
            .map(|(k, v)| (Box::from(*k), Box::from(*v)))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    /// Python-style boolean helper (falsy check).
    pub fn is_enabled(&self, key: &str) -> bool {
        self.pairs
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| !v.is_empty() && !FALSY_VALUES.contains(&v.to_lowercase().as_str()))
            .unwrap_or(false)
    }

    /// Generic retrieval with type-parsing and default fallback.
    pub fn get_or<T: FromStr>(&self, key: &str, default: T) -> T {
        self.pairs
            .iter()
            .find(|(k, _)| *k == key)
            .and_then(|(_, v)| v.parse::<T>().ok())
            .unwrap_or(default)
    }
}

/// Helper extension to allow the stored Boxed version to behave like the Cfg view.
pub trait CfgStorage {
    fn is_enabled(&self, key: &str) -> bool;
    fn get_value(&self, key: &str) -> &str;
}

impl CfgStorage for Box<[(Box<str>, Box<str>)]> {
    fn is_enabled(&self, key: &str) -> bool {
        self.iter()
            .find(|(k, _)| k.as_ref() == key)
            .map(|(_, v)| !v.is_empty() && !FALSY_VALUES.contains(&v.to_lowercase().as_str()))
            .unwrap_or(false)
    }

    fn get_value(&self, key: &str) -> &str {
        self.iter()
            .find(|(k, _)| k.as_ref() == key)
            .map(|(_, v)| v.as_ref())
            .unwrap_or("")
    }
}

impl<'a> Index<&str> for Cfg<'a> {
    type Output = str;
    fn index(&self, key: &str) -> &Self::Output {
        self.pairs
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
            .unwrap_or("")
    }
}

impl<'a> fmt::Debug for Cfg<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(self.pairs.iter().map(|&(k, v)| (k, v)))
            .finish()
    }
}

impl<'a> From<Cfg<'a>> for Box<[(Box<str>, Box<str>)]> {
    fn from(cfg: Cfg<'a>) -> Self {
        cfg.into_boxed_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_and_storage() {
        let storage = [("trace", "1"), ("mode", "strict")];
        let cfg = Cfg::new(&storage);

        // Cfg is alive here
        assert!(cfg.is_enabled("trace"));

        // Cfg is consumed here
        let boxed = cfg.into_boxed_slice();

        // Using the CfgStorage trait on the boxed version
        assert!(boxed.is_enabled("trace"));
        assert_eq!(boxed.get_value("mode"), "strict");

        // cfg.is_enabled("trace"); // <-- This would now fail to compile!
    }
}
