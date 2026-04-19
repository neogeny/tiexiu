// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::util::cfg;
use std::ops::Deref;

// NOTE! Order matters here! Debug < Mode < Trace
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub enum CfgK {
    #[default]
    None,

    Debug,
    Verbose,
    Trace,

    // Grammar directives
    Grammar(String),
    Wsp(String),
    Cmt(String),
    Eol(String),
    NameChars(String),

    IgnoreCase,
    NoNameGuard,
    NoLeftRecursion,
    NoParseInfo,
    NoMemoization,
}

#[derive(Clone, Default, Debug)]
pub struct Cfg(cfg::Cfg<CfgK>);

pub type CfgA<'c> = cfg::CfgA<'c, CfgK>;

/// Specialized trait for types that can be configured with the project-specific Cfg.
pub trait Configurable {
    fn configure(&mut self, _cfg: &Cfg) {}
}

impl cfg::CfgMapper<CfgK> for Cfg {
    fn map(key: &str, value: &str) -> Option<CfgK> {
        Cfg::map(key, value)
    }
}

impl Cfg {
    pub fn new(options: CfgA) -> Self {
        Self(cfg::Cfg::new(options))
    }

    pub fn map(key: &str, value: &str) -> Option<CfgK> {
        use super::constants::*;
        let is_truthy = !is_falsy(value);

        match (key.to_lowercase().as_str(), value) {
            ("trace", "1") => Some(CfgK::Trace),
            ("debug", "1") => Some(CfgK::Debug),
            ("verbose", "1") => Some(CfgK::Verbose),

            (STR_GRAMMAR_NAME, name) => Some(CfgK::Grammar(name.to_string())),
            (STR_WSP, pattern) => Some(CfgK::Wsp(pattern.to_string())),
            (STR_CMT, pattern) => Some(CfgK::Cmt(pattern.to_string())),
            (STR_EOL, pattern) => Some(CfgK::Eol(pattern.to_string())),

            ("ignorecase", _) if is_truthy => Some(CfgK::IgnoreCase),
            ("nameguard", _) if !is_truthy => Some(CfgK::NoNameGuard),
            ("left_recursion", _) if !is_truthy => Some(CfgK::NoLeftRecursion),
            ("parseinfo", _) if !is_truthy => Some(CfgK::NoParseInfo),
            ("memoization", _) if !is_truthy => Some(CfgK::NoMemoization),
            ("namechars", pattern) => Some(CfgK::NameChars(pattern.to_string())),

            _ => None,
        }
    }

    pub fn merge(&self, other: &Cfg) -> Self {
        Self(self.0.merge(&other.0))
    }

    pub fn load_from_env(prefix: &str) -> Self {
        Self(<Self as cfg::CfgMapper<CfgK>>::load_from_env(prefix))
    }
}

impl Deref for Cfg {
    type Target = cfg::Cfg<CfgK>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromIterator<CfgK> for Cfg {
    fn from_iter<I: IntoIterator<Item = CfgK>>(iter: I) -> Self {
        Self(cfg::Cfg::from_iter(iter))
    }
}

impl<'a, const N: usize> From<&'a [CfgK; N]> for Cfg {
    fn from(options: &'a [CfgK; N]) -> Self {
        Self::new(options.as_slice())
    }
}

impl From<Vec<CfgK>> for Cfg {
    fn from(options: Vec<CfgK>) -> Self {
        options.into_iter().collect()
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
    use std::env;

    #[test]
    fn test_cfg_concrete() {
        let options = [CfgK::Trace, CfgK::Debug];
        let cfg = Cfg::new(&options);

        // Tests Deref to access generic Cfg methods
        assert!(cfg.contains(&CfgK::Trace));
        assert!(cfg.contains(&CfgK::Debug));
        assert!(!cfg.contains(&CfgK::Verbose));
    }

    #[test]
    fn test_cfg_load_from_env() {
        unsafe {
            env::set_var("TIEXIU_TRACE", "1");
            env::set_var("TIEXIU_WHITESPACE", r"\s+");
            env::set_var("TIEXIU_PARSEINFO", "False");
        }

        let cfg = Cfg::load_from_env("TIEXIU_");

        assert!(cfg.contains(&CfgK::Trace));
        assert!(cfg.contains(&CfgK::Wsp(r"\s+".to_string())));
        assert!(cfg.contains(&CfgK::NoParseInfo));
    }

    #[test]
    fn test_bool_mapping() {
        assert_eq!(Cfg::map("ignorecase", "True"), Some(CfgK::IgnoreCase));
        assert_eq!(Cfg::map("parseinfo", "False"), Some(CfgK::NoParseInfo));
        assert_eq!(Cfg::map("parseinfo", "True"), None);
        assert_eq!(Cfg::map("left_recursion", "0"), Some(CfgK::NoLeftRecursion));
    }
}
