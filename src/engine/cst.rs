// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::parsed::{Parsed, ParsedValue};

/// The Concrete Syntax Tree (CST) representation.
/// 
/// This is the recursive backbone of the value system.
#[derive(Debug, Clone, PartialEq)]
pub enum Cst {
    Item(Box<Parsed>),
    List(Vec<Parsed>),
}

/// Adds a node to an existing CST, optionally forcing list behavior.
pub fn cst_add(current: Option<Cst>, node: Parsed, as_list: bool) -> Cst {
    match current {
        None => {
            if as_list {
                Cst::List(vec![node])
            } else {
                Cst::Item(Box::new(node))
            }
        }
        Some(Cst::Item(prev)) => {
            // Upgrade Item to List
            Cst::List(vec![*prev, node])
        }
        Some(Cst::List(mut list)) => {
            list.push(node);
            Cst::List(list)
        }
    }
}

/// Merges a node into the current CST context.
pub fn cst_merge(current: Option<Cst>, node: Parsed) -> Cst {
    // Match on a reference to the value to avoid a partial move
    match (&current, &node.value) {
        // If both are lists, we merge the elements
        (Some(Cst::List(_)), ParsedValue::Cst(Cst::List(other_list))) => {
            if let Some(Cst::List(mut list)) = current {
                list.extend(other_list.clone());
                return Cst::List(list);
            }
            Cst::List(vec![node]) // Fallback (should be unreachable)
        }
        // If we have an existing list, just push the whole node
        (Some(Cst::List(_)), _) => {
            if let Some(Cst::List(mut list)) = current {
                list.push(node);
                return Cst::List(list);
            }
            Cst::List(vec![node]) // Fallback
        }
        // For everything else, use the standard add logic
        _ => cst_add(current, node, false),
    }
}

/// Finalizes a CST node (e.g., unwrapping single-item lists if necessary).
pub fn cst_final(cst: Cst) -> Cst {
    match cst {
        Cst::List(list) if list.len() == 1 => {
            let item = list.into_iter().next().unwrap();
            match item.value {
                ParsedValue::Cst(inner) => inner,
                _ => Cst::Item(Box::new(item)),
            }
        }
        _ => cst,
    }
}