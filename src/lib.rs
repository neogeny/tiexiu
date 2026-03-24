#![allow(dead_code)]
use pyo3::prelude::*;

pub mod input;
pub mod model;
pud mod grammars;

#[pymodule]
mod _tiexiu {
    use pyo3::prelude::*;

    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }
}
