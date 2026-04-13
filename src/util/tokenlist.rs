// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A simple Lisp-like List implementation.
//! Optimized for ergonomic one-liners and memory safety.

use std::fmt;
use std::rc::Rc;

pub type Token = Rc<str>;

#[derive(Clone, Default)]
pub enum TokenList {
    Cons(Rc<TokenList>, Rc<TokenList>),
    Atom(Token),
    #[default]
    Nil,
}

impl fmt::Display for TokenList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.to_vec().join(" "))
    }
}

impl fmt::Debug for TokenList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TokenList[{}]", self.to_vec().join(" "))
    }
}

impl TokenList {
    pub fn new() -> Self {
        TokenList::Nil
    }

    pub fn atom(a: &str) -> Self {
        TokenList::Atom(a.into())
    }

    pub fn insert(self, a: &str) -> Self {
        let atom = TokenList::atom(a);
        match self {
            TokenList::Nil => atom,
            _ => TokenList::Cons(atom.into(), self.into()),
        }
    }

    pub fn append(&self, a: &str) -> Self {
        let atom = TokenList::atom(a);
        match self {
            TokenList::Nil => atom,
            TokenList::Atom(_) => TokenList::Cons(self.clone().into(), atom.into()),
            TokenList::Cons(car, cdr) => TokenList::Cons(car.clone(), cdr.append(a).into()),
        }
    }

    pub fn tail(&self) -> Option<&TokenList> {
        match self {
            TokenList::Cons(_, cdr) => Some(cdr),
            TokenList::Atom(_) => None,
            TokenList::Nil => None,
        }
    }

    pub fn first(&self) -> Option<&str> {
        let mut current = self;
        loop {
            match current {
                TokenList::Atom(a) => return Some(a),
                TokenList::Cons(car, _) => current = car,
                TokenList::Nil => return None,
            }
        }
    }

    pub fn last(&self) -> Option<&str> {
        let mut current = self;
        loop {
            match current {
                TokenList::Atom(a) => return Some(a),
                TokenList::Cons(_, cdr) => current = cdr,
                TokenList::Nil => return None,
            }
        }
    }


    pub fn to_vec(&self) -> Vec<&str> {
        let mut atoms = Vec::new();
        let mut stack = vec![self];

        while let Some(current) = stack.pop() {
            match current {
                TokenList::Atom(a) => atoms.push(&**a),
                TokenList::Cons(car, cdr) => {
                    stack.push(cdr);
                    stack.push(car);
                }
                TokenList::Nil => {}
            }
        }
        atoms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET: usize = 24;

    #[test]
    fn test_tokelist_size() {
        let size = size_of::<TokenList>();
        assert!(
            size <= TARGET,
            "TokenList size is {} > {} bytes",
            size,
            TARGET
        );
    }
}
