// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Tree;
use crate::trees::KeyValue;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString};

fn set_item(dict: &Bound<'_, PyDict>, key: &str, val: Py<PyAny>) -> PyResult<()> {
    dict.call_method1("__setitem__", (key, val))?;
    Ok(())
}

fn to_python(tree: &Tree, py: Python<'_>) -> PyResult<Py<PyAny>> {
    match tree {
        Tree::Text(s) => Ok(PyString::new(py, s).into()),
        Tree::Seq(items) => {
            let py_items: Vec<Py<PyAny>> = items
                .iter()
                .map(|item| to_python(item, py))
                .collect::<PyResult<_>>()?;
            let py_list = PyList::new(py, py_items)?;
            Ok(py_list.into())
        }
        Tree::List(items) => {
            let py_items: Vec<Py<PyAny>> = items
                .iter()
                .map(|item| to_python(item, py))
                .collect::<PyResult<_>>()?;
            let py_seq = PyList::new(py, py_items)?;
            Ok(py_seq.into())
        }
        Tree::Map(m) => {
            let dict = PyDict::new(py);
            for (k, v) in m.iter() {
                let val = to_python(v, py)?;
                set_item(&dict, k.as_str(), val)?;
            }
            Ok(dict.into())
        }
        Tree::Node { typename, tree } => {
            let name: &str = typename;
            let dict = PyDict::new(py);
            set_item(&dict, "__class__", PyString::new(py, name).into())?;
            if let Tree::Map(m) = tree.as_ref() {
                for (k, v) in m.iter() {
                    let val = to_python(v, py)?;
                    set_item(&dict, k.as_str(), val)?;
                }
            } else {
                let inner = to_python(tree, py)?;
                set_item(&dict, "tree", inner)?;
            }
            Ok(dict.into())
        }
        Tree::Null => Ok(py.None()),
        Tree::Named(kv) => {
            let KeyValue(name, tree) = kv;
            let dict = PyDict::new(py);
            let val = to_python(tree, py)?;
            set_item(&dict, name.as_str(), val)?;
            Ok(dict.into())
        }
        Tree::NamedAsList(kv) => {
            let KeyValue(name, tree) = kv;
            let dict = PyDict::new(py);
            let val = to_python(tree, py)?;
            set_item(&dict, name.as_str(), val)?;
            Ok(dict.into())
        }
        Tree::Override(t) | Tree::OverrideAsList(t) => {
            let dict = PyDict::new(py);
            let val = to_python(t, py)?;
            set_item(&dict, "@", val)?;
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
