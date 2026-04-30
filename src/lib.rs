// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]

pub mod api;
pub mod cfg;
pub mod engine;
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
pub(crate) mod python;

#[cfg(feature = "pyo3")]
#[allow(dead_code)]
use pyo3::prelude::*;

#[cfg(feature = "pyo3")]
#[pymodule]
mod tiexiu {
    use super::python::grammar::GrammarPy;
    use super::python::pyfnapi;
    use super::python::pyooapi::TieXiuPy;
    use pyo3::prelude::*;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(pyfnapi::pegapi, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::parse_grammar, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::parse_grammar_to_json, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::parse_grammar_to_json_string, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::compile_to_json, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::compile_to_json_string, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::compile, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::pretty, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::grammar_pretty, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::load_boot_as_json, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::boot_grammar_to_json, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::boot_grammar_to_json_string, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::boot_grammar_pretty, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::boot_grammar, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::parse_input, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::parse_input_to_json, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::parse_input_to_json_string, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::parse, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::parse_to_json, m)?)?;
        m.add_function(wrap_pyfunction!(pyfnapi::parse_to_json_string, m)?)?;

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
