// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::rc::Rc;

type SExp<S> = Rc<Cell<S>>;

#[derive(Clone, Default)]
pub enum Cell<S: Clone> {
    Cons(SExp<S>, SExp<S>),
    Atom(S),
    #[default]
    Nil,
}

impl<S> Cell<S>
where
    S: Clone,
{
    pub fn atom(a: S) -> Self {
        Cell::Atom(a)
    }

    pub fn cons(car: Self, cdr: Self) -> Self {
        Cell::Cons(car.into(), cdr.into())
    }

    pub fn car(&self) -> Option<Self> {
        match self {
            Cell::Cons(car, _) => Some((**car).clone()),
            _ => None,
        }
    }

    pub fn cdr(&self) -> Option<Self> {
        match self {
            Cell::Cons(_, cdr) => Some((**cdr).clone()),
            _ => None,
        }
    }

    pub fn to_vec(&self) -> Vec<S> {
        match self {
            Cell::Atom(a) => vec![a.clone()],
            Cell::Cons(car, cdr) => {
                let mut atoms = car.to_vec();
                atoms.extend(cdr.to_vec());
                atoms
            }
            Cell::Nil => vec![],
        }
    }

    pub fn to_arr(&self) -> Box<[S]> {
        self.to_vec().into_boxed_slice()
    }
}
