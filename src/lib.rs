use pyo3::prelude::*;

pub mod model;
mod input;

#[pymodule]
mod _tiexiu {
    use pyo3::prelude::*;

    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }
}
