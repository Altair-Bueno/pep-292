use std::collections::HashMap;

use pep_292::Template as PEP292Template;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
pub struct Template {
    string: String,
}

#[pymethods]
impl Template {
    #[new]
    pub fn __new__(string: String) -> Template {
        Self { string }
    }

    #[pyo3(signature = (**kws))]
    pub fn substitute(&self, kws: Option<HashMap<String, String>>) -> PyResult<String> {
        let map = kws.unwrap_or_default();
        self.string
            .substitute(&map)
            .map_err(|err| PyValueError::new_err(format!("{err}")))
    }

    #[pyo3(signature = (**kws))]
    pub fn safe_substitute(&self, kws: Option<HashMap<String, String>>) -> PyResult<String> {
        let map = kws.unwrap_or_default();
        self.string
            .safe_substitute(&map)
            .map_err(|err| PyValueError::new_err(format!("{err}")))
    }
}
