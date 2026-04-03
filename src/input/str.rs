use super::Cursor;
#[cfg(feature = "regex")]
use regex::Regex;
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait Patterns: Clone + Debug + Default {
    const WHITESPACE: &'static str = r"\s+";
    const EOL_COMMENTS: &'static str = r"//.*$";
    const COMMENTS: &'static str = r"";

    fn whitespace_re(&self) -> &Regex;
    fn comments_re(&self) -> &Regex;
    fn eol_comments_re(&self) -> &Regex;
}

#[derive(Clone, Debug)]
pub struct DefaultPatterns {
    _whitespace_re: Regex,
    _comments_re: Regex,
    _eol_comments_re: Regex,
}

impl Default for DefaultPatterns {
    fn default() -> Self {
        Self {
            _whitespace_re: Regex::new(DefaultPatterns::WHITESPACE).unwrap(),
            _comments_re: Regex::new(DefaultPatterns::COMMENTS).unwrap(),
            _eol_comments_re: Regex::new(DefaultPatterns::EOL_COMMENTS).unwrap(),
        }
    }
}

impl Patterns for DefaultPatterns {
    fn whitespace_re(&self) -> &Regex {
        &self._whitespace_re
    }

    fn comments_re(&self) -> &Regex {
        &self._comments_re
    }

    fn eol_comments_re(&self) -> &Regex {
        &self._eol_comments_re
    }
}

#[derive(Debug, Clone)]
pub struct StrCursor<'a, P = DefaultPatterns> {
    text: &'a str,
    offset: usize,
    _p: PhantomData<P>, // Zero-sized: 0 bytes
}

impl<'a, P: Patterns> StrCursor<'a, P> {
    pub fn new(text: &'a str) -> Self {
        let _p = P::default();
        Self {
            text,
            offset: 0,
            _p: PhantomData,
        }
    }

    fn eat_regex(&mut self, re: &Regex) -> bool {
        #[cfg(feature = "regex")]
        if let Some(mat) = re.find_at(self.text, self.offset) {
            if mat.start() != self.offset {
                return false;
            }
            self.offset = mat.end();
            return true;
        }
        false
    }
}

impl<'a, P: Patterns> Cursor for StrCursor<'a, P> {
    fn mark(&self) -> usize {
        self.offset
    }

    fn reset(&mut self, mark: usize) {
        self.offset = mark;
    }

    fn textstr(&self) -> &str {
        self.text
    }

    fn at_end(&self) -> bool {
        self.offset >= self.text.len()
    }

    fn next(&mut self) -> Option<char> {
        if self.at_end() {
            return None;
        }
        // Slice from the current byte offset to the end
        let rest = self.text.get(self.offset..)?;
        // Decode the first Unicode character
        let c = rest.chars().next()?;
        // Increment the byte offset by the character's UTF-8 width
        self.offset += c.len_utf8();

        Some(c)
    }

    fn token(&mut self, token: &str) -> bool {
        if self.text[self.offset..].starts_with(token) {
            self.offset += token.len();
            true
        } else {
            false
        }
    }

    #[cfg(feature = "regex")]
    fn pattern(&mut self, pattern: &str) -> Option<&'a str> {
        if pattern.is_empty() {
            return None;
        }
        {
            let re = Regex::new(pattern).ok()?;
            let caps = re.captures_at(self.text, self.offset)?;

            // Only match if it starts EXACTLY at the cursor
            let whole = caps.get(0)?;
            if whole.start() != self.offset {
                return None;
            }

            // Move the cursor by the whole match, but return the "Value"
            self.offset = whole.end();

            // Logic: If there is 1+ group, return group 1. Else return group 0.
            Some(caps.get(1).or(caps.get(0))?.as_str())
        }
    }

    fn next_token(&mut self) {
        let patterns = P::default();
        let wre = patterns.whitespace_re();
        let cre = patterns.comments_re();
        let ere = patterns.eol_comments_re();

        let mut last_offset = usize::MAX;
        while self.offset != last_offset {
            last_offset = self.offset;
            self.eat_regex(wre);
            if self.eat_regex(ere) {
                self.eat_regex(wre);
            }
            self.eat_regex(cre);
        }
    }
}
