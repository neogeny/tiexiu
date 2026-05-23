// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

/// INTERNAL: Captures the location and condition.
/// This is the "raw" macro that doesn't return a Result,
/// just the struct itself.
#[macro_export]
macro_rules! ensure_error {
    ($cond:expr) => {
        $crate::util::ensure::Ensure {
            condition: stringify!($cond),
            file: file!(),
            line: line!(),
            column: column!(),
        }
    };
}

/// PUBLIC: The standard result-based check.
/// It uses ensure_error! to do the heavy lifting.
#[macro_export]
macro_rules! ensure {
    ($cond:expr) => {
        if $cond {
            Ok(())
        } else {
            Err($crate::ensure_error!($cond))
        }
    };
}

/// Captures a failed `ensure!()` condition with source location.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub struct Ensure {
    pub condition: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl std::fmt::Display for Ensure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "!({}) at {}[{}:{}]",
            self.condition, self.file, self.line, self.column
        )
    }
}
