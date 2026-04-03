// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::ast::Ast;
use std::ops::Add;
use std::ops::Deref;

pub type KeyValue = (Box<str>, Cst);

pub fn keyval(name: &str, cst: Cst) -> KeyValue {
    (name.into(), cst.clone())
}

#[derive(Debug, Clone, PartialEq)]
pub enum Cst {
    Token(Box<str>),          // 16 bytes
    Literal(Box<str>),        // 16 bytes
    Number(f64),              // 8 bytes
    List(Box<[Cst]>),         // 16 bytes
    Closure(Box<[Cst]>),      // 16 bytes
    Named(Box<KeyValue>),     // 8 bytes
    NamedList(Box<KeyValue>), // 8 bytes
    OverrideValue(Box<Cst>),  // 8 bytes
    OverrideList(Box<Cst>),   // 8 bytes
    Ast(Box<Ast>),            // 8 bytes
    Void,
    Nil,
    Bottom,
}

pub fn cst_add(prev: Cst, node: Cst) -> Cst {
    prev.add(node)
}

pub fn cst_addlist(prev: Cst, node: Cst) -> Cst {
    prev.addlist(node)
}

pub fn cst_merge(prev: Cst, node: Cst) -> Cst {
    prev.merge(node)
}

pub fn cst_closed(cst: Cst) -> Cst {
    cst.closed()
}

impl Add for Cst {
    type Output = Self;

    fn add(self, node: Self) -> Self {
        self._add(node)
    }
}

impl From<Vec<Cst>> for Cst {
    fn from(v: Vec<Cst>) -> Self {
        Cst::List(v.into_boxed_slice())
    }
}

impl<const N: usize> From<[Cst; N]> for Cst {
    fn from(arr: [Cst; N]) -> Self {
        Cst::List(arr.into())
    }
}


impl Cst {
    pub fn named(key: &str, value: Cst) -> Self {
        Cst::Named(Box::new(keyval(key, value)))
    }

    pub fn named_list(key: &str, value: Cst) -> Self {
        Cst::NamedList(Box::new(keyval(key, value)))
    }

