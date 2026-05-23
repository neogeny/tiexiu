// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]

#[cfg(not(feature = "dhat"))]
#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;
#[cfg(not(feature = "dhat"))]
#[global_allocator]
#[cfg(feature = "mimalloc")]
static GLOBAL: MiMalloc = MiMalloc;

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

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
