use std::ops::Index;
use std::str::FromStr;
use std::{env, fmt};

pub type CfgA<'c> = &'c [(&'c str, &'c str)];

/// Python-style falsy values for string-based configuration.
pub const FALSY_VALUES: &[&str] = &["false", "0", "no", "none", ""];

/// Helper to determine if a string is "falsy" in a Pythonic context.
pub fn is_falsy(v: &str) -> bool {
    v.is_empty() || FALSY_VALUES.contains(&v.to_lowercase().as_str())
}

/// An owned, optimized configuration container.
#[derive(Clone)]
pub struct Cfg {
    pairs: Box<[(Box<str>, Box<str>)]>,
}

impl Cfg {
    /// Creates a new Cfg from borrowed slices.
    pub fn new(pairs: CfgA) -> Self {
        let boxed_pairs = pairs
            .iter()
            .map(|(k, v)| (Box::from(*k), Box::from(*v)))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self { pairs: boxed_pairs }
    }

    pub fn fromenv(prefix: &str) -> Self {
        let pairs = env::vars()
            .filter(|(key, _)| key.starts_with(prefix))
            .map(|(key, val)| (key[prefix.len()..].to_string(), val))
            .map(|(mut key, val)| {
                if key.starts_with('_') {
                    key.remove(0);
                }
                (key, val)
            })
            .map(|(key, val)| (key.to_lowercase(), val))
            .map(|(k, v)| (k.into_boxed_str(), v.into_boxed_str()))
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self { pairs }
    }

    /// Merges two configurations, returning a new one.
    /// Values from 'other' win on key collisions.
    pub fn merge(&self, other: &Cfg) -> Self {
        let mut map: std::collections::BTreeMap<Box<str>, Box<str>> =
            self.pairs.iter().cloned().collect();

        for (k, v) in other.pairs.iter() {
            let s = map.get(k);
            if let Some(u) = s
                && !is_falsy(u)
            {
                continue;
            }
            map.insert(k.clone(), v.clone());
        }

        let merged_pairs = map.into_iter().collect::<Vec<_>>().into_boxed_slice();

        Self {
            pairs: merged_pairs,
        }
    }

    pub fn is_enabled(&self, key: &str) -> bool {
        self.pairs
            .iter()
            .find(|(k, _)| k.as_ref() == key)
            .map(|(_, v)| !is_falsy(v))
            .unwrap_or(false)
    }

    pub fn get_or<T: FromStr>(&self, key: &str, default: T) -> T {
        self.pairs
            .iter()
            .find(|(k, _)| k.as_ref() == key)
            .and_then(|(_, v)| v.parse::<T>().ok())
            .unwrap_or(default)
    }

    pub fn get_value(&self, key: &str) -> &str {
        self.pairs
            .iter()
            .find(|(k, _)| k.as_ref() == key)
            .map(|(_, v)| v.as_ref())
            .unwrap_or("")
    }
}

/// Fixed Index implementation.
/// Since Output is 'str', we return a reference '&str'.
impl Index<&str> for Cfg {
    type Output = str;
    fn index(&self, key: &str) -> &Self::Output {
        self.get_value(key)
    }
}

impl fmt::Debug for Cfg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_map = f.debug_map();
        for (k, v) in self.pairs.iter() {
            debug_map.entry(&k.as_ref(), &v.as_ref());
        }
        debug_map.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_conversion_and_storage() {
        let storage = [("trace", "1"), ("mode", "strict")];
        let cfg = Cfg::new(&storage);

        // Cfg is alive here
        assert!(cfg.is_enabled("trace"));
    }

    #[test]
    fn test_owned_cfg_logic() {
        let base = Cfg::new(&[("trace", "0"), ("memoize", "true")]);
        let overrides = Cfg::new(&[("trace", "1"), ("new_opt", "yes")]);

        let merged = base.merge(&overrides);

        assert!(merged.is_enabled("trace"));
        assert!(merged.is_enabled("memoize"));

        // This should now compile perfectly.
        // We compare the &str returned by index to the &str "1".
        assert_eq!(&merged["trace"], "1");
        assert_eq!(&merged["missing"], "");
    }

    #[test]
    fn test_cfg_fromenv_mangling() {
        let prefix = "TX";

        // Set up the environment for the test
        unsafe {
            env::set_var("TXVERBOSE", "true");
            env::set_var("TX_DEBUG", "1");
            env::set_var("TX_MAX_DEPTH", "50");
            env::set_var("OTHER_VAR", "ignore_me");
        }

        let cfg = Cfg::fromenv(prefix);

        // Helper to check if a key exists in our boxed slice
        let get_val = |key: &str| {
            cfg.pairs
                .iter()
                .find(|(k, _)| k.as_ref() == key)
                .map(|(_, v)| v.as_ref())
        };

        // Assertions for the four cases
        assert_eq!(get_val("verbose"), Some("true")); // Case 1: Simple prefix removal
        assert_eq!(get_val("debug"), Some("1")); // Case 2: Underscore peeling
        assert_eq!(get_val("max_depth"), Some("50")); // Case 3: Lowercasing
        assert!(get_val("other_var").is_none()); // Case 4: Filtering

        // Clean up (optional, but polite for other tests)
        unsafe {
            env::remove_var("TXVERBOSE");
            env::remove_var("TX_DEBUG");
            env::remove_var("TX_MAX_DEPTH");
        }
    }
}
