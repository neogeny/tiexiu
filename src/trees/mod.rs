/// Display formatting for Tree types.
pub mod fmt;
/// TreeMap type for named key-value tree storage.
/// Shortcut constructor functions for trees.
pub mod short;

/// Core Tree enum and supporting types.
pub mod tree;
pub use tree::*;

/// Tree builder helper methods.
pub mod build;
pub use build::*;

pub mod named;
pub use named::*;

/// Folds trait for tree transformation.
pub mod fold;
pub use fold::*;

/// Tree error types.
pub mod error;
pub mod path;

pub use error::Error;
