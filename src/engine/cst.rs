// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::ast::Ast;

#[derive(Debug, Clone, PartialEq)]
pub enum Cst {
    Token(&'static str),
    Literal(&'static str),
    Item(Box<Cst>),
    List(Vec<Box<Cst>>),
    Closure(Vec<Box<Cst>>),
    Named(&'static str, Box<Cst>),
    NamedList(&'static str, Box<Cst>),
    OverrideValue(Box<Cst>),
    OverrideList(Box<Cst>),
    Ast(Box<Ast>),
    Nil,
}

impl Default for Cst {
    fn default() -> Self {
        Cst::Nil
    }
}

impl From<Vec<Cst>> for Cst {
    fn from(v: Vec<Cst>) -> Self {
        let boxed = v.into_iter().map(Box::new).collect();
        Cst::List(boxed)
    }
}


impl<const N: usize> From<[Cst; N]> for Cst {
    fn from(arr: [Cst; N]) -> Self {
        let boxed = arr.into_iter().map(Box::new).collect();
        Cst::List(boxed)
    }
}

impl Cst {
    pub fn add(self, node: Cst) -> Cst {
        let node_box = Box::new(node);
        match self {
            Cst::Nil => *node_box,
            Cst::List(mut list) => {
                list.push(node_box);
                Cst::List(list)
            }
            _ => Cst::List(vec![Box::new(self), node_box]),
        }
    }

    pub fn addlist(self, node: Cst) -> Cst {
        let node_box = Box::new(node);
        match self {
            Cst::Nil => Cst::List(vec![node_box]),
            Cst::List(mut list) => {
                list.push(node_box);
                Cst::List(list)
            }
            _ => Cst::List(vec![Box::new(self), node_box]),
        }
    }

    pub fn merge(self, node: Cst) -> Cst {
        match (self, node) {
            (Cst::List(mut list), Cst::List(other_list)) => {
                list.extend(other_list);
                Cst::List(list)
            }
            (Cst::List(mut list), other_node) => {
                list.push(Box::new(other_node));
                Cst::List(list)
            }
            (some_node, Cst::List(mut other_list)) => {
                other_list.insert(0, Box::new(some_node));
                Cst::List(other_list)
            }
            (s, n) => s.add(n),
        }
    }

    pub fn closed(self) -> Cst {
        match self {
            Cst::List(mut list) if list.len() == 1 => *list.pop().unwrap(),
            Cst::List(list) => Cst::Closure(list),
            Cst::Item(cst) => *cst,
            _ => self,
        }
    }

    pub fn distill(self) -> Cst {
        match self {
            Cst::List(elements) => {
                let mut new_list = Vec::new();
                let mut ast = Ast::new();

                for node in elements {
                    // Recurse to handle nested structures
                    let child = node.distill();
                    
                    match child {
                        Cst::Nil => continue,
                        
                        // Labels are extracted to the current rule's AST
                        Cst::Named(name, value) => {
                            ast.set(&name, *value);
                        }
                        
                        // An Ast from a sub-rule is just another positional item
                        // unless it was Named (handled above).
                        other => new_list.push(Box::new(other)),
                    }
                }

                if !ast.is_empty() {
                    Cst::Ast(Box::new(ast))
                } else {
                    Cst::List(new_list)
                }
            }
            // A standalone Named node always promotes to an Ast
            Cst::Named(name, value) => {
                let mut ast = Ast::new();
                ast.set(&name, *value);
                Cst::Ast(Box::new(ast))
            }
            other => other,
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