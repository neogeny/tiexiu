// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Tree;
use crate::trees::KeyValue;
use pyo3::BoundObject;
use pyo3::IntoPyObject;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString};

fn to_python(tree: &Tree, py: Python<'_>) -> PyResult<Py<PyAny>> {
    match tree {
        Tree::Bool(b) => Ok((*b).into_pyobject(py)?.into_any().unbind()),
        Tree::Number(n) => Ok((*n).into_pyobject(py)?.into_any().unbind()),
        Tree::Text(s) => Ok(PyString::new(py, s).into()),
        Tree::Seq(items) => {
            let py_items: Vec<Py<PyAny>> = items
                .iter()
                .map(|item| to_python(item, py))
                .collect::<PyResult<_>>()?;
            let py_list = PyList::new(py, py_items)?;
            Ok(py_list.into())
        }
        Tree::Array(items) => {
            let py_items: Vec<Py<PyAny>> = items
                .iter()
                .map(|item| to_python(item, py))
                .collect::<PyResult<_>>()?;
            let py_seq = PyList::new(py, py_items)?;
            Ok(py_seq.into())
        }
        Tree::Object(m) => {
            let dict = PyDict::new(py);
            for (k, v) in m.iter() {
                dict.set_item(k.as_str(), to_python(v, py)?)?;
            }
            Ok(dict.into())
        }
        Tree::Node { typename, tree } => {
            let name: &str = typename;
            let dict = PyDict::new(py);
            dict.set_item("__class__", PyString::new(py, name))?;
            if let Tree::Object(m) = tree.as_ref() {
                for (k, v) in m.iter() {
                    dict.set_item(k.as_str(), to_python(v, py)?)?;
                }
            } else {
                dict.set_item("tree", to_python(tree, py)?)?;
            }
            Ok(dict.into())
        }
        Tree::Nil => Ok(py.None()),
        Tree::Named(kv) => {
            let KeyValue(name, tree) = kv;
            let dict = PyDict::new(py);
            dict.set_item(name.as_str(), to_python(tree, py)?)?;
            Ok(dict.into())
        }
        Tree::NamedAsList(kv) => {
            let KeyValue(name, tree) = kv;
            let dict = PyDict::new(py);
            dict.set_item(name.as_str(), to_python(tree, py)?)?;
            Ok(dict.into())
        }
        Tree::Override(t) | Tree::OverrideAsList(t) => {
            let dict = PyDict::new(py);
            dict.set_item("@", to_python(t, py)?)?;
            Ok(dict.into())
        }
        Tree::Bottom => Ok(py.None()),
    }
}

/// Converts a TieXiu `Tree` into a Python object (dict, list, str, or None).
pub fn tree_to_py(tree: Tree) -> PyResult<Py<PyAny>> {
    let py = unsafe { Python::assume_attached() };
    to_python(&tree, py)
}
