/// Cursor trait for input sources.
pub mod cursor;
/// Input error types.
pub mod error;
/// Parse memento for error reporting.
pub mod memento;
/// String-backed cursor implementation.
pub mod strcursor;
/// Tokenizing pattern management.
pub mod tokenizing;

/// Trait for input sources that can advance, peek, and match.
pub use cursor::Cursor;
pub(crate) use error::Error;
/// A cursor over a string input.
pub use strcursor::StrCursor;
