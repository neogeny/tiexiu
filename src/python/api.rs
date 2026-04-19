// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::*;
use pyo3::prelude::*;
use pyo3::types::PyDict;

fn pykwargs_to_cfg(kwargs: &Bound<'_, PyDict>) -> Vec<Cfg> {
    let mut cfg: Vec<Cfg> = Vec::new();
    for (key, value) in kwargs.iter() {
        let key_str: String = key.extract().unwrap_or_default();
        let value_str = value.str().map(|s| s.to_string()).unwrap_or_default();
        if let Some(opt) = Cfg::map(&key_str, &value_str) {
            cfg.push(opt);
        }
    }
    cfg
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub fn parse_grammar(grammar: &str, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let tree = crate::api::parse_grammar(grammar, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    super::tree::tree_to_py(tree)
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub fn parse_grammar_to_json(
    grammar: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let result = crate::api::parse_grammar_as_json(grammar, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub fn compile_to_json(grammar: &str, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let result = crate::api::compile_to_json(grammar, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub fn pretty(grammar: &str, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let result = crate::api::pretty(grammar, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
#[pyo3(signature = (**kwargs))]
pub fn load_boot_as_json(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let result = crate::api::load_boot_as_json(&cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
#[pyo3(signature = (**kwargs))]
pub fn boot_grammar_as_json(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let result = crate::api::boot_grammar_json(&cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
#[pyo3(signature = (grammar, text, **kwargs))]
pub fn parse(grammar: &str, text: &str, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let tree = crate::api::parse(grammar, text, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    super::tree::tree_to_py(tree)
}

#[pyfunction]
#[pyo3(signature = (grammar, text, **kwargs))]
pub fn parse_to_json(
    grammar: &str,
    text: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let result = crate::api::parse_to_json(grammar, text, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}
