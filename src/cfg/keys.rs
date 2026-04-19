// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::util::cfg;
use std::ops::Deref;

// NOTE! Order matters here! Debug < Mode < Trace
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub enum Cfg {
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
pub struct CfgBox {
    cfg: cfg::CfgBox<Cfg>,
}

pub type CfgA = cfg::CfgA<Cfg>;

/// Specialized trait for types that can be configured with the project-specific Cfg.
pub trait Configurable {
    fn configure(&mut self, _cfg: &CfgBox) {}
}

impl cfg::CfgMapper<Cfg> for CfgBox {
    fn map(key: &str, value: &str) -> Option<Cfg> {
        CfgBox::map(key, value)
    }
}

impl From<&CfgA> for CfgBox {
    fn from(cfg: &CfgA) -> Self {
        Self::new(cfg)
    }
}

impl CfgBox {
    pub fn new(options: &CfgA) -> Self {
        Self {
            cfg: cfg::CfgBox::new(options),
        }
    }

    pub fn map(key: &str, value: &str) -> Option<Cfg> {
        use super::constants::*;
        let is_truthy = !is_falsy(value);

        match (key.to_lowercase().as_str(), value) {
            ("trace", "1") => Some(Cfg::Trace),
            ("debug", "1") => Some(Cfg::Debug),
            ("verbose", "1") => Some(Cfg::Verbose),

            (STR_GRAMMAR_NAME, name) => Some(Cfg::Grammar(name.to_string())),
            (STR_WSP, pattern) => Some(Cfg::Wsp(pattern.to_string())),
            (STR_CMT, pattern) => Some(Cfg::Cmt(pattern.to_string())),
            (STR_EOL, pattern) => Some(Cfg::Eol(pattern.to_string())),

            ("ignorecase", _) if is_truthy => Some(Cfg::IgnoreCase),
            ("nameguard", _) if !is_truthy => Some(Cfg::NoNameGuard),
            ("left_recursion", _) if !is_truthy => Some(Cfg::NoLeftRecursion),
            ("parseinfo", _) if !is_truthy => Some(Cfg::NoParseInfo),
            ("memoization", _) if !is_truthy => Some(Cfg::NoMemoization),
            ("namechars", pattern) => Some(Cfg::NameChars(pattern.to_string())),

            _ => None,
        }
    }

    pub fn merge(&self, other: &CfgBox) -> Self {
        Self {
            cfg: self.cfg.merge(&other.cfg),
        }
    }

    pub fn load_from_env(prefix: &str) -> Self {
        Self {
            cfg: <Self as cfg::CfgMapper<Cfg>>::load_from_env(prefix),
        }
    }
}

impl Deref for CfgBox {
    type Target = cfg::CfgBox<Cfg>;
    fn deref(&self) -> &Self::Target {
        &self.cfg
    }
}

impl FromIterator<Cfg> for CfgBox {
    fn from_iter<I: IntoIterator<Item = Cfg>>(iter: I) -> Self {
        Self {
            cfg: cfg::CfgBox::from_iter(iter),
        }
    }
}

impl<'a, const N: usize> From<&'a [Cfg; N]> for CfgBox {
    fn from(options: &'a [Cfg; N]) -> Self {
        Self::new(options.as_slice())
    }
}

impl From<Vec<Cfg>> for CfgBox {
    fn from(options: Vec<Cfg>) -> Self {
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
        let options = [Cfg::Trace, Cfg::Debug];
        let cfg = CfgBox::new(&options);

        // Tests Deref to access generic Cfg methods
        assert!(cfg.contains(&Cfg::Trace));
        assert!(cfg.contains(&Cfg::Debug));
        assert!(!cfg.contains(&Cfg::Verbose));
    }

    #[test]
    fn test_cfg_load_from_env() {
        unsafe {
            env::set_var("TIEXIU_TRACE", "1");
            env::set_var("TIEXIU_WHITESPACE", r"\s+");
            env::set_var("TIEXIU_PARSEINFO", "False");
        }

        let cfg = CfgBox::load_from_env("TIEXIU_");

        assert!(cfg.contains(&Cfg::Trace));
        assert!(cfg.contains(&Cfg::Wsp(r"\s+".to_string())));
        assert!(cfg.contains(&Cfg::NoParseInfo));
    }

    #[test]
    fn test_bool_mapping() {
        assert_eq!(CfgBox::map("ignorecase", "True"), Some(Cfg::IgnoreCase));
        assert_eq!(CfgBox::map("parseinfo", "False"), Some(Cfg::NoParseInfo));
        assert_eq!(CfgBox::map("parseinfo", "True"), None);
        assert_eq!(
            CfgBox::map("left_recursion", "0"),
            Some(Cfg::NoLeftRecursion)
        );
    }
}
