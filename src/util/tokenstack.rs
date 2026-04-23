// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A simple Lisp-like List implementation.
//! Optimized for ergonomic one-liners and memory safety.

use std::fmt;
use std::rc::Rc;

pub type Token = Rc<str>;

#[derive(Debug, Clone)]
enum Node {
    Cons(Rc<Node>, Rc<Node>),
    Atom(Token),
    Nil,
}

impl Node {
    fn new_with_tail(&self, a: &str) -> Self {
        match self {
            Node::Nil => Node::Atom(a.into()),
            Node::Atom(_) => Node::Cons(Rc::new(self.clone()), Rc::new(Node::Atom(a.into()))),
            Node::Cons(car, cdr) => Node::Cons(car.clone(), Rc::new(cdr.new_with_tail(a))),
        }
    }
}

#[derive()]
pub struct TokenStack(Rc<Node>);

impl Default for TokenStack {
    fn default() -> Self {
        Self(Rc::new(Node::Nil))
    }
}

impl Clone for TokenStack {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl fmt::Display for TokenStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.to_vec().join(" "))
    }
}

impl fmt::Debug for TokenStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.to_vec().join(" "))
    }
}

impl TokenStack {
    pub fn new() -> Self {
        Self(Rc::new(Node::Nil))
    }

    pub fn atom(a: &str) -> Self {
        Self(Rc::new(Node::Atom(a.into())))
    }

    /// Prepends a new atom to the list (O(1)).
    /// Reuses the existing list structure via Rc::clone.
    pub fn push(&mut self, a: &str) {
        let atom = Node::Atom(a.into());
        let new_node = Node::Cons(Rc::new(atom), Rc::clone(&self.0));
        self.0 = Rc::new(new_node);
    }

    /// Returns a new TokenList with the provided string as the last element (O(N)).
    pub fn new_with_tail(&self, a: &str) -> Self {
        Self(Rc::new(self.0.new_with_tail(a)))
    }

    pub fn tail(&self) -> Option<TokenStack> {
        match self.0.as_ref() {
            Node::Cons(_, cdr) => Some(Self(Rc::clone(cdr))),
            _ => None,
        }
    }

    pub fn first(&self) -> Option<&str> {
        let mut current = &self.0;
        loop {
            match current.as_ref() {
                Node::Atom(a) => return Some(a),
                Node::Cons(car, _) => current = car,
                Node::Nil => return None,
            }
        }
    }

    pub fn iter(&self) -> TokenListIter<'_> {
        TokenListIter::new(self)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.0.as_ref(), Node::Nil)
    }

    pub fn to_vec(&self) -> Vec<&str> {
        self.iter().collect()
    }
}

pub struct TokenListIter<'a> {
    stack: Vec<&'a Node>,
}

impl<'a> TokenListIter<'a> {
    fn new(list: &'a TokenStack) -> Self {
        Self {
            stack: vec![list.0.as_ref()],
        }
    }
}

impl<'a> Iterator for TokenListIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = self.stack.pop() {
            match current {
                Node::Atom(a) => return Some(a),
                Node::Cons(car, cdr) => {
                    self.stack.push(cdr.as_ref());
                    self.stack.push(car.as_ref());
                }
                Node::Nil => {}
            }
        }
        None
    }
}

impl<'a> IntoIterator for &'a TokenStack {
    type Item = &'a str;
    type IntoIter = TokenListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl FromIterator<Token> for TokenStack {
    fn from_iter<I: IntoIterator<Item = Token>>(iter: I) -> Self {
        let mut list = TokenStack::new();
        for token in iter {
            let atom = Node::Atom(token);
            let new_node = Node::Cons(Rc::new(atom), Rc::clone(&list.0));
            list.0 = Rc::new(new_node);
        }
        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    const TARGET: usize = 8; // On 64-bit, Rc is 8 bytes

    #[test]
    fn test_tokelist_size() {
        let size = size_of::<TokenStack>();
        assert!(
            size <= TARGET,
            "TokenList size is {} > {} bytes",
            size,
            TARGET
        );
    }

    #[test]
    fn test_push_behavior() {
        let mut list = TokenStack::new();
        list.push("a");
        list.push("b");
        assert_eq!(list.to_string(), "[b a]");
    }

    #[test]
    fn test_new_with_tail() {
        let list = TokenStack::new().new_with_tail("a").new_with_tail("b");
        assert_eq!(list.to_string(), "[a b]");
    }

    #[test]
    fn test_iteration() {
        let mut list = TokenStack::new();
        list.push("a");
        list.push("b");
        let vec: Vec<_> = list.iter().collect();
        assert_eq!(vec, vec!["b", "a"]);
    }
}
