// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Generic configuration container.
pub mod cfg;
/// Ensure macro and error type for precondition checks.
pub mod ensure;
/// Utility error types.
pub mod error;
/// Scope-based finalizer (defer/cleanup) utility.
pub mod finally;
/// Generic tree folding traits.
pub mod fold;
/// Fuse type for lifecycle enforcement.
pub mod fuse;
/// ScopeGuard for transactional rollback.
pub mod guard;
/// IndentWriter for pretty-printing with indentation.
pub mod indent;
/// Conversion traits for string types.
pub mod into;
/// Globally-unique Memento identifier.
pub mod memento;
/// Newline/indentation detection utilities.
pub mod newlines;
/// Pattern matching using Python-compatible regex.
pub mod pyre;
/// String sanitization and identifier utilities.
pub mod strtools;
/// TokenStack cons-list for PEG parsing.
pub mod tokenstack;

/// Re-export of Cfg type.
pub use cfg::Cfg;
/// Re-export of error types.
pub use error::*;
/// Re-export of finally function.
pub use finally::finally;
/// Re-export of safe_name and to_snake_case.
pub use strtools::{safe_name, to_snake_case};
