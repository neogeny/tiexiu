// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::peg::Grammar;
use pyo3::prelude::*;

#[pyclass]
pub struct GrammarPy(crate::peg::Grammar);

#[pymethods]
impl GrammarPy {
    fn to_json(&self) -> PyResult<String> {
        Ok(self.0.to_json_string()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?)
    }

    fn pretty(&self) -> PyResult<String> {
        Ok(self.0.pretty_print())
    }
}