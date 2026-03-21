// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::parsed::{Parsed, ParsedValue};

/// The Carrier Enum: Manages the "Open" vs "Closed" state of the CST.
///
/// A `List` is "Open" and will be flattened during a merge.
/// An `Item` is "Closed" (atomic) and will be appended as a single unit.
pub enum Cst {
    List(Vec<Parsed>),
    Item(Box<Parsed>),
}

impl Cst {
    /// Attempts to get a reference to the inner Parsed object if it is an Item.
    pub fn as_item(&self) -> Option<&Parsed> {
        match self {
            Cst::Item(p) => Some(p),
            Cst::List(_) => None,
        }
    }
}

/// Closes a list by boxing it into an Item, making it atomic.
pub fn cst_final(cst: Cst) -> Cst {
    match cst {
        Cst::List(v) => Cst::Item(Box::new(Parsed::from(Cst::List(v)))),
        item => item,
    }
}

/// Adds a Parsed item to the CST structure.
pub fn cst_add(cst: Option<Cst>, item: Parsed, as_list: bool) -> Cst {
    match (cst, as_list) {
        (None, true) => Cst::List(vec![item]),
        (None, false) => Cst::Item(Box::new(item)),
        (Some(Cst::List(mut v)), _) => {
            v.push(item);
            Cst::List(v)
        }
        (Some(Cst::Item(i)), _) => Cst::List(vec![*i, item]),
    }
}

/// Merges an existing structure into another, flattening if it's an open list.
///
/// Because we are using the `Parsed` struct, we can inspect the content
/// directly without expensive downcasting.
pub fn cst_merge(cst: Option<Cst>, other: Parsed) -> Cst {
    // Determine if the incoming 'other' is already a Cst we can flatten
    let other_cst = match other.value {
        ParsedValue::Cst(c) => c,
        _ => Cst::Item(Box::new(other)),
    };

    match (cst, other_cst) {
        (None, right) => right,

        (Some(Cst::List(mut l_vec)), Cst::List(r_vec)) => {
            l_vec.extend(r_vec);
            Cst::List(l_vec)
        }
        (Some(Cst::List(mut l_vec)), Cst::Item(r_node)) => {
            l_vec.push(*r_node);
            Cst::List(l_vec)
        }
        (Some(Cst::Item(l_node)), Cst::List(mut r_vec)) => {
            let mut new_vec = Vec::with_capacity(r_vec.len() + 1);
            new_vec.push(*l_node);
            new_vec.append(&mut r_vec);
            Cst::List(new_vec)
        }
        (Some(Cst::Item(l_node)), Cst::Item(r_node)) => {
            Cst::List(vec![*l_node, *r_node])
        }
    }
}