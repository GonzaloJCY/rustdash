use crate::bindings::strings_helpers::apply_string_transform;
use crate::core::strings::to_parse;
use crate::core::strings::{
    StringMode, _capitalize, _lower_case, _trim, _trim_end, _trim_start, _upper_case, _words,
};
use pyo3::prelude::*;

/// Python wrapper for camel_case function.
/// Accepts either a single string or a list of strings.
#[pyfunction]
pub fn camel_case(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    apply_string_transform(py_input, |s| to_parse(s, &StringMode::CamelCase))
}

#[pyfunction]
pub fn snake_case(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    apply_string_transform(py_input, |s| to_parse(s, &StringMode::SnakeCase))
}

#[pyfunction]
pub fn kebab_case(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    apply_string_transform(py_input, |s| to_parse(s, &StringMode::KebabCase))
}

#[pyfunction]
pub fn pascal_case(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    apply_string_transform(py_input, |s| to_parse(s, &StringMode::PascalCase))
}

#[pyfunction]
pub fn capitalize(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    apply_string_transform(py_input, |s| _capitalize(s))
}

#[pyfunction]
pub fn upper_case(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    apply_string_transform(py_input, |s| _upper_case(s))
}

#[pyfunction]
pub fn lower_case(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    apply_string_transform(py_input, |s| _lower_case(s))
}

#[pyfunction]
pub fn trim(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    apply_string_transform(py_input, |s| _trim(s))
}

#[pyfunction]
pub fn trim_start(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    apply_string_transform(py_input, |s| _trim_start(s))
}

#[pyfunction]
pub fn trim_end(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    apply_string_transform(py_input, |s| _trim_end(s))
}

#[pyfunction]
pub fn words(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    apply_string_transform(py_input, |s| _words(s).join(" "))
}
