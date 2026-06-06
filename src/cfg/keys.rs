// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::heartbeat::HeartbeatRef;
use std::sync::Arc;

use super::ENV_PREFIX;
pub use crate::util::cfg;
pub use cfg::*;

/// Type alias for a configuration array.
pub type CfgA = cfg::CfgA<CfgKey>;
/// Type alias for a concrete configuration box.
pub type Cfg = cfg::Cfg<CfgKey>;

/// Configuration Key are addditive, so the default is empty
const DEFAULT_CFGA: &CfgA = &[];

/// Configuration key for the TieXiu parser engine.
#[derive(Debug, Clone, Default)]
pub enum CfgKey {
    /// No-op placeholder.
    #[default]
    Null,

    /// Enable debug output.
    Debug,
    /// Enable verbose output.
    Verbose,
    /// Enable trace output.
    Trace,

    // Grammar directives
    /// Name of the grammar.
    Grammar(String),
    /// Whitespace pattern.
    Wsp(String),
    /// Comment pattern.
    Cmt(String),
    /// End-of-line comment pattern.
    Eol(String),
    /// Allowed name characters.
    NameChars(String),
    /// Start rule name.
    Start(String),

    /// Enable case-insensitive matching.
    IgnoreCase,
    /// Disable case-insensitive matching.
    NoIgnoreCase,
    /// Enable or disable name guard (bool).
    NameGuard(bool),
    /// Disable left-recursion support.
    NoLeftRecursion,
    /// Disable parse info tracking.
    NoParseInfo,
    /// Disable memoization.
    NoMemoization,

    // Cursor
    /// The source of the input
    Source(String),

    /// Heartbeat callback for progress reporting
    Heartbeat(HeartbeatRef),
}

/// Build a `Cfg` by merging defaults, environment, and the given overrides.
pub fn config(cfga: &CfgA) -> Cfg {
    // NOTE:
    //  Configurations are meant to be mostly one-time
    //  except for options passed by library users through
    Cfg::from(DEFAULT_CFGA)
        .merge(&Cfg::load_from_env(ENV_PREFIX))
        .merge(&cfga.into())
}

pub(crate) trait CfgBoxWrapper {
    #[allow(dead_code)]
    fn trace(&self) -> bool;
    fn heartbeat(&self) -> Option<&HeartbeatRef>;
    fn start(&self) -> Option<&str>;
}

impl CfgBoxWrapper for Cfg {
    fn trace(&self) -> bool {
        self.contains(&CfgKey::Trace)
    }

    fn heartbeat(&self) -> Option<&HeartbeatRef> {
        self.iter().find_map(|k| match k {
            CfgKey::Heartbeat(h) => Some(h),
            _ => None,
        })
    }

    fn start(&self) -> Option<&str> {
        self.iter().find_map(|k| match k {
            CfgKey::Start(s) => Some(s.as_str()),
            _ => None,
        })
    }
}

impl PartialEq for CfgKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Debug, Self::Debug) => true,
            (Self::Verbose, Self::Verbose) => true,
            (Self::Trace, Self::Trace) => true,
            (Self::Grammar(a), Self::Grammar(b)) => a == b,
            (Self::Wsp(a), Self::Wsp(b)) => a == b,
            (Self::Cmt(a), Self::Cmt(b)) => a == b,
            (Self::Eol(a), Self::Eol(b)) => a == b,
            (Self::NameChars(a), Self::NameChars(b)) => a == b,
            (Self::Start(a), Self::Start(b)) => a == b,
            (Self::IgnoreCase, Self::IgnoreCase) => true,
            (Self::NoIgnoreCase, Self::NoIgnoreCase) => true,
            (Self::NameGuard(a), Self::NameGuard(b)) => a == b,
            (Self::NoLeftRecursion, Self::NoLeftRecursion) => true,
            (Self::NoParseInfo, Self::NoParseInfo) => true,
            (Self::NoMemoization, Self::NoMemoization) => true,
            (Self::Source(a), Self::Source(b)) => a == b,
            (Self::Heartbeat(a), Self::Heartbeat(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Eq for CfgKey {}

impl std::hash::Hash for CfgKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            CfgKey::Grammar(s)
            | CfgKey::Wsp(s)
            | CfgKey::Cmt(s)
            | CfgKey::Eol(s)
            | CfgKey::NameChars(s)
            | CfgKey::Start(s)
            | CfgKey::Source(s) => {
                s.hash(state);
            }
            CfgKey::Heartbeat(h) => {
                std::ptr::hash(Arc::as_ptr(h), state);
            }
            _ => {}
        }
    }
}

