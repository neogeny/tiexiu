// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::constants::*;
use crate::cfg::*;
use crate::input::Error;
use crate::util::pyre::Pattern;
use crate::util::pyre::traits::Pattern as _;

#[derive(Clone, Debug)]
pub struct TokenizingPatterns {
    pub wsp: Pattern,
    pub cmt: Pattern,
    pub eol: Pattern,
    pub has_wsp: bool,
    pub has_cmt: bool,
    pub has_eol: bool,
}

impl Default for TokenizingPatterns {
    fn default() -> Self {
        Self::try_new("", "", "").expect("empty patterns must compile to never-match")
    }
}

impl TokenizingPatterns {
    pub fn configure(&mut self, cfg: &Cfg) {
        for opt in cfg.iter() {
            match opt {
                CfgKey::Wsp(p) => {
                    if p.is_empty() {
                        self.has_wsp = false;
                    }
                    if let Ok(new) = Self::compile(STR_WHITESPACE, p.as_str()) {
                        self.wsp = new;
                    }
                }
                CfgKey::Cmt(p) => {
                    if p.is_empty() {
                        self.has_cmt = false;
                    }
                    if let Ok(new) = Self::compile(STR_COMMENTS, p.as_str()) {
                        self.cmt = new;
                    }
                }
                CfgKey::Eol(p) => {
                    if p.is_empty() {
                        self.has_eol = false;
                    }
                    if let Ok(new) = Self::compile(STR_EOL_COMMENTS, p.as_str()) {
                        self.eol = new;
                    }
                }
                _ => {}
            }
        }
    }

    pub fn compile(kind: &'static str, mut pattern: &str) -> Result<Pattern, Error> {
        if pattern.is_empty() {
            pattern = "(?!)";
        }
        let p = Pattern::new(pattern).map_err(|source| Error::InvalidRegex {
            kind,
            pattern: pattern.to_string(),
            source,
        })?;
        Self::validate_no_empty_match(&p, kind);
        Ok(p)
    }

    pub fn from_cfg(cfg: &Cfg) -> Result<Self, Error> {
        let mut wsp = "";
        let mut cmt = "";
        let mut eol = "";

        for opt in cfg.iter() {
            match opt {
                CfgKey::Wsp(p) => wsp = p.as_str(),
                CfgKey::Cmt(p) => cmt = p.as_str(),
                CfgKey::Eol(p) => eol = p.as_str(),
                _ => {}
            }
        }

        TokenizingPatterns::try_new(wsp, cmt, eol)
    }

    pub fn validate_no_empty_match(pattern: &Pattern, kind: &str) {
        assert!(
            !pattern.matches_empty(),
            "pattern '{}' for {} matches empty string, which would cause infinite loop",
            pattern.pattern(),
            kind
        );
    }

    pub fn try_new(ws: &str, cm: &str, eo: &str) -> Result<Self, Error> {
        let wsp = Self::compile(STR_WHITESPACE, ws)?;
        let cmt = Self::compile(STR_COMMENTS, cm)?;
        let eol = Self::compile(STR_EOL_COMMENTS, eo)?;

        let mut parts = Vec::new();
        if !wsp.is_empty() {
            parts.push(format!("(?:{})", wsp.pattern()));
        }
        if !cmt.is_empty() {
            parts.push(format!("(?:{})", cmt.pattern()));
        }
        if !eol.is_empty() {
            parts.push(format!("(?:{})", eol.pattern()));
        }
        Ok(Self {
            wsp,
            cmt,
            eol,
            has_wsp: !ws.is_empty(),
            has_cmt: !cm.is_empty(),
            has_eol: !eo.is_empty(),
        })
    }
}
