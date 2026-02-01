use super::helpers::{extract_vec, py_to_value, value_to_py};
use crate::core::objects::{
    get as core_get, get_all as core_get_all, has as core_has, has_all as core_has_all,
    keys as core_keys, merge as core_merge, omit as core_omit, pick as core_pick,
    values as core_values,
};
use pyo3::prelude::*;
use serde_json::Value;

// ─── Python wrappers ─────────────────────────────────────────────────────────
/// Python wrapper for get_all function.
#[pyfunction]
pub fn get(py_input: Bound<'_, PyAny>, path: &str, default: PyObject) -> PyResult<PyObject> {
    let py = py_input.py();
    let obj = py_to_value(&py_input)?;
    let default = py_to_value(&default.bind(py))?;
    let result = core_get(&obj, path, default);
    value_to_py(py, &Value::from(result))
}

/// Python wrapper for has_all function.
#[pyfunction]
pub fn has(py_input: Bound<'_, PyAny>, path: &str) -> PyResult<bool> {
    let obj = py_to_value(&py_input)?;
    Ok(core_has(&obj, path))
}
/// Python wrapper for get_all function.
#[pyfunction]
pub fn get_all(py_input: Bound<'_, PyAny>, path: &str) -> PyResult<PyObject> {
    let py = py_input.py();
    let obj = py_to_value(&py_input)?;
    let result = core_get_all(&obj, path);
    value_to_py(py, &Value::Array(result))
}

/// Python wrapper for has_all function.
#[pyfunction]
pub fn has_all(py_input: Bound<'_, PyAny>, path: &str) -> PyResult<bool> {
    let obj = py_to_value(&py_input)?;
    Ok(core_has_all(&obj, path))
}

/// Python wrapper for pick function.
#[pyfunction]
pub fn pick(py_input: Bound<'_, PyAny>, py_keys: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let obj = py_to_value(&py_input)?;
    let keys: Vec<String> = py_keys.extract()?;
    let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
    let result = core_pick(&obj, &key_refs);
    value_to_py(py, &result)
}

/// Python wrapper for omit function.
#[pyfunction]
pub fn omit(py_input: Bound<'_, PyAny>, py_keys: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let obj = py_to_value(&py_input)?;
    let keys: Vec<String> = py_keys.extract()?;
    let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
    let result = core_omit(&obj, &key_refs);
    value_to_py(py, &result)
}

/// Python wrapper for merge function.
#[pyfunction]
pub fn merge(py_objects: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_objects.py();
    let objects: Vec<Value> = extract_vec(&py_objects)?;
    let result = core_merge(&objects);
    value_to_py(py, &result)
}

/// Python wrapper for keys function.
#[pyfunction]
pub fn keys(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let obj = py_to_value(&py_input)?;
    let result: Vec<String> = core_keys(&obj);
    value_to_py(
        py,
        &Value::Array(result.into_iter().map(Value::String).collect()),
    )
}
/// Python wrapper for values function.
#[pyfunction]
pub fn values(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let obj = py_to_value(&py_input)?;
    let result = core_values(&obj);
    value_to_py(py, &Value::Array(result))
}