unsafe impl Send for CfgKey {}
unsafe impl Sync for CfgKey {}

/// Specialized trait for types that can be configured with the project-specific CfgBox.
pub trait Configurable {
    fn configure(&mut self, cfg: &Cfg) {
        let _ = cfg;
    }
}

impl From<&CfgA> for Cfg {
    fn from(cfga: &CfgA) -> Self {
        Cfg::new(cfga)
    }
}

impl Cfg {
    /// Merges configurations where `other` overrides `self` by variant type.
    /// For entries with the same variant (e.g. Eol("a") vs Eol("b")),
    /// the value from `other` is kept.
    /// Preserves insertion order: self's items first, then other's new items,
    /// with other's updates to same-variant keys replacing self's in place.
    pub fn override_merge(&self, other: &Cfg) -> Self {
        let mut result = Vec::with_capacity(self.len() + other.len());
        // Build a lookup of which variant ordinals other will override
        let mut override_ords = std::collections::HashSet::new();
        for k in other.iter() {
            override_ords.insert(Self::variant_ord(k));
        }
        // Take self's items that won't be overridden
        for k in self.iter() {
            let v = Self::variant_ord(k);
            if !override_ords.contains(&v) {
                result.push(k.clone());
            }
        }
        // Add all of other's items (preserving insertion order)
        for k in other.iter() {
            result.push(k.clone());
        }
        Self::from_boxed_slice(result.into_boxed_slice())
    }

    fn variant_ord(k: &CfgKey) -> u8 {
        match k {
            CfgKey::Null => 0,
            CfgKey::Debug => 1,
            CfgKey::Verbose => 2,
            CfgKey::Trace => 3,
            CfgKey::Grammar(_) => 4,
            CfgKey::Wsp(_) => 5,
            CfgKey::Cmt(_) => 6,
            CfgKey::Eol(_) => 7,
            CfgKey::NameChars(_) => 8,
            CfgKey::Start(_) => 8,
            CfgKey::IgnoreCase => 9,
            CfgKey::NoIgnoreCase => 10,
            CfgKey::NameGuard(_) => 11,
            CfgKey::NoLeftRecursion => 12,
            CfgKey::NoParseInfo => 13,
            CfgKey::NoMemoization => 14,
            CfgKey::Source(_) => 15,
            CfgKey::Heartbeat(_) => 16,
        }
    }
}

impl CfgMapper<CfgKey> for CfgKey {
    fn map(key: &str, value: &str) -> Option<CfgKey> {
        use super::constants::*;
        let is_truthy = !is_falsy(value);

        match (key.to_lowercase().as_str(), value) {
            (STR_TRACE, _) => {
                if is_truthy {
                    Some(CfgKey::Trace)
                } else {
                    Some(CfgKey::Null)
                }
            }
            (STR_DEBUG, _) => {
                if is_truthy {
                    Some(CfgKey::Debug)
                } else {
                    Some(CfgKey::Null)
                }
            }
            (STR_VERBOSE, _) => {
                if is_truthy {
                    Some(CfgKey::Verbose)
                } else {
                    Some(CfgKey::Null)
                }
            }
            (STR_GRAMMAR_NAME, name) => Some(CfgKey::Grammar(name.to_string())),
            (STR_WHITESPACE, pattern) => Some(CfgKey::Wsp(pattern.to_string())),
            (STR_COMMENTS, pattern) => Some(CfgKey::Cmt(pattern.to_string())),
            (STR_EOL_COMMENTS, pattern) => Some(CfgKey::Eol(pattern.to_string())),

            (STR_IGNORECASE, _) => {
                if is_truthy {
                    Some(CfgKey::IgnoreCase)
                } else {
                    Some(CfgKey::NoIgnoreCase)
                }
            }
            (STR_NAMEGUARD, _) => Some(CfgKey::NameGuard(is_truthy)),
            (STR_LEFTREC, _) => {
                if !is_truthy {
                    Some(CfgKey::NoLeftRecursion)
                } else {
                    Some(CfgKey::Null)
                }
            }
            (STR_PARSEINFO, _) => {
                if !is_truthy {
                    Some(CfgKey::NoParseInfo)
                } else {
                    Some(CfgKey::Null)
                }
            }
            (STR_MEMOIZATION, _) => {
                if !is_truthy {
                    Some(CfgKey::NoMemoization)
                } else {
                    Some(CfgKey::Null)
                }
            }
            (STR_NAMECHARS, pattern) => Some(CfgKey::NameChars(pattern.to_string())),
            (STR_START, name) => Some(CfgKey::Start(name.to_string())),
            (STR_SOURCE, name) => Some(CfgKey::Source(name.to_string())),
            (STR_FILENAME, name) => Some(CfgKey::Source(name.to_string())),

            _ => None,
        }
    }

