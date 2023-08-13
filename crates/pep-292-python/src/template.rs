use std::collections::HashMap;

use pep_292::Template as PEP292Template;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;

#[pyclass]
pub struct Template {
    string: Py<PyString>,
}

#[pymethods]
impl Template {
    #[new]
    pub fn __new__(string: Py<PyString>) -> Template {
        Self { string }
    }

    #[pyo3(signature = (**kws))]
    pub fn substitute(&self, py: Python<'_>, kws: Option<HashMap<&str, &str>>) -> PyResult<String> {
        let map = kws.unwrap_or_default();
        self.string
            .as_ref(py)
            .to_str()?
            .substitute(&map)
            .map_err(|err| PyValueError::new_err(format!("{err}")))
    }

    #[pyo3(signature = (**kws))]
    pub fn safe_substitute(
        &self,
        py: Python<'_>,
        kws: Option<HashMap<&str, &str>>,
    ) -> PyResult<String> {
        let map = kws.unwrap_or_default();
        self.string
            .as_ref(py)
            .to_str()?
            .safe_substitute(&map)
            .map_err(|err| PyValueError::new_err(format!("{err}")))
    }
}
