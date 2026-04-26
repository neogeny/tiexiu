// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::ENV_PREFIX;
pub use crate::util::cfg;
pub use cfg::*;

pub type CfgA = cfg::CfgA<CfgKey>;
pub type Cfg = cfg::Cfg<CfgKey>;

/// Configuration Key are addditive, so the default is empty
const DEFAULT_CFGA: &CfgA = &[];

/// Add configurations over default and env
pub fn config(cfga: &CfgA) -> Cfg {
    // NOTE:
    //  Configurations are meant to be mostly one-time
    //  except for options passed by library users through
    Cfg::from(DEFAULT_CFGA)
        .merge(&Cfg::load_from_env(ENV_PREFIX))
        .merge(&cfga.into())
}

pub trait CfgBoxWrapper {
    fn trace(&self) -> bool;
}

impl CfgBoxWrapper for Cfg {
    fn trace(&self) -> bool {
        self.contains(&CfgKey::Trace)
    }
}

// NOTE! Order matters here! Debug < Mode < Trace
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub enum CfgKey {
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

impl CfgMapper<CfgKey> for CfgKey {
    fn map(key: &str, value: &str) -> Option<CfgKey> {
        use super::constants::*;
        let is_truthy = !is_falsy(value);

        match (key.to_lowercase().as_str(), value) {
            ("trace", "1") => Some(CfgKey::Trace),
            ("debug", "1") => Some(CfgKey::Debug),
            ("verbose", "1") => Some(CfgKey::Verbose),

            (STR_GRAMMAR_NAME, name) => Some(CfgKey::Grammar(name.to_string())),
            (STR_WHITESPACE, pattern) => Some(CfgKey::Wsp(pattern.to_string())),
            (STR_COMMENTS, pattern) => Some(CfgKey::Cmt(pattern.to_string())),
            (STR_EOL_COMMENTS, pattern) => Some(CfgKey::Eol(pattern.to_string())),

            (STR_IGNORECASE, _) if is_truthy => Some(CfgKey::IgnoreCase),
            (STR_NAMEGUARD, _) if !is_truthy => Some(CfgKey::NoNameGuard),
            (STR_LEFTREC, _) if !is_truthy => Some(CfgKey::NoLeftRecursion),
            (STR_PARSEINFO, _) if !is_truthy => Some(CfgKey::NoParseInfo),
            (STR_MEMOIZATION, _) if !is_truthy => Some(CfgKey::NoMemoization),
            (STR_NAMECHARS, pattern) => Some(CfgKey::NameChars(pattern.to_string())),

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

            CfgKey::IgnoreCase => Some((STR_IGNORECASE, true_str)),
            CfgKey::NoNameGuard => Some((STR_NAMEGUARD, false_str)),
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
            env::set_var("TIEXIU_TRACE", "1");
            env::set_var("TIEXIU_WHITESPACE", r"\s+");
            env::set_var("TIEXIU_PARSEINFO", "False");
        }

        let cfg = CfgKey::load_from_env("TIEXIU_");

        assert!(cfg.contains(&CfgKey::Trace));
        assert!(cfg.contains(&CfgKey::Wsp(r"\s+".to_string())));
        assert!(cfg.contains(&CfgKey::NoParseInfo));
        Ok(())
    }

    #[test]
    fn test_bool_mapping() -> Result<()> {
        assert_eq!(CfgKey::map("ignorecase", "True"), Some(CfgKey::IgnoreCase));
        assert_eq!(CfgKey::map("parseinfo", "False"), Some(CfgKey::NoParseInfo));
        assert_eq!(CfgKey::map("parseinfo", "True"), None);
        assert_eq!(
            CfgKey::map("left_recursion", "0"),
            Some(CfgKey::NoLeftRecursion)
        );
        Ok(())
    }
}
