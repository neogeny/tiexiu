// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::*;
use crate::python::GrammarPy;
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

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub(crate) fn parse_grammar(
    grammar: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
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
pub(crate) fn parse_grammar_to_json(
    grammar: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let value = crate::api::parse_grammar_to_json(grammar, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let py = unsafe { pyo3::Python::assume_attached() };
    pythonize::pythonize(py, &value)
        .map(|obj| obj.into())
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub(crate) fn compile_to_json(
    grammar: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let value = crate::api::compile_to_json(grammar, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let py = unsafe { pyo3::Python::assume_attached() };
    pythonize::pythonize(py, &value)
        .map(|obj| obj.into())
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub(crate) fn compile(grammar: &str, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<GrammarPy> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let grammar = crate::api::compile(grammar, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(GrammarPy::new(grammar))
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub(crate) fn pretty(grammar: &str, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let result = crate::api::grammar_pretty(grammar, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
#[pyo3(signature = (**kwargs))]
pub(crate) fn load_boot_as_json(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let value = crate::api::boot_grammar_to_json(&cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let py = unsafe { pyo3::Python::assume_attached() };
    pythonize::pythonize(py, &value)
        .map(|obj| obj.into())
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

#[pyfunction]
#[pyo3(signature = (**kwargs))]
pub(crate) fn boot_grammar_to_json(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let value = crate::api::boot_grammar_to_json(&cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let py = unsafe { pyo3::Python::assume_attached() };
    pythonize::pythonize(py, &value)
        .map(|obj| obj.into())
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

#[pyfunction]
#[pyo3(signature = (**kwargs))]
pub(crate) fn boot_grammar_pretty(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let result = crate::api::boot_grammar_pretty(&cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
#[pyo3(signature = (grammar, text, **kwargs))]
pub(crate) fn parse(
    grammar: &str,
    text: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
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
pub(crate) fn parse_to_json(
    grammar: &str,
    text: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let value = crate::api::parse_to_json(grammar, text, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let py = unsafe { pyo3::Python::assume_attached() };
    pythonize::pythonize(py, &value)
        .map(|obj| obj.into())
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub(crate) fn parse_grammar_to_json_string(
    grammar: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let value = crate::api::parse_grammar_to_json_string(grammar, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(value)
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub(crate) fn compile_to_json_string(
    grammar: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let value = crate::api::compile_to_json_string(grammar, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(value)
}

#[pyfunction]
#[pyo3(signature = (grammar, text, **kwargs))]
pub(crate) fn parse_to_json_string(
    grammar: &str,
    text: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let value = crate::api::parse_to_json_string(grammar, text, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(value)
}

#[pyfunction]
#[pyo3(signature = (**kwargs))]
pub(crate) fn boot_grammar_to_json_string(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let value = crate::api::boot_grammar_to_json_string(&cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(value)
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub(crate) fn grammar_pretty(
    grammar: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let result = crate::api::grammar_pretty(grammar, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
#[pyo3(signature = (**kwargs))]
pub(crate) fn boot_grammar(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<GrammarPy> {
    let _ = kwargs;
    let grammar = crate::api::boot_grammar()
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(GrammarPy::new(grammar))
}

#[pyfunction]
#[pyo3(signature = (parser, text, **kwargs))]
pub(crate) fn parse_input(
    parser: &GrammarPy,
    text: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let tree = crate::api::parse_input(parser.grammar(), text, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    super::tree::tree_to_py(tree)
}

#[pyfunction]
#[pyo3(signature = (parser, text, **kwargs))]
pub(crate) fn parse_input_to_json(
    parser: &GrammarPy,
    text: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let value = crate::api::parse_input_to_json(parser.grammar(), text, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let py = unsafe { pyo3::Python::assume_attached() };
    pythonize::pythonize(py, &value)
        .map(|obj| obj.into())
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

#[pyfunction]
#[pyo3(signature = (parser, text, **kwargs))]
pub(crate) fn parse_input_to_json_string(
    parser: &GrammarPy,
    text: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let value = crate::api::parse_input_to_json_string(parser.grammar(), text, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(value)
}
