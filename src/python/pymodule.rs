// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::grammar::GrammarPy;
use super::pyfnapi;
use super::pyooapi::TieXiuPy;
use crate::ParseError;
use pyo3::prelude::*;

/// Python module initializer: registers `_tiexiu` with all functions and classes.
#[pymodule(name = "_tiexiu")]
pub fn tiexiu(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("ParseError", m.py().get_type::<ParseError>())?;
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

    Ok(())
}
