// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::ParseError;
use crate::cfg::*;
use crate::peg::Grammar;
use crate::peg::pretty::PrettyPrint;
use pyo3::prelude::*;
use pyo3::types::PyDict;

/// A compiled grammar exposed to Python.
#[pyclass(module = "_tiexiu", unsendable)]
pub struct GrammarPy(crate::peg::Grammar);

impl GrammarPy {
    /// Wraps a compiled `Grammar` for Python use.
    pub fn new(grammar: Grammar) -> Self {
        Self(grammar)
    }

    /// Returns a reference to the inner `Grammar`.
    pub fn grammar(&self) -> &Grammar {
        &self.0
    }
}

#[pymethods]
impl GrammarPy {
    fn pretty(&self) -> String {
        self.0.pretty_print()
    }

    #[pyo3(signature = (text, **kwargs))]
    fn parse(&self, text: &str, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Py<PyAny>> {
        self.parse_input(text, kwargs)
    }

    #[pyo3(signature = (text, **kwargs))]
    fn parse_input(&self, text: &str, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Py<PyAny>> {
        let cfg: Vec<CfgKey> = if let Some(k) = kwargs {
            let mut cfg: Vec<CfgKey> = Vec::new();
            for (key, value) in k.iter() {
                let key_str: String = key.extract().unwrap_or_default();
                let value_str = value.str().map(|s| s.to_string()).unwrap_or_default();
                if let Some(opt) = CfgKey::map(&key_str, &value_str) {
                    cfg.push(opt);
                } else {
                    return Err(pyo3::exceptions::PyValueError::new_err(format!(
                        "unknown configuration option: {}",
                        key_str
                    )));
                }
            }
            cfg
        } else {
            Vec::new()
        };
        let tree = crate::api::parse_input(&self.0, text, &cfg)
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        crate::python::tree::tree_to_py(tree)
    }
}
