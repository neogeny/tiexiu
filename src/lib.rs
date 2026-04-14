// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod api;
pub mod error;
pub mod input;
pub mod json;
pub mod peg;
pub mod state;
pub mod trees;
pub mod ui;
pub mod util;

pub use api::{compile, parse};
pub use error::{Error, Result};

#[cfg(feature = "pyo3")]
pub mod python;

#[cfg(feature = "pyo3")]
#[allow(dead_code)]
use pyo3::prelude::*;

#[cfg(feature = "pyo3")]
#[pymodule]
mod tiexiu {
    use super::python::api;
    use pyo3::prelude::*;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(api::parse_grammar, m)?)?;
        m.add_function(wrap_pyfunction!(api::parse_grammar_to_json, m)?)?;
        m.add_function(wrap_pyfunction!(api::compile_to_json, m)?)?;
        m.add_function(wrap_pyfunction!(api::pretty, m)?)?;
        m.add_function(wrap_pyfunction!(api::load_boot_as_json, m)?)?;
        m.add_function(wrap_pyfunction!(api::boot_grammar_as_json, m)?)?;
        m.add_function(wrap_pyfunction!(api::parse, m)?)?;
        m.add_function(wrap_pyfunction!(api::parse_to_json, m)?)?;
        Ok(())
    }
}
