use json::JsonValue;
use pyo3::prelude::*;
// These trait imports make methods like .append() and .set_item() visible
use pyo3::IntoPyObjectExt;
use pyo3::types::{PyDict, PyDictMethods, PyList, PyListMethods};

/// Converts a `json::JsonValue` into the equivalent Python object.
pub fn pythonize(py: Python<'_>, value: &JsonValue) -> PyResult<Py<PyAny>> {
    match value {
        JsonValue::Null => Ok(py.None()),
        JsonValue::Boolean(b) => {
            // Converts bool -> Py<PyAny>
            Ok(b.into_py_any(py)?)
        }
        JsonValue::Number(n) => {
            if let Some(i) = n.as_fixed_point_i64(0) {
                Ok(i.into_py_any(py)?)
            } else if let Some(u) = n.as_fixed_point_u64(0) {
                Ok(u.into_py_any(py)?)
            } else {
                let f: f64 = (*n).into();
                Ok(f.into_py_any(py)?)
            }
        }
        JsonValue::String(s) => Ok(s.into_py_any(py)?),
        JsonValue::Short(s) => Ok(s.as_str().into_py_any(py)?),
        JsonValue::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr.iter() {
                let py_item = pythonize(py, item)?;
                list.append(py_item)?;
            }
            // In 0.28, we cast to generic 'Any' then 'unbind' to return Py<PyAny>
            Ok(list.into_any().unbind())
        }
        JsonValue::Object(obj) => {
            let dict = PyDict::new(py);
            for (key, val) in obj.iter() {
                let py_val = pythonize(py, val)?;
                dict.set_item(key, py_val)?;
            }
            Ok(dict.into_any().unbind())
        }
    }
}
