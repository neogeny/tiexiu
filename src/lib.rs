// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]

#[cfg(all(feature = "pyo3", not(target_env = "msvc")))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

/// Functional API (free functions) and the OO `TieXiu` wrapper.
pub mod api;
/// Configuration system for the parser engine.
pub mod cfg;
/// Parsing context types (cursor, memoization, state).
pub mod context;
/// Public error types and result alias.
pub mod error;
/// Input sources and cursor abstractions.
pub mod input;
pub(crate) mod json;
/// PEG grammar representation, compilation, and parsing.
pub mod peg;
/// Visualization and diagramming utilities (e.g., railroad diagrams).
pub mod tools;
/// Parse-tree types including `Tree` and `TreeMap`.
pub mod trees;
/// Internal utility modules (string tools, patterns, etc.).
pub mod util;

pub use api::*;
#[doc(inline)]
#[allow(unused_imports)]
pub use error::Error;
#[doc(inline)]
pub use error::Result;

#[cfg(feature = "pyo3")]
pyo3::create_exception!(_tiexiu, ParseError, pyo3::exceptions::PyException);

#[cfg(feature = "pyo3")]
pub(crate) mod python;
