// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::any::Any;

/// The primary opaque type for user-defined semantic nodes.
///
/// In the "Iron" model, this is a heap-allocated box containing
/// a type-erased value that the parser moves but does not inspect.
pub type Node = Box<dyn Any>;

/// The Carrier Enum: Manages the "Open" vs "Closed" state of the CST.
///
/// A `List` is considered "Open" and will be flattened during a merge.
/// An `Item` is "Closed" (atomic) and will be appended as a single unit.
pub enum Cst {
    List(Vec<Node>),
    Item(Node),
}

impl Cst {
    /// Attempts to downcast a single Item to a specific type reference.
    ///
    /// Returns `None` if the variant is a `List` or if the type ID
    /// does not match the requested type `T`.
    pub fn as_ref<T: Any>(&self) -> Option<&T> {
        match self {
            Cst::Item(node) => node.downcast_ref::<T>(),
            Cst::List(_) => None,
        }
    }

    /// Takes ownership and attempts to downcast to a specific type.
    ///
    /// If the downcast fails, it returns the original `Cst` inside the `Err`
    /// variant to prevent the loss of the moved data.
    pub fn downcast<T: Any>(self) -> Result<T, Self> {
        match self {
            Cst::Item(node) => match node.downcast::<T>() {
                Ok(val) => Ok(*val),
                Err(node) => Err(Cst::Item(node)),
            },
            _ => Err(self),
        }
    }
}

/// Closes a list by boxing it into an Item, making it atomic.
///
/// Once a list is "finalized," it can no longer be flattened into
/// other lists during a `cst_merge` operation.
pub fn cst_final(cst: Cst) -> Cst {
    match cst {
        Cst::List(v) => Cst::Item(Box::new(v)),
        item => item,
    }
}

/// Adds a raw item of type T to the CST.
///
/// If `as_list` is true and `cst` is `None`, a new `List` is started.
/// Otherwise, the item is appended to the existing structure.
pub fn cst_add<T: Any>(cst: Option<Cst>, item: T, as_list: bool) -> Cst {
    let node: Node = Box::new(item);

    match (cst, as_list) {
        (None, true) => Cst::List(vec![node]),
        (None, false) => Cst::Item(node),
        (Some(Cst::List(mut v)), _) => {
            v.push(node);
            Cst::List(v)
        }
        (Some(Cst::Item(i)), _) => Cst::List(vec![i, node]),
    }
}

/// Merges an existing structure into another, flattening if it's an open list.
///
/// This function is symmetric with `cst_add`. It uses a single downcast
/// to determine if the incoming `other` is a `Cst` structure or a raw leaf.
pub fn cst_merge<T: Any>(cst: Option<Cst>, other: T) -> Cst {
    let other_any: Node = Box::new(other);

    let other_cst = match other_any.downcast::<Cst>() {
        Ok(extracted) => *extracted,
        Err(original_node) => Cst::Item(original_node),
    };

    match (cst, other_cst) {
        (None, right) => right,

        (Some(Cst::List(mut l_vec)), Cst::List(r_vec)) => {
            l_vec.extend(r_vec);
            Cst::List(l_vec)
        }
        (Some(Cst::List(mut l_vec)), Cst::Item(r_node)) => {
            l_vec.push(r_node);
            Cst::List(l_vec)
        }
        (Some(Cst::Item(l_node)), Cst::List(mut r_vec)) => {
            let mut new_vec = Vec::with_capacity(r_vec.len() + 1);
            new_vec.push(l_node);
            new_vec.append(&mut r_vec);
            Cst::List(new_vec)
        }
        (Some(Cst::Item(l_node)), Cst::Item(r_node)) => {
            Cst::List(vec![l_node, r_node])
        }
    }
}