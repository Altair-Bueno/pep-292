use std::collections::HashMap;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
pub struct Template {
    template: pep_292::Template<String>,
}

#[pymethods]
impl Template {
    #[new]
    pub fn __new__(string: String) -> Template {
        Self {
            template: pep_292::Template::new(string),
        }
    }

    #[pyo3(signature = (**kws))]
    pub fn substitute(&self, kws: Option<HashMap<String, String>>) -> PyResult<String> {
        let map = kws.unwrap_or_default();
        self.template
            .substitute(&map)
            .map_err(|err| PyValueError::new_err(format!("{err}")))
    }

    #[pyo3(signature = (**kws))]
    pub fn safe_substitute(&self, kws: Option<HashMap<String, String>>) -> PyResult<String> {
        let map = kws.unwrap_or_default();
        self.template
            .safe_substitute(&map)
            .map_err(|err| PyValueError::new_err(format!("{err}")))
    }
}
