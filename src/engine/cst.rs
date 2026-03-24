// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;
use super::ast::Ast;

pub type CstRc = Rc<Cst>;

#[derive(Debug, Clone, PartialEq)]
pub enum Cst {
    Token(String),
    List(Vec<CstRc>),
    Closed(Vec<CstRc>),
    Ast(Ast),
    Void
}

impl Default for Cst {
    fn default() -> Self {
        Cst::Void
    }
}

impl From<Vec<Cst>> for Cst {
    fn from(v: Vec<Cst>) -> Self {
        let rcvec = v.into_iter().map(|c| Rc::new(c)).collect();
        Cst::List(rcvec)
    }
}

impl<const N: usize> From<[Cst; N]> for Cst {
    fn from(arr: [Cst; N]) -> Self {
        let rcvec = arr.into_iter().map(|c| Rc::new(c)).collect();
        Cst::List(rcvec)
    }
}

impl Cst {
    pub fn add(self, node: Cst) -> CstRc {
        let noderc = Rc::new(node);
        match self {
            Cst::Void => noderc,
            Cst::List(mut list) => {
                list.push(noderc);
                Rc::new(Cst::List(list))
            },
            _ => Rc::new(Cst::List(vec![Rc::new(self), noderc]))
        }
    }

    pub fn addlist(self, node: Cst) -> CstRc {
        let noderc = Rc::new(node);
        match self {
            Cst::Void => Rc::new(Cst::List(vec![noderc])),
            Cst::List(mut list) => {
                list.push(noderc);
                Rc::new(Cst::List(list))
            }
            _ => {
                Rc::new(Cst::List(vec![Rc::new(self), noderc]))
            }
        }
    }

    pub fn merge(self, node: Cst) -> CstRc {
        match (self, node) {
            (Cst::List(mut list), Cst::List(other_list)) => {
                list.extend(other_list);
                Rc::new(Cst::List(list))
            }
            (Cst::List(mut list), other_node) => {
                list.push(Rc::new(other_node));
                Rc::new(Cst::List(list))
            }
            (some_node, Cst::List(mut other_list)) => {
                other_list.insert(0, Rc::new(some_node));
                Rc::new(Cst::List(other_list))
            }
            (s, n) => s.add(n),
        }
    }

    pub fn closed(self) -> CstRc {
        match self {
            Cst::List(list) if list.len() == 1 => {
                let item = list.into_iter().next().unwrap();
                item
            }
            Cst::List(list) => {
                Rc::new(Cst::Closed(list))
            }
            _ => Rc::new(self),
        }
    }
}

pub fn cst_add(prev: Cst, node: Cst) -> CstRc {
    prev.add(node)
}

pub fn cst_addlist(prev: Cst, node: Cst) -> CstRc {
    prev.addlist(node)
}

pub fn cst_merge(prev: Cst, node: Cst) -> CstRc {
    prev.merge(node)
}

pub fn cst_closed(cst: Cst) -> CstRc {
    cst.closed()
}