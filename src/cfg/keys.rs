// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::ENV_PREFIX;
pub use crate::util::cfg;
pub use cfg::*;

pub type CfgA = cfg::CfgA<Key>;
pub type Cfg = cfg::Cfg<Key>;

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
        self.contains(&Key::Trace)
    }
}

// NOTE! Order matters here! Debug < Mode < Trace
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub enum Key {
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

impl CfgMapper<Key> for Key {
    fn map(key: &str, value: &str) -> Option<Key> {
        use super::constants::*;
        let is_truthy = !is_falsy(value);

        match (key.to_lowercase().as_str(), value) {
            ("trace", "1") => Some(Key::Trace),
            ("debug", "1") => Some(Key::Debug),
            ("verbose", "1") => Some(Key::Verbose),

            (STR_GRAMMAR_NAME, name) => Some(Key::Grammar(name.to_string())),
            (STR_WHITESPACE, pattern) => Some(Key::Wsp(pattern.to_string())),
            (STR_COMMENTS, pattern) => Some(Key::Cmt(pattern.to_string())),
            (STR_EOL_COMMENTS, pattern) => Some(Key::Eol(pattern.to_string())),

            (STR_IGNORECASE, _) if is_truthy => Some(Key::IgnoreCase),
            (STR_NAMEGUARD, _) if !is_truthy => Some(Key::NoNameGuard),
            (STR_LEFTREC, _) if !is_truthy => Some(Key::NoLeftRecursion),
            (STR_PARSEINFO, _) if !is_truthy => Some(Key::NoParseInfo),
            (STR_MEMOIZATION, _) if !is_truthy => Some(Key::NoMemoization),
            (STR_NAMECHARS, pattern) => Some(Key::NameChars(pattern.to_string())),

            _ => None,
        }
    }

    fn unmap(value: &Key) -> Option<(&str, &str)> {
        use super::constants::*;
        let true_str = "True";
        let false_str = "False";
        match value {
            Key::Grammar(v) => Some((STR_GRAMMAR_NAME, v.as_str())),
            Key::Wsp(v) => Some((STR_WHITESPACE, v.as_str())),
            Key::Cmt(v) => Some((STR_COMMENTS, v.as_str())),
            Key::Eol(v) => Some((STR_EOL_COMMENTS, v.as_str())),
            Key::NameChars(v) => Some((STR_NAMECHARS, v.as_str())),

            Key::IgnoreCase => Some((STR_IGNORECASE, true_str)),
            Key::NoNameGuard => Some((STR_NAMEGUARD, false_str)),
            Key::NoLeftRecursion => Some((STR_LEFTREC, false_str)),
            Key::NoParseInfo => Some((STR_PARSEINFO, false_str)),
            Key::NoMemoization => Some((STR_MEMOIZATION, false_str)),
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
        let options = [Key::Trace, Key::Debug];
        let cfg = Cfg::new(&options);

        assert!(cfg.contains(&Key::Trace));
        assert!(cfg.contains(&Key::Debug));
        Ok(())
    }

    #[test]
    fn test_cfg_load_from_env() -> Result<()> {
        unsafe {
            env::set_var("TIEXIU_TRACE", "1");
            env::set_var("TIEXIU_WHITESPACE", r"\s+");
            env::set_var("TIEXIU_PARSEINFO", "False");
        }

        let cfg = Key::load_from_env("TIEXIU_");

        assert!(cfg.contains(&Key::Trace));
        assert!(cfg.contains(&Key::Wsp(r"\s+".to_string())));
        assert!(cfg.contains(&Key::NoParseInfo));
        Ok(())
    }

    #[test]
    fn test_bool_mapping() -> Result<()> {
        assert_eq!(Key::map("ignorecase", "True"), Some(Key::IgnoreCase));
        assert_eq!(Key::map("parseinfo", "False"), Some(Key::NoParseInfo));
        assert_eq!(Key::map("parseinfo", "True"), None);
        assert_eq!(Key::map("left_recursion", "0"), Some(Key::NoLeftRecursion));
        Ok(())
    }
}
