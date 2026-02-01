use pyo3::prelude::*;
use serde_json::Value;

/// Depythonize a Python object into `Vec<Value>`.
pub(crate) fn extract_vec(py_input: &Bound<'_, PyAny>) -> PyResult<Vec<Value>> {
    pythonize::depythonize(py_input)
        .map_err(|e| pyo3::exceptions::PyTypeError::new_err(format!("Expected a list: {e}")))
}

/// Convert a `serde_json::Value` back to a Python object.
pub(crate) fn value_to_py(py: Python<'_>, val: &Value) -> PyResult<PyObject> {
    Ok(pythonize::pythonize(py, val)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?
        .unbind())
}

/// Convert a Python object to a `serde_json::Value`.
pub(crate) fn py_to_value(obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    pythonize::depythonize(obj)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}
