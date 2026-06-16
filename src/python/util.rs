use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyDictMethods, PyList, PyListMethods};
use serde_json::Value;

/// Converts a `serde_json::Value` into the equivalent Python object.
pub fn pythonize(py: Python<'_>, value: &Value) -> PyResult<Py<PyAny>> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.into_py_any(py)?),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_py_any(py)?)
            } else if let Some(u) = n.as_u64() {
                Ok(u.into_py_any(py)?)
            } else {
                let f: f64 = n.as_f64().unwrap_or(0.0);
                Ok(f.into_py_any(py)?)
            }
        }
        Value::String(s) => Ok(s.into_py_any(py)?),
        Value::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr.iter() {
                let py_item = pythonize(py, item)?;
                list.append(py_item)?;
            }
            Ok(list.into_any().unbind())
        }
        Value::Object(obj) => {
            let dict = PyDict::new(py);
            for (key, val) in obj.iter() {
                let py_val = pythonize(py, val)?;
                dict.set_item(key, py_val)?;
            }
            Ok(dict.into_any().unbind())
        }
    }
}
