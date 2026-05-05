// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]

pub mod api;
pub mod cfg;
pub mod context;
pub mod error;
pub mod input;
pub mod json;
pub mod peg;
pub mod tools;
pub mod trees;
pub mod util;

pub use api::*;
#[allow(unused_imports)]
pub use error::{Error, Result};

pub use peg::Exp;
pub use peg::ExpKind;
// re-exports
/// A comment about why this is re-exported here
pub use peg::Grammar;
pub use peg::Rule;
pub use trees::Tree;
pub use trees::TreeMap;


#[cfg(feature = "pyo3")]
pyo3::create_exception!(_tiexiu, ParseError, pyo3::exceptions::PyException);

#[cfg(feature = "pyo3")]
pub(crate) mod python;
