mod template;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn pep_292_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<template::Template>()?;
    Ok(())
}
