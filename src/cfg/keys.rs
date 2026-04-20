// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub use crate::util::cfg;
pub use cfg::*;

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

pub type CfgA = cfg::CfgA<Cfg>;
pub type CfgBox = cfg::CfgBox<Cfg>;

/// Specialized trait for types that can be configured with the project-specific CfgBox.
pub trait Configurable {
    fn configure(&mut self, cfg: &CfgBox) {
        let _ = cfg;
    }
}

impl CfgMapper<Cfg> for Cfg {
    fn map(key: &str, value: &str) -> Option<Cfg> {
        use super::constants::*;
        let is_truthy = !is_falsy(value);

        match (key.to_lowercase().as_str(), value) {
            ("trace", "1") => Some(Cfg::Trace),
            ("debug", "1") => Some(Cfg::Debug),
            ("verbose", "1") => Some(Cfg::Verbose),

            (STR_GRAMMAR_NAME, name) => Some(Cfg::Grammar(name.to_string())),
            (STR_WHITESPACE, pattern) => Some(Cfg::Wsp(pattern.to_string())),
            (STR_COMMENTS, pattern) => Some(Cfg::Cmt(pattern.to_string())),
            (STR_EOL_COMMENTS, pattern) => Some(Cfg::Eol(pattern.to_string())),

            (STR_IGNORECASE, _) if is_truthy => Some(Cfg::IgnoreCase),
            (STR_NAMEGUARD, _) if !is_truthy => Some(Cfg::NoNameGuard),
            (STR_LEFTREC, _) if !is_truthy => Some(Cfg::NoLeftRecursion),
            (STR_PARSEINFO, _) if !is_truthy => Some(Cfg::NoParseInfo),
            (STR_MEMOIZATION, _) if !is_truthy => Some(Cfg::NoMemoization),
            (STR_NAMECHARS, pattern) => Some(Cfg::NameChars(pattern.to_string())),

            _ => None,
        }
    }

    fn unmap(value: &Cfg) -> Option<(&str, &str)> {
        use super::constants::*;
        let true_str = "True";
        let false_str = "False";
        match value {
            Cfg::Grammar(v) => Some((STR_GRAMMAR_NAME, v.as_str())),
            Cfg::Wsp(v) => Some((STR_WHITESPACE, v.as_str())),
            Cfg::Cmt(v) => Some((STR_COMMENTS, v.as_str())),
            Cfg::Eol(v) => Some((STR_EOL_COMMENTS, v.as_str())),
            Cfg::NameChars(v) => Some((STR_NAMECHARS, v.as_str())),

            Cfg::IgnoreCase => Some((STR_IGNORECASE, true_str)),
            Cfg::NoNameGuard => Some((STR_NAMEGUARD, false_str)),
            Cfg::NoLeftRecursion => Some((STR_LEFTREC, false_str)),
            Cfg::NoParseInfo => Some((STR_PARSEINFO, false_str)),
            Cfg::NoMemoization => Some((STR_MEMOIZATION, false_str)),
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
    use std::env;

    #[test]
    fn test_cfg_box_is_alias() {
        let options = [Cfg::Trace, Cfg::Debug];
        let cfg = CfgBox::new(&options);

        assert!(cfg.contains(&Cfg::Trace));
        assert!(cfg.contains(&Cfg::Debug));
    }

    #[test]
    fn test_cfg_load_from_env() {
        unsafe {
            env::set_var("TIEXIU_TRACE", "1");
            env::set_var("TIEXIU_WHITESPACE", r"\s+");
            env::set_var("TIEXIU_PARSEINFO", "False");
        }

        let cfg = Cfg::load_from_env("TIEXIU_");

        assert!(cfg.contains(&Cfg::Trace));
        assert!(cfg.contains(&Cfg::Wsp(r"\s+".to_string())));
        assert!(cfg.contains(&Cfg::NoParseInfo));
    }

    #[test]
    fn test_bool_mapping() {
        assert_eq!(Cfg::map("ignorecase", "True"), Some(Cfg::IgnoreCase));
        assert_eq!(Cfg::map("parseinfo", "False"), Some(Cfg::NoParseInfo));
        assert_eq!(Cfg::map("parseinfo", "True"), None);
        assert_eq!(Cfg::map("left_recursion", "0"), Some(Cfg::NoLeftRecursion));
    }
}
