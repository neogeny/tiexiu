// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::ast::Ast;

#[derive(Debug, Clone, PartialEq)]
pub enum Cst {
    Token(String),
    Literal(String),
    List(Vec<Cst>),
    Closure(Vec<Cst>),
    Named(String, Box<Cst>),
    NamedList(String, Box<Cst>),
    OverrideValue(Box<Cst>),
    OverrideList(Box<Cst>),
    Ast(Ast),
    Nil,
}

impl Default for Cst {
    fn default() -> Self {
        Cst::Nil
    }
}

impl From<Vec<Cst>> for Cst {
    fn from(v: Vec<Cst>) -> Self {
        Cst::List(v)
    }
}


impl<'c, const N: usize> From<[Cst; N]> for Cst {
    fn from(arr: [Cst; N]) -> Self {
        Cst::List(arr.into())
    }
}

impl Cst {
    pub fn add(self, node: Cst) -> Cst {
        match self {
            Cst::Nil => node,
            Cst::List(mut list) => {
                list.push(node);
                Cst::List(list)
            }
            _ => Cst::List(vec![self, node]),
        }
    }

    pub fn addlist(self, node: Cst) -> Cst {
        match self {
            Cst::Nil => Cst::List(vec![node]),
            Cst::List(mut list) => {
                list.push(node);
                Cst::List(list)
            }
            _ => Cst::List(vec![self, node]),
        }
    }

    pub fn merge(self, node: Cst) -> Cst {
        match (self, node) {
            (Cst::List(mut list), Cst::List(other_list)) => {
                list.extend(other_list);
                Cst::List(list)
            }
            (Cst::List(mut list), other_node) => {
                list.push(other_node);
                Cst::List(list)
            }
            (self_node, Cst::List(mut other_list)) => {
                other_list.insert(0, self_node);
                Cst::List(other_list)
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

            Cst::Named(name, val) => ast.set(&name, *val),
            Cst::NamedList(name, val) => ast.set_list(&name, *val),
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
            Cst::Ast(ast)
        } else {
            cst_closed(cst)
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