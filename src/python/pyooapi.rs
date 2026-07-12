// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::ParseError;
use crate::api::ooapi::TieXiu;
use pyo3::prelude::*;
use pyo3::types::*;

use super::util::{pykwargs_to_cfg, pythonize_json_value};

fn update_cfg_from_kwargs(tx: &mut TieXiu, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<()> {
    if let Some(k) = kwargs {
        let cfg = pykwargs_to_cfg(k)?;
        tx.update_cfg(&cfg);
    }
    Ok(())
}

/// Python wrapper around the OO `TieXiu` API.
#[pyclass(module = "_tiexiu", unsendable)]
pub struct TieXiuPy(pub TieXiu);

#[pymethods]
impl TieXiuPy {
    #[new]
    #[pyo3(signature = (**kwargs))]
    fn new(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        let cfg = if let Some(k) = kwargs {
            pykwargs_to_cfg(k)?
        } else {
            Vec::new()
        };
        Ok(Self(TieXiu::new(&cfg)))
    }

    fn get(&mut self, grammar: &str) -> Option<String> {
        self.0.get(grammar).map(|_| "cached".to_string())
    }

    fn get_or_compile(&mut self, grammar: &str) -> PyResult<String> {
        let _ = self
            .0
            .get_or_compile(grammar)
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        Ok("compiled".to_string())
    }

    #[pyo3(signature = (grammar, **kwargs))]
    fn parse_grammar(
        &mut self,
        py: Python<'_>,
        grammar: &str,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        update_cfg_from_kwargs(&mut self.0, kwargs)?;
        let tree = self
            .0
            .parse_grammar(grammar)
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        super::tree::tree_to_py(py, tree)
    }

    #[pyo3(signature = (grammar, **kwargs))]
    fn parse_grammar_to_json(
        &mut self,
        py: Python<'_>,
        grammar: &str,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        update_cfg_from_kwargs(&mut self.0, kwargs)?;
        let tree = self
            .0
            .parse_grammar(grammar)
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        pythonize_json_value(py, tree.to_json()).map_err(|e| ParseError::new_err(e.to_string()))
    }

    #[pyo3(signature = (grammar, **kwargs))]
    fn compile(
        &mut self,
        grammar: &str,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<super::grammar::GrammarPy> {
        update_cfg_from_kwargs(&mut self.0, kwargs)?;
        let grammar = self
            .0
            .compile(grammar)
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        Ok(super::grammar::GrammarPy::new(grammar))
    }

    #[pyo3(signature = (grammar, **kwargs))]
    fn compile_to_json(
        &mut self,
        py: Python<'_>,
        grammar: &str,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        update_cfg_from_kwargs(&mut self.0, kwargs)?;
        let grammar = self
            .0
            .compile(grammar)
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        pythonize_json_value(py, grammar.to_json()).map_err(|e| ParseError::new_err(e.to_string()))
    }

    #[pyo3(signature = (json, **kwargs))]
    fn load(
        &mut self,
        py: Python<'_>,
        json: &str,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        update_cfg_from_kwargs(&mut self.0, kwargs)?;
        let grammar = self
            .0
            .load(json)
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        let value = grammar.to_json();
        pythonize_json_value(py, value).map_err(|e| ParseError::new_err(e.to_string()))
    }

    #[pyo3(signature = (json, **kwargs))]
    fn load_tree(
        &mut self,
        py: Python<'_>,
        json: &str,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        update_cfg_from_kwargs(&mut self.0, kwargs)?;
        let tree = self
            .0
            .load_tree(json)
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        super::tree::tree_to_py(py, tree)
    }

    #[pyo3(signature = (**kwargs))]
    fn boot_grammar(
        &mut self,
        py: Python<'_>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        update_cfg_from_kwargs(&mut self.0, kwargs)?;
        let grammar = self
            .0
            .boot_grammar()
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        let value = grammar.to_json();
        pythonize_json_value(py, value).map_err(|e| ParseError::new_err(e.to_string()))
    }

    fn boot_grammar_pretty(&mut self) -> PyResult<String> {
        let result = self
            .0
            .boot_grammar_pretty()
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        Ok(result)
    }

    #[pyo3(signature = (**kwargs))]
    fn boot_grammar_to_json(
        &mut self,
        py: Python<'_>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        update_cfg_from_kwargs(&mut self.0, kwargs)?;
        let grammar = self
            .0
            .boot_grammar()
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        pythonize_json_value(py, grammar.to_json()).map_err(|e| ParseError::new_err(e.to_string()))
    }

    #[pyo3(signature = (**kwargs))]
    fn load_boot(
        &mut self,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<super::grammar::GrammarPy> {
        update_cfg_from_kwargs(&mut self.0, kwargs)?;
        let grammar = self
            .0
            .load_boot()
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        Ok(super::grammar::GrammarPy::new(grammar))
    }

    #[pyo3(signature = (**kwargs))]
    fn load_boot_as_json(
        &mut self,
        py: Python<'_>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        update_cfg_from_kwargs(&mut self.0, kwargs)?;
        let grammar = self
            .0
            .boot_grammar()
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        pythonize_json_value(py, grammar.to_json()).map_err(|e| ParseError::new_err(e.to_string()))
    }

    #[pyo3(signature = (grammar, **kwargs))]
    fn pretty(&mut self, grammar: &str, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<String> {
        update_cfg_from_kwargs(&mut self.0, kwargs)?;
        let result = self
            .0
            .grammar_pretty(grammar)
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        Ok(result)
    }

    #[pyo3(signature = (grammar, text, **kwargs))]
    fn parse(
        &mut self,
        py: Python<'_>,
        grammar: &str,
        text: &str,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        update_cfg_from_kwargs(&mut self.0, kwargs)?;
        let tree = self
            .0
            .parse(grammar, text)
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        super::tree::tree_to_py(py, tree)
    }

    #[pyo3(signature = (grammar, text, **kwargs))]
    fn parse_to_json(
        &mut self,
        py: Python<'_>,
        grammar: &str,
        text: &str,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        update_cfg_from_kwargs(&mut self.0, kwargs)?;
        let tree = self
            .0
            .parse(grammar, text)
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        pythonize_json_value(py, tree.to_json()).map_err(|e| ParseError::new_err(e.to_string()))
    }

    #[pyo3(signature = (grammar, text, **kwargs))]
    fn parse_to_json_string(
        &mut self,
        py: Python<'_>,
        grammar: &str,
        text: &str,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyString>> {
        update_cfg_from_kwargs(&mut self.0, kwargs)?;
        let tree = self
            .0
            .parse(grammar, text)
            .map_err(|e| ParseError::new_err(e.to_string()))?;
        let json = tree.to_json_string();
        Ok(PyString::new(py, &json).into())
    }
}