    fn _add(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => n,
            (s, Self::Nil) => s,
            (Self::List(list), node) => {
                let mut v = list.into_vec();
                v.push(node);
                Self::List(v.into_boxed_slice())
            }
            (s, n) => Self::List(vec![s, n].into_boxed_slice()),
        }
    }

    pub fn addlist(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => Self::List(vec![n].into_boxed_slice()),
            (s, Self::Nil) => s,
            (Self::List(list), n) => {
                let mut v = list.into_vec();
                v.push(n);
                Self::List(v.into_boxed_slice())
            }
            (s, n) => Self::List(vec![s, n].into_boxed_slice()),
        }
    }

    pub fn merge(self, node: Self) -> Self {
        match (self, node) {
            (Self::Nil, n) => n,
            (s, Self::Nil) => s,
            (Self::List(l1), Self::List(l2)) => {
                let mut v = l1.into_vec();
                v.extend(l2.into_vec());
                Self::List(v.into_boxed_slice())
            }
            (Self::List(l1), n) => {
                let mut v = l1.into_vec();
                v.push(n);
                Self::List(v.into_boxed_slice())
            }
            (s, Self::List(l2)) => {
                let mut v = vec![s];
                v.extend(l2.into_vec());
                Self::List(v.into_boxed_slice())
            }
            (s, n) => s.add(n),
        }
    }

    pub fn closed(self) -> Cst {
        match self {
            Cst::List(list) => Cst::Closure(list),
            _ => self,
        }
    }

    fn _distill(self) -> (Ast, Cst, Cst) {
        let mut ast = Ast::new();
        let mut ovr = Cst::Nil;
        let mut cst = Cst::Nil;

        match self {
            Cst::List(elements) => {
                for node in elements {
                    let (child_ast, child_ovr, child_cst) = node._distill();

                    ast.update(&child_ast);
                    ovr = ovr.merge(child_ovr);
                    cst = cst.merge(child_cst);
                }
            }
            Cst::Named(keyval) => {
                let (name, val) = keyval.deref();
                ast.set(name, val.clone())
            }
            Cst::NamedList(keyval) => {
                let (name, val) = keyval.deref();
                ast.set_list(name, val.clone())
            }
            Cst::OverrideValue(val) => ovr = ovr.add(*val),
            Cst::OverrideList(val) => ovr = ovr.addlist(*val),
            Cst::Nil => {}
            other => cst = cst.merge(other),
        }

        (ast, ovr, cst)
    }

    pub fn node(self) -> Cst {
        let (ast, ovr, cst) = self._distill();

        // Priority Gate: Override > AST > CST
        if ovr != Cst::Nil {
            cst_closed(ovr)
        } else if !ast.is_empty() {
            Cst::Ast(ast.into())
        } else {
            cst_closed(cst)
        }
    }

    pub fn length(&self) -> usize {
        match self {
            Cst::Token(text) | Cst::Literal(text) => text.len(),

            Cst::Number(val) => val.to_string().len(),

            Cst::List(items) | Cst::Closure(items) => items.iter().map(|item| item.length()).sum(),

            Cst::Named(pair) | Cst::NamedList(pair) => {
                let (_, val) = &**pair;
                val.length()
            }

            Cst::OverrideValue(inner) | Cst::OverrideList(inner) => inner.length(),

            Cst::Ast(ast) => ast.fields.values().map(|node| node.length()).sum(),

            Cst::Void | Cst::Nil | Cst::Bottom => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET: usize = 32;

    #[test]
    fn test_cst_size() {
        let size = size_of::<Cst>();
        // 24 bytes: Box (8) + Rc (8) + bool/padding (8)
        assert!(size <= TARGET, "Cst size is {} > {} bytes", size, TARGET);
    }

    #[test]
    fn test_node_nil_removal() {
        // Input: List([Nil, Bottom, Nil])
        // Step 1: cst = merge(Nil, Nil) -> Nil
        // Step 2: cst = merge(Nil, Bottom) -> Bottom
        // Step 3: cst = merge(Bottom, Nil) -> Bottom (The Identity Law)
        let raw = Cst::List([Cst::Nil, Cst::Bottom, Cst::Nil].into());
        let result = raw.node();

        // result = cst_closed(Bottom)
        assert_eq!(result, cst_closed(Cst::Bottom));
    }

    #[test]
    fn test_node_nil_removal_to_bottom() {
        let raw = Cst::List([Cst::Nil, Cst::Bottom, Cst::Nil].into());
        let result = raw.node();

        // If cst_closed is identity for non-lists, this is just Bottom
        assert_eq!(result, Cst::Bottom);
    }

    #[test]
    fn test_node_nil_removal_to_list() {
        // Input: [Bottom, Nil, Bottom]
        // Step 1: cst = merge(Bottom, Nil) -> Bottom
        // Step 2: cst = merge(Bottom, Bottom) -> List([Bottom, Bottom])
        let raw = Cst::List([Cst::Bottom, Cst::Nil, Cst::Bottom].into());
        let result = raw.node();

        if let Cst::Closure(v) = result {
            assert_eq!(v.len(), 2); // Nil is gone, only the two Bottoms remain
            assert_eq!(v[0], Cst::Bottom);
            assert_eq!(v[1], Cst::Bottom);
        } else {
            panic!("Expected Closure, got {:?}", result);
        }
    }

    #[test]
    fn test_node_nil_purging_preserves_count() {
        // Input: List([Nil, Bottom, Nil])
        let raw = Cst::List([Cst::Nil, Cst::Bottom, Cst::Nil].into());
        let result = raw.node();

        // Since it's effectively Bottom, and Bottom isn't a list,
        // it doesn't become a Closure of len 1. It just stays Bottom.
        assert_eq!(result, Cst::Bottom);
    }
}
