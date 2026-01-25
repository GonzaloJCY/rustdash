
use pyo3::prelude::*;
use pyo3::types::{PyList, PyString};

pub fn apply_string_transform<F>(py_input: Bound<'_, PyAny>, transform: F) -> PyResult<PyObject>
where
    F: Fn(&str) -> String,
{
    let py = py_input.py();

    // List case
    if let Ok(strings) = py_input.extract::<Vec<String>>() {
        let result: Vec<String> = strings.iter().map(|s| transform(s)).collect();
        return Ok(PyList::new_bound(py, &result).to_object(py));
    }

    // Single string case
    if let Ok(single_str) = py_input.extract::<String>() {
        let result = transform(&single_str);
        return Ok(PyString::new_bound(py, &result).to_object(py));
    }

    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
        "Expected str or list of str",
    ))
}
