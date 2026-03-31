// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;
use crate::input::{Cursor, StrCursor};
use crate::grammars::{CanParse, ParseResult};
use std::fmt::Debug;

pub trait Ctx: Clone + Debug {
    // fn resolve(&self, name: &str) -> Box<dyn CanParse<Self>>;
    fn call(self, name: &str) -> ParseResult<Self>;
    fn mark(&self) -> usize;
    fn dot(&self) -> bool;
    fn eof_check(&self) -> bool;
    fn next(&self) -> Option<char>;
    fn token(&self, token: &str) -> bool;
    fn pattern(&self, pattern: &str) -> bool;
    fn search(&self, pattern: &str) -> bool;

    fn cut(&mut self);
    fn uncut(&mut self);
    fn cut_seen(&self) -> bool;
}

#[derive(Clone, Debug)]
pub struct StrCtx<'c> {
    cursor: StrCursor<'c>,
    _cut_seen: bool,
    rulemap: HashMap<&'c str, &'c dyn CanParse<Self>>,
}

impl<'c> StrCtx<'c> {
    pub fn new(cursor: StrCursor<'c>) -> Self {
        Self {
            cursor,
            _cut_seen: false,
            rulemap: HashMap::new(),
        }
    }
}

impl<'c> Ctx for StrCtx<'c> {
    // fn resolve(&self, name: &str) -> Box<dyn CanParse<Self>> {
    //     let rule = self.rulemap
    //         .get(name)
    //         .expect("Rule not found");
    //     Box::new(rule)
    // }

    fn call(self, name: &str) -> ParseResult<Self> {
        let rule = self.rulemap
            .get(name)
            .expect("Rule not found");
        rule.parse(self)
    }

    fn mark(&self) -> usize {
        self.cursor.mark()
    }

    fn dot<'a>(&self) -> bool {
        unimplemented!()
    }

    fn eof_check(&self) -> bool {
        unimplemented!()
    }

    fn next(&self) -> Option<char> {
        // do it with cursor goto(+1)?
        unimplemented!()
    }

    fn token<'p>(&self, _token: &str) -> bool {
        // if self.cursor.token(token) {
        //     Ok(
        //         (self, Cst::Token(token.into()))
        //     )
        // }
        // else {
        //     Err(self)
        // }
        unimplemented!()
    }

    fn pattern(&self, _pattern: &str) -> bool {
        unimplemented!()
    }

    fn search(&self, _pattern: &str) -> bool {
        unimplemented!()
    }

    fn cut(&mut self) {
        self._cut_seen = true;
    }

    fn uncut(&mut self) {
        self._cut_seen = false;
    }

    fn cut_seen(&self) -> bool {
        self._cut_seen
    }
}

#[derive(Clone, Debug)]
pub struct CtxImpl<'c> {
    pub cursor: &'c dyn Cursor,
    pub cut_seen: bool,
    pub error_msg: Option<String>,
    // pub rules: Rc<HashMap<&'c str, &'c Rule<'c, CtxImpl>>>,
}