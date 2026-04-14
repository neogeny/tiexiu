use pyo3::prelude::*;

#[pyfunction]
pub fn parse_grammar(grammar: &str) -> PyResult<Py<PyAny>> {
    let tree = crate::api::parse_grammar(grammar)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    super::tree::tree_to_py(tree)
}

#[pyfunction]
pub fn parse_grammar_to_json(grammar: &str) -> PyResult<String> {
    let result = crate::api::parse_grammar_as_json(grammar)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
pub fn compile_to_json(grammar: &str) -> PyResult<String> {
    let result = crate::api::compile_to_json(grammar)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
pub fn pretty(grammar: &str) -> PyResult<String> {
    let result = crate::api::pretty(grammar)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
pub fn load_boot_as_json() -> PyResult<String> {
    let result = crate::api::load_boot_as_json()
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
pub fn boot_grammar_as_json() -> PyResult<String> {
    let result = crate::api::boot_grammar_json()
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
pub fn parse(grammar: &str, text: &str) -> PyResult<Py<PyAny>> {
    let tree = crate::api::parse(grammar, text)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    super::tree::tree_to_py(tree)
}

#[pyfunction]
pub fn parse_to_json(grammar: &str, text: &str) -> PyResult<String> {
    let result = crate::api::parse_to_json(grammar, text)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}
