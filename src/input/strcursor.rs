use super::Cursor;
#[cfg(feature = "regex")]
use regex::Regex;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

pub trait Patterns: Clone + Debug + Default {
    const WHITESPACE: &'static str = r"\s+";
    const EOL_COMMENTS: &'static str = r"//.*$";
    const COMMENTS: &'static str = r"";

    fn whitespace_re(&self) -> Regex;
    fn comments_re(&self) -> Regex;
    fn eol_comments_re(&self) -> Regex;
}

#[derive(Clone, Debug)]
pub struct DefaultPatterns {
    _whitespace_re: Box<Regex>,
    _comments_re: Box<Regex>,
    _eol_comments_re: Box<Regex>,
}

impl Default for DefaultPatterns {
    fn default() -> Self {
        Self {
            _whitespace_re: Regex::new(DefaultPatterns::WHITESPACE).unwrap().into(),
            _comments_re: Regex::new(DefaultPatterns::COMMENTS).unwrap().into(),
            _eol_comments_re: Regex::new(DefaultPatterns::EOL_COMMENTS).unwrap().into(),
        }
    }
}

impl Patterns for DefaultPatterns {
    fn whitespace_re(&self) -> Regex {
        self._whitespace_re.deref().clone()
    }

    fn comments_re(&self) -> Regex {
        self._comments_re.deref().clone()
    }

    fn eol_comments_re(&self) -> Regex {
        self._eol_comments_re.deref().clone()
    }
}

#[derive(Debug, Clone)]
pub struct StrCursor<'a, P = DefaultPatterns> {
    text: &'a str,
    offset: usize,
    patterns: Rc<P>, // Zero-sized: 0 bytes
}

impl<'a, P: Patterns> StrCursor<'a, P> {
    pub fn new(text: &'a str) -> Self {
        let patterns = P::default();
        Self {
            text,
            offset: 0,
            patterns: patterns.into()
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
    fn pattern_re(&mut self, re: &Regex) -> Option<String> {
        let caps = re.captures_at(self.text, self.offset)?;

        let whole = caps.get(0)?;
        if whole.start() != self.offset {
            return None;
        }

        self.offset = whole.end();
        Some(caps.get(1).or(caps.get(0))?.as_str().into())
    }

    fn next_token(&mut self) {
        let wre = &self.patterns.whitespace_re();
        let cre = &self.patterns.comments_re();
        let ere = &self.patterns.eol_comments_re();

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
