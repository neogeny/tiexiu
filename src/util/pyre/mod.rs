pub mod error;
// #[allow(dead_code)]
// mod fancy;
pub mod pattern;
#[cfg(feature = "pcre2")]
mod pcre2;
pub mod traits;

pub use error::*;
pub use pattern::*;

// #[cfg(not(feature = "pcre2"))]
// pub use fancy::*;
#[cfg(feature = "pcre2")]
pub use pcre2::*;
