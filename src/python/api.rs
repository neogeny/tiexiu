use pyo3::prelude::*;
use pyo3::types::PyDict;

fn pykwargs_to_cfg(kwargs: &Bound<'_, PyDict>) -> Vec<(&'static str, &'static str)> {
    let mut cfg: Vec<(&'static str, &'static str)> = Vec::new();
    for (key, value) in kwargs.iter() {
        let key_str: String = key.extract().unwrap_or_default();
        let value_str: String = value.extract().unwrap_or_default();
        let key_boxed = Box::leak(key_str.into_boxed_str());
        let value_boxed = Box::leak(value_str.into_boxed_str());
        cfg.push((key_boxed, value_boxed));
    }
    cfg
}

#[pyfunction]
pub fn parse_grammar(grammar: &str, kwargs: Option<Bound<'_, PyDict>>) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(ref k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let tree = crate::api::parse_grammar(grammar, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    super::tree::tree_to_py(tree)
}

#[pyfunction]
pub fn parse_grammar_to_json(grammar: &str, kwargs: Option<Bound<'_, PyDict>>) -> PyResult<String> {
    let cfg = if let Some(ref k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let result = crate::api::parse_grammar_as_json(grammar, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
pub fn compile_to_json(grammar: &str, kwargs: Option<Bound<'_, PyDict>>) -> PyResult<String> {
    let cfg = if let Some(ref k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let result = crate::api::compile_to_json(grammar, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
pub fn pretty(grammar: &str, kwargs: Option<Bound<'_, PyDict>>) -> PyResult<String> {
    let cfg = if let Some(ref k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let result = crate::api::pretty(grammar, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
pub fn load_boot_as_json(kwargs: Option<Bound<'_, PyDict>>) -> PyResult<String> {
    let cfg = if let Some(ref k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let result = crate::api::load_boot_as_json(&cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
pub fn boot_grammar_as_json(kwargs: Option<Bound<'_, PyDict>>) -> PyResult<String> {
    let cfg = if let Some(ref k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let result = crate::api::boot_grammar_json(&cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
pub fn parse(grammar: &str, text: &str, kwargs: Option<Bound<'_, PyDict>>) -> PyResult<Py<PyAny>> {
    let cfg = if let Some(ref k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let tree = crate::api::parse(grammar, text, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    super::tree::tree_to_py(tree)
}

#[pyfunction]
pub fn parse_to_json(
    grammar: &str,
    text: &str,
    kwargs: Option<Bound<'_, PyDict>>,
) -> PyResult<String> {
    let cfg = if let Some(ref k) = kwargs {
        pykwargs_to_cfg(k)
    } else {
        Vec::new()
    };
    let result = crate::api::parse_to_json(grammar, text, &cfg)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}