    fn unmap(value: &CfgKey) -> Option<(&str, &str)> {
        use super::constants::*;
        let true_str = "True";
        let false_str = "False";
        match value {
            CfgKey::Grammar(v) => Some((STR_GRAMMAR_NAME, v.as_str())),
            CfgKey::Wsp(v) => Some((STR_WHITESPACE, v.as_str())),
            CfgKey::Cmt(v) => Some((STR_COMMENTS, v.as_str())),
            CfgKey::Eol(v) => Some((STR_EOL_COMMENTS, v.as_str())),
            CfgKey::NameChars(v) => Some((STR_NAMECHARS, v.as_str())),
            CfgKey::Start(v) => Some((STR_START, v.as_str())),

            CfgKey::IgnoreCase => Some((STR_IGNORECASE, true_str)),
            CfgKey::NoIgnoreCase => Some((STR_IGNORECASE, false_str)),
            CfgKey::NameGuard(true) => Some((STR_NAMEGUARD, true_str)),
            CfgKey::NameGuard(false) => Some((STR_NAMEGUARD, false_str)),
            CfgKey::NoLeftRecursion => Some((STR_LEFTREC, false_str)),
            CfgKey::NoParseInfo => Some((STR_PARSEINFO, false_str)),
            CfgKey::NoMemoization => Some((STR_MEMOIZATION, false_str)),
            _ => None,
        }
    }
}

/// Helper to determine if a string is "falsy" in a Pythonic context.
fn is_falsy(v: &str) -> bool {
    const FALSY_VALUES: &[&str] = &["false", "0", "no", "none", "False", "No"];
    v.is_empty() || FALSY_VALUES.contains(&v.to_lowercase().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;
    use std::env;

    #[test]
    fn test_cfg_box_is_alias() -> Result<()> {
        let options = [CfgKey::Trace, CfgKey::Debug];
        let cfg = Cfg::new(&options);

        assert!(cfg.contains(&CfgKey::Trace));
        assert!(cfg.contains(&CfgKey::Debug));
        Ok(())
    }

    #[test]
    fn test_cfg_load_from_env() -> Result<()> {
        unsafe {
            env::set_var("CFG_TEST_TRACE", "1");
            env::set_var("CFG_TEST_WHITESPACE", r"\s+");
            env::set_var("CFG_TEST_PARSEINFO", "False");
        }

        let cfg = CfgKey::load_from_env("CFG_TEST_");

        unsafe {
            env::remove_var("CFG_TEST_TRACE");
            env::remove_var("CFG_TEST_WHITESPACE");
            env::remove_var("CFG_TEST_PARSEINFO");
        }

        assert!(cfg.contains(&CfgKey::Trace));
        assert!(cfg.contains(&CfgKey::Wsp(r"\s+".to_string())));
        assert!(cfg.contains(&CfgKey::NoParseInfo));
        Ok(())
    }

    #[test]
    fn test_bool_mapping() -> Result<()> {
        assert_eq!(CfgKey::map("ignorecase", "True"), Some(CfgKey::IgnoreCase));
        assert_eq!(CfgKey::map("parseinfo", "False"), Some(CfgKey::NoParseInfo));
        assert_eq!(CfgKey::map("parseinfo", "True"), Some(CfgKey::Null));
        assert_eq!(
            CfgKey::map("left_recursion", "0"),
            Some(CfgKey::NoLeftRecursion)
        );
        Ok(())
    }
}
