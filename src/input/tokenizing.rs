// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::constants::*;
use crate::cfg::*;
use crate::input::Error;
use crate::util::pyre::Pattern;
use crate::util::pyre::traits::Pattern as _;

/// Whitespace, comment, and EOL patterns used during tokenization.
#[derive(Clone, Debug)]
pub struct TokenizingPatterns {
    pub(super) wsp: Pattern,
    pub(super) cmt: Pattern,
    pub(super) eol: Pattern,
    // wsp is not the default
    pub(super) not_default: bool,
}

/// Default patterns: standard whitespace, no comments, no EOL comments.
impl Default for TokenizingPatterns {
    fn default() -> Self {
        Self::try_new(r"(?m)\s+", "(?!)", "(?!)").expect("default patterns must be valid")
    }
}

impl TokenizingPatterns {
    /// Configure patterns from a `Cfg` key set.
    pub fn configure(&mut self, cfg: &Cfg) {
        for opt in cfg.iter() {
            match opt {
                CfgKey::Wsp(p) => {
                    self.not_default = true;
                    if let Ok(new) = Self::compile(STR_WHITESPACE, p.as_str()) {
                        self.wsp = new;
                    }
                }
                CfgKey::Cmt(p) => {
                    self.not_default = true;
                    if let Ok(new) = Self::compile(STR_COMMENTS, p.as_str()) {
                        self.cmt = new;
                    }
                }
                CfgKey::Eol(p) => {
                    self.not_default = true;
                    if let Ok(new) = Self::compile(STR_EOL_COMMENTS, p.as_str()) {
                        self.eol = new;
                    }
                }
                _ => {}
            }
        }
    }

    /// Compile a pattern, validating it does not match empty.
    pub fn compile(kind: &'static str, pattern: &str) -> Result<Pattern, Error> {
        let p = Pattern::new(pattern).map_err(|source| Error::InvalidRegex {
            kind,
            pattern: pattern.to_string(),
            source,
        })?;
        Self::validate_no_empty_match(kind, p)
    }

    pub fn validate_no_empty_match(kind: &'static str, pattern: Pattern) -> Result<Pattern, Error> {
        if !pattern.pattern().is_empty() && pattern.matches_empty() {
            return Err(Error::RegexMatchesEmpty {
                kind,
                pattern: pattern.pattern().to_string(),
            });
        }
        Ok(pattern)
    }

    /// Try to create patterns from wsp, comment, and EOL strings.
    pub fn try_new(ws: &str, cm: &str, eo: &str) -> Result<Self, Error> {
        let wsp = Self::compile(STR_WHITESPACE, ws)?;
        let cmt = Self::compile(STR_COMMENTS, cm)?;
        let eol = Self::compile(STR_EOL_COMMENTS, eo)?;

        Ok(Self {
            wsp,
            cmt,
            eol,
            not_default: false,
        })
    }
}
