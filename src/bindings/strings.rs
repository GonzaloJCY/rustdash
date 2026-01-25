use pyo3::prelude::*;
use pyo3::types::PyList;
use crate::core::strings::to_parse;
use crate::core::strings::StringMode;




/// Python wrapper for camel_case function.
/// Accepts either a single string or a list of strings.
#[pyfunction]
pub fn camel_case(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();

    // List case
    if let Ok(strings) = py_input.extract::<Vec<String>>() {
        let result = to_parse(strings, StringMode::CamelCase);
        return Ok(PyList::new_bound(py, &result).to_object(py));
    }

    // Single string case
    if let Ok(single_str) = py_input.extract::<String>() {
        let result = to_parse(vec![single_str], StringMode::CamelCase);
        return Ok(result[0].clone().into_py(py));
    }

    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
        "Expected str or list of str"
    ))
}
