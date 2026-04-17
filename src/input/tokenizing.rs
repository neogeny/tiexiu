// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::input::Error;
use crate::util::pyre::Pattern;

#[derive(Clone, Debug)]
pub struct TokenizingPatterns {
    pub wsp: Pattern,
    pub cmt: Pattern,
    pub eol: Pattern,
}

impl TokenizingPatterns {
    const DEFAULT_WSP: &'static str = r"\s+";
    const DEFAULT_EOL: &'static str = r"//.*$";
    const DEFAULT_CMT: &'static str = r"/\*\*/";

    fn compile(kind: &'static str, pattern: &str) -> Result<Pattern, Error> {
        let p = Pattern::new(pattern).map_err(|source| Error::InvalidRegex {
            kind,
            pattern: pattern.to_string(),
            source,
        })?;
        Self::validate_no_empty_match(&p, kind);
        Ok(p)
    }

    fn validate_no_empty_match(pattern: &Pattern, kind: &str) {
        assert!(
            pattern.search("").is_none(),
            "pattern '{}' for {} matches empty string, which would cause infinite loop",
            pattern.pattern(),
            kind
        );
    }

    pub fn try_new(ws: &str, cmt: &str, eol: &str) -> Result<Self, Error> {
        Ok(Self {
            wsp: Self::compile("whitespace", ws)?,
            cmt: Self::compile("comment", cmt)?,
            eol: Self::compile("end-of-line", eol)?,
        })
    }
}

impl Default for TokenizingPatterns {
    fn default() -> Self {
        Self::try_new(Self::DEFAULT_WSP, Self::DEFAULT_CMT, Self::DEFAULT_EOL)
            .expect("default StrCursor regex patterns must be valid")
    }
}
