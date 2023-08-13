mod template;
use pyo3::prelude::*;

#[pymodule]
fn pep_292(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<template::Template>()?;
    Ok(())
}
