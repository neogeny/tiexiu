// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::ParseError;
use crate::python::GrammarPy;
use crate::python::pyooapi::TieXiuPy;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use super::util::{pykwargs_to_cfg, pythonize_json_value};

#[pyfunction]
pub(crate) fn pegapi(py: Python<'_>) -> PyResult<Py<PyAny>> {
    let tx_py = TieXiuPy(crate::api::pegapi());

    let bound_obj = Bound::new(py, tx_py)?;

    Ok(bound_obj.into_any().unbind())
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub(crate) fn parse_grammar(
    py: Python<'_>,
    grammar: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)?
    } else {
        Vec::new()
    };
    let tree =
        crate::api::parse_grammar(grammar, &cfg).map_err(|e| ParseError::new_err(e.to_string()))?;
    super::tree::tree_to_py(py, tree)
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub(crate) fn parse_grammar_to_json(
    py: Python<'_>,
    grammar: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)?
    } else {
        Vec::new()
    };
    let tree =
        crate::api::parse_grammar(grammar, &cfg).map_err(|e| ParseError::new_err(e.to_string()))?;
    pythonize_json_value(py, tree.to_json())
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub(crate) fn compile_to_json(
    py: Python<'_>,
    grammar: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)?
    } else {
        Vec::new()
    };
    let grammar =
        crate::api::compile(grammar, &cfg).map_err(|e| ParseError::new_err(e.to_string()))?;
    pythonize_json_value(py, grammar.to_json())
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub(crate) fn compile(grammar: &str, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<GrammarPy> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)?
    } else {
        Vec::new()
    };
    let grammar =
        crate::api::compile(grammar, &cfg).map_err(|e| ParseError::new_err(e.to_string()))?;
    Ok(GrammarPy::new(grammar))
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub(crate) fn pretty(grammar: &str, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)?
    } else {
        Vec::new()
    };
    let result = crate::api::grammar_pretty(grammar, &cfg)
        .map_err(|e| ParseError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
#[pyo3(signature = (**kwargs))]
pub(crate) fn load_boot_as_json(
    py: Python<'_>,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let _ = kwargs;
    let grammar = crate::api::boot_grammar().map_err(|e| ParseError::new_err(e.to_string()))?;
    pythonize_json_value(py, grammar.to_json())
}

#[pyfunction]
#[pyo3(signature = (**kwargs))]
pub(crate) fn boot_grammar_to_json(
    py: Python<'_>,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let _ = kwargs;
    let grammar = crate::api::boot_grammar().map_err(|e| ParseError::new_err(e.to_string()))?;
    pythonize_json_value(py, grammar.to_json())
}

#[pyfunction]
#[pyo3(signature = (**kwargs))]
pub(crate) fn boot_grammar_pretty(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<String> {
    let _ = kwargs;
    let result =
        crate::api::boot_grammar_pretty().map_err(|e| ParseError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
#[pyo3(signature = (grammar, text, **kwargs))]
pub(crate) fn parse(
    py: Python<'_>,
    grammar: &str,
    text: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)?
    } else {
        Vec::new()
    };
    let tree =
        crate::api::parse(grammar, text, &cfg).map_err(|e| ParseError::new_err(e.to_string()))?;
    super::tree::tree_to_py(py, tree)
}

#[pyfunction]
#[pyo3(signature = (grammar, text, **kwargs))]
pub(crate) fn parse_to_json(
    py: Python<'_>,
    grammar: &str,
    text: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)?
    } else {
        Vec::new()
    };
    let tree =
        crate::api::parse(grammar, text, &cfg).map_err(|e| ParseError::new_err(e.to_string()))?;
    pythonize_json_value(py, tree.to_json())
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub(crate) fn parse_grammar_to_json_string(
    grammar: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)?
    } else {
        Vec::new()
    };
    let tree =
        crate::api::parse_grammar(grammar, &cfg).map_err(|e| ParseError::new_err(e.to_string()))?;
    Ok(tree.to_json_string())
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub(crate) fn compile_to_json_string(
    grammar: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)?
    } else {
        Vec::new()
    };
    let grammar =
        crate::api::compile(grammar, &cfg).map_err(|e| ParseError::new_err(e.to_string()))?;
    grammar
        .to_json_string()
        .map_err(|e| ParseError::new_err(e.to_string()))
}

#[pyfunction]
#[pyo3(signature = (grammar, text, **kwargs))]
pub(crate) fn parse_to_json_string(
    grammar: &str,
    text: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)?
    } else {
        Vec::new()
    };
    let tree =
        crate::api::parse(grammar, text, &cfg).map_err(|e| ParseError::new_err(e.to_string()))?;
    Ok(tree.to_json_string())
}

#[pyfunction]
#[pyo3(signature = (**kwargs))]
pub(crate) fn boot_grammar_to_json_string(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<String> {
    let _ = kwargs;
    let grammar = crate::api::boot_grammar().map_err(|e| ParseError::new_err(e.to_string()))?;
    grammar
        .to_json_string()
        .map_err(|e| ParseError::new_err(e.to_string()))
}

#[pyfunction]
#[pyo3(signature = (grammar, **kwargs))]
pub(crate) fn grammar_pretty(
    grammar: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)?
    } else {
        Vec::new()
    };
    let result = crate::api::grammar_pretty(grammar, &cfg)
        .map_err(|e| ParseError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
#[pyo3(signature = (**kwargs))]
pub(crate) fn boot_grammar(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<GrammarPy> {
    let _ = kwargs;
    let grammar = crate::api::boot_grammar().map_err(|e| ParseError::new_err(e.to_string()))?;
    Ok(GrammarPy::new(grammar))
}

#[pyfunction]
#[pyo3(signature = (parser, text, **kwargs))]
pub(crate) fn parse_input(
    py: Python<'_>,
    parser: &GrammarPy,
    text: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)?
    } else {
        Vec::new()
    };
    let tree = crate::api::parse_input(parser.grammar(), text, &cfg)
        .map_err(|e| ParseError::new_err(e.to_string()))?;
    super::tree::tree_to_py(py, tree)
}

#[pyfunction]
#[pyo3(signature = (parser, text, **kwargs))]
pub(crate) fn parse_input_to_json(
    py: Python<'_>,
    parser: &GrammarPy,
    text: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)?
    } else {
        Vec::new()
    };
    let tree = crate::api::parse_input(parser.grammar(), text, &cfg)
        .map_err(|e| ParseError::new_err(e.to_string()))?;
    pythonize_json_value(py, tree.to_json())
}

#[pyfunction]
#[pyo3(signature = (parser, text, **kwargs))]
pub(crate) fn parse_input_to_json_string(
    parser: &GrammarPy,
    text: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    let cfg = if let Some(k) = kwargs {
        pykwargs_to_cfg(k)?
    } else {
        Vec::new()
    };
    let tree = crate::api::parse_input(parser.grammar(), text, &cfg)
        .map_err(|e| ParseError::new_err(e.to_string()))?;
    Ok(tree.to_json_string())
}
