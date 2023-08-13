mod template;
use pyo3::prelude::*;

#[pymodule]
fn pep_292_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<template::Template>()?;
    Ok(())
}
