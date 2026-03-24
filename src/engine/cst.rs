// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::ast::Ast;

#[derive(Debug, Clone, PartialEq)]
pub enum Cst {
    Token(String),
    List(Vec<Cst>),
    Closed(Vec<Cst>),
    Ast(Ast),
    Void
}

impl From<Vec<Cst>> for Cst {
    fn from(v: Vec<Cst>) -> Self {
        Cst::List(v)
    }
}

impl<const N: usize> From<[Cst; N]> for Cst {
    fn from(arr: [Cst; N]) -> Self {
        Cst::List(arr.into())
    }
}

impl Cst {
    pub fn add(self, node: Cst) -> Cst {
        match self {
            Cst::Void => node,
            Cst::List(mut list) => {
                list.push(node);
                Cst::List(list)
            },
            _ => Cst::List(vec![self, node])
        }
    }

    pub fn addlist(self, node: Cst) -> Cst {
        match self {
            Cst::Void => Cst::List(vec![node]),
            Cst::List(mut list) => {
                list.push(node);
                Cst::List(list)
            }
            _ => {
                Cst::List(vec![self, node])
            }
        }
    }

    pub fn merge(self, node: Cst) -> Cst {
        match (&self, &node) {
            (Cst::List(list), Cst::List(other)) => {
                let mut list = list.clone();
                list.extend(other.clone());
                Cst::List(list)
            }
            // If we have an existing list, just push the whole node
            (Cst::List(list), _) => {
                let mut list = list.clone();
                list.push(node);
                Cst::List(list)
            }
            // For everything else, use the standard add logic
            _ => self.add(node),
        }
    }
    
    pub fn closed(self) -> Cst {
        match self {
            Cst::List(list) if list.len() == 1 => {
                let item = list.into_iter().next().unwrap();
                item
            }
            Cst::List(list) => {
                Cst::Closed(list)
            }
            _ => self,
        }
    }
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