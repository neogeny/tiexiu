// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod contexts;
pub mod grammars;
pub mod input;
pub mod json;

#[allow(dead_code)]
use pyo3::prelude::*;

#[pymodule]
mod tiexiu {
    use pyo3::prelude::*;

    fn _tiexiu(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(compile, m)?)?;
        m.add_function(wrap_pyfunction!(parse, m)?)?;
        Ok(())
    }

    #[pyfunction]
    fn compile(_grammar: String) -> PyResult<String> {
        Ok("model".to_string())
    }

    #[pyfunction]
    fn parse(_grammar: String, _text: String) -> PyResult<String> {
        Ok("output".to_string())
    }
}
