// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::*;
use crate::peg::Grammar;
use crate::peg::pretty::PrettyPrint;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass]
pub struct GrammarPy(crate::peg::Grammar);

impl GrammarPy {
    pub fn new(grammar: Grammar) -> Self {
        Self(grammar)
    }

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
    fn parse_input(&self, text: &str, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Py<PyAny>> {
        let cfg: Vec<CfgKey> = if let Some(k) = kwargs {
            let mut cfg: Vec<CfgKey> = Vec::new();
            for (key, value) in k.iter() {
                let key_str: String = key.extract().unwrap_or_default();
                let value_str = value.str().map(|s| s.to_string()).unwrap_or_default();
                if let Some(opt) = CfgKey::map(&key_str, &value_str) {
                    cfg.push(opt);
                }
            }
            cfg
        } else {
            Vec::new()
        };
        let tree = crate::api::parse_input(&self.0, text, &cfg)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        crate::python::tree::tree_to_py(tree)
    }
}
