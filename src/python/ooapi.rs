// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::api::fnapi;
use crate::cfg::*;
use crate::json::ToExpJson;
use pyo3::prelude::*;
use pyo3::types::PyDict;

fn pykwargs_to_cfg(kwargs: &Bound<'_, PyDict>) -> Vec<CfgKey> {
    let mut cfg: Vec<CfgKey> = Vec::new();
    for (key, value) in kwargs.iter() {
        let key_str: String = key.extract().unwrap_or_default();
        let value_str = value.str().map(|s| s.to_string()).unwrap_or_default();
        if let Some(opt) = CfgKey::map(&key_str, &value_str) {
            cfg.push(opt);
        }
    }
    cfg
}

#[pyclass]
pub struct TieXiuPy {
    pub cfg: Vec<CfgKey>,
}

#[pymethods]
impl TieXiuPy {
    #[new]
    #[pyo3(signature = (**kwargs))]
    fn new(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        let cfg = if let Some(k) = kwargs {
            pykwargs_to_cfg(k)
        } else {
            Vec::new()
        };
        Ok(Self { cfg })
    }

    fn parse_grammar(&self, grammar: &str) -> PyResult<Py<PyAny>> {
        let tree = fnapi::parse_grammar(grammar, &self.cfg)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        super::tree::tree_to_py(tree)
    }

    fn parse_grammar_to_json(&self, grammar: &str) -> PyResult<String> {
        let result = fnapi::parse_grammar_to_json_string(grammar, &self.cfg)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(result)
    }

    fn compile(&self, grammar: &str) -> PyResult<String> {
        let result = fnapi::compile_to_json_string(grammar, &self.cfg)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(result)
    }

    fn compile_to_json(&self, grammar: &str) -> PyResult<String> {
        let result = fnapi::compile_to_json_string(grammar, &self.cfg)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(result)
    }

    fn load(&self, json: &str) -> PyResult<String> {
        let grammar = fnapi::load(json, &self.cfg)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let result = grammar
            .to_json_string()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(result)
    }

    fn load_tree(&self, json: &str) -> PyResult<Py<PyAny>> {
        let tree = fnapi::load_tree(json, &self.cfg)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        super::tree::tree_to_py(tree)
    }

    fn boot_grammar(&self) -> PyResult<String> {
        let grammar = fnapi::boot_grammar()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let result = grammar
            .to_json_string()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(result)
    }

    fn boot_grammar_to_json(&self) -> PyResult<String> {
        let result = fnapi::boot_grammar_to_json_string(&self.cfg)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(result)
    }

    fn boot_grammar_pretty(&self) -> PyResult<String> {
        let result = fnapi::boot_grammar_pretty(&self.cfg)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(result)
    }
}
