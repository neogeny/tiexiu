// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(dead_code)]

pub mod api;
pub mod cfg;
pub mod engine;
pub mod error;
pub mod input;
pub mod json;
pub mod peg;
pub(crate) mod tools;
pub mod trees;
pub(crate) mod util;

pub use api::*;
#[allow(unused_imports)]
pub use error::{Error, Result};

#[cfg(feature = "pyo3")]
pub(crate) mod python;

#[cfg(feature = "pyo3")]
#[allow(dead_code)]
use pyo3::prelude::*;

#[cfg(feature = "pyo3")]
#[pymodule]
mod tiexiu {
    use super::python::api;
    use super::python::grammar::GrammarPy;
    use super::python::ooapi::TieXiuPy;
    use pyo3::prelude::*;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(api::parse_grammar, m)?)?;
        m.add_function(wrap_pyfunction!(api::parse_grammar_to_json, m)?)?;
        m.add_function(wrap_pyfunction!(api::parse_grammar_to_json_string, m)?)?;
        m.add_function(wrap_pyfunction!(api::compile_to_json, m)?)?;
        m.add_function(wrap_pyfunction!(api::compile_to_json_string, m)?)?;
        m.add_function(wrap_pyfunction!(api::compile, m)?)?;
        m.add_function(wrap_pyfunction!(api::pretty, m)?)?;
        m.add_function(wrap_pyfunction!(api::grammar_pretty, m)?)?;
        m.add_function(wrap_pyfunction!(api::load_boot_as_json, m)?)?;
        m.add_function(wrap_pyfunction!(api::boot_grammar_to_json, m)?)?;
        m.add_function(wrap_pyfunction!(api::boot_grammar_to_json_string, m)?)?;
        m.add_function(wrap_pyfunction!(api::boot_grammar_pretty, m)?)?;
        m.add_function(wrap_pyfunction!(api::boot_grammar, m)?)?;
        m.add_function(wrap_pyfunction!(api::parse_input, m)?)?;
        m.add_function(wrap_pyfunction!(api::parse_input_to_json, m)?)?;
        m.add_function(wrap_pyfunction!(api::parse_input_to_json_string, m)?)?;
        m.add_function(wrap_pyfunction!(api::parse, m)?)?;
        m.add_function(wrap_pyfunction!(api::parse_to_json, m)?)?;
        m.add_function(wrap_pyfunction!(api::parse_to_json_string, m)?)?;

        m.add_class::<TieXiuPy>()?;
        m.add_class::<GrammarPy>()?;

        m.add_function(wrap_pyfunction!(pegapi, m)?)?;

        Ok(())
    }

    #[pyfunction]
    fn pegapi() -> TieXiuPy {
        TieXiuPy(crate::api::ooapi::TieXiu::new(&[]))
    }
}
