use super::helpers::{extract_vec, py_to_value, value_to_py};
use crate::core::arrays::{
    chunk as core_chunk, compact as core_compact, filter as core_filter, find as core_find,
    flatten_deep as core_flatten_deep, group_by as core_group_by,
    intersection as core_intersection, map as core_map, reduce as core_reduce,
    sort_by as core_sort_by, unique as core_unique, zip as core_zip,
};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::Value;

// ─── Helper types ────────────────────────────────────────────────────────────

/// Wrapper to make serde_json::Value implement Hash (needed for unique / group_by).
/// Hashing is done via the canonical JSON string representation.
#[derive(Clone, PartialEq, Eq)]
struct HashableValue(Value);

impl std::hash::Hash for HashableValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        serde_json::to_string(&self.0)
            .unwrap_or_default()
            .hash(state);
    }
}

/// Wrapper to make serde_json::Value implement Ord (needed for sort_by).
/// Numbers are compared numerically; strings lexicographically; other types are
/// treated as equal.
#[derive(Clone, PartialEq, Eq)]
struct OrdValue(Value);

impl PartialOrd for OrdValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrdValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (&self.0, &other.0) {
            (Value::Number(a), Value::Number(b)) => {
                let fa = a.as_f64().unwrap_or(0.0);
                let fb = b.as_f64().unwrap_or(0.0);
                fa.partial_cmp(&fb).unwrap_or(std::cmp::Ordering::Equal)
            }
            (Value::String(a), Value::String(b)) => a.cmp(b),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

// ─── Python wrappers ─────────────────────────────────────────────────────────

/// Python wrapper for chunk function.
#[pyfunction]
pub fn chunk(py_input: Bound<'_, PyAny>, size: usize) -> PyResult<PyObject> {
    let py = py_input.py();
    let vec = extract_vec(&py_input)?;
    let result = core_chunk(&vec, size);
    value_to_py(py, &Value::Array(result.into_iter().map(Value::Array).collect()))
}

/// Python wrapper for compact function.
/// Removes null (None) values from the list.
#[pyfunction]
pub fn compact(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let vec = extract_vec(&py_input)?;
    let options: Vec<Option<Value>> = vec
        .into_iter()
        .map(|v| if v.is_null() { None } else { Some(v) })
        .collect();
    let result = core_compact(&options);
    value_to_py(py, &Value::Array(result))
}

/// Python wrapper for flatten_deep function.
#[pyfunction]
pub fn flatten_deep(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let vec: Vec<Vec<Value>> = pythonize::depythonize(&py_input)
        .map_err(|e| pyo3::exceptions::PyTypeError::new_err(format!("Expected nested list: {e}")))?;
    let result = core_flatten_deep(&vec);
    value_to_py(py, &Value::Array(result))
}

/// Python wrapper for unique function.
#[pyfunction]
pub fn unique(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let vec = extract_vec(&py_input)?;
    let hashable: Vec<HashableValue> = vec.into_iter().map(HashableValue).collect();
    let result: Vec<Value> = core_unique(&hashable).into_iter().map(|hv| hv.0).collect();
    value_to_py(py, &Value::Array(result))
}

/// Python wrapper for group_by function.
#[pyfunction]
pub fn group_by(py_input: Bound<'_, PyAny>, py_func: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let vec = extract_vec(&py_input)?;

    if vec.is_empty() {
        let dict = PyDict::new_bound(py);
        return Ok(dict.into());
    }

    let result = core_group_by(&vec, |x: &Value| -> HashableValue {
        let py_x = value_to_py(py, x).unwrap();
        let py_key = py_func.call1((py_x,)).unwrap();
        HashableValue(py_to_value(&py_key).unwrap())
    });

    let dict = PyDict::new_bound(py);
    for (key, values) in result {
        let py_key = value_to_py(py, &key.0)?;
        let py_values: Vec<PyObject> = values
            .iter()
            .map(|v| value_to_py(py, v).unwrap())
            .collect();
        dict.set_item(py_key, py_values)?;
    }
    Ok(dict.into())
}

/// Python wrapper for map function.
#[pyfunction]
pub fn map(py_input: Bound<'_, PyAny>, py_func: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let vec = extract_vec(&py_input)?;
    let result: Vec<Value> = core_map(&vec, |x: &Value| -> Value {
        let py_x = value_to_py(py, x).unwrap();
        let py_result = py_func.call1((py_x,)).unwrap();
        py_to_value(&py_result).unwrap()
    });
    value_to_py(py, &Value::Array(result))
}

/// Python wrapper for filter function.
#[pyfunction]
pub fn filter(py_input: Bound<'_, PyAny>, py_func: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let vec = extract_vec(&py_input)?;
    let result: Vec<Value> = core_filter(&vec, |x: &Value| -> bool {
        let py_x = value_to_py(py, x).unwrap();
        py_func.call1((py_x,)).unwrap().extract::<bool>().unwrap()
    })
    .into_iter()
    .cloned()
    .collect();
    value_to_py(py, &Value::Array(result))
}

/// Python wrapper for find function.
#[pyfunction]
pub fn find(py_input: Bound<'_, PyAny>, py_func: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let vec = extract_vec(&py_input)?;
    match core_find(&vec, |x: &Value| -> bool {
        let py_x = value_to_py(py, x).unwrap();
        py_func.call1((py_x,)).unwrap().extract::<bool>().unwrap()
    }) {
        Some(val) => value_to_py(py, val),
        None => Ok(py.None()),
    }
}

/// Python wrapper for sort_by function.
#[pyfunction]
pub fn sort_by(py_input: Bound<'_, PyAny>, py_func: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let vec = extract_vec(&py_input)?;

    if vec.is_empty() {
        return value_to_py(py, &Value::Array(vec![]));
    }

    let result: Vec<Value> = core_sort_by(&vec, |x: &Value| -> OrdValue {
        let py_x = value_to_py(py, x).unwrap();
        let py_key = py_func.call1((py_x,)).unwrap();
        OrdValue(py_to_value(&py_key).unwrap())
    })
    .into_iter()
    .cloned()
    .collect();
    value_to_py(py, &Value::Array(result))
}

/// Python wrapper for reduce function.
#[pyfunction]
pub fn reduce(
    py_input: Bound<'_, PyAny>,
    py_func: Bound<'_, PyAny>,
    initial: PyObject,
) -> PyResult<PyObject> {
    let py = py_input.py();
    let vec = extract_vec(&py_input)?;
    let init_value = py_to_value(&initial.bind(py))?;

    let result = core_reduce(
        &vec,
        |acc: Value, x: &Value| -> Value {
            let py_acc = value_to_py(py, &acc).unwrap();
            let py_x = value_to_py(py, x).unwrap();
            let py_result = py_func.call1((py_acc, py_x)).unwrap();
            py_to_value(&py_result).unwrap()
        },
        init_value,
    );
    value_to_py(py, &result)
}

/// Python wrapper for zip function.
#[pyfunction]
pub fn zip(py_a: Bound<'_, PyAny>, py_b: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_a.py();
    let a = extract_vec(&py_a)?;
    let b = extract_vec(&py_b)?;
    let result = core_zip(&a, &b);

    // Convert Vec<(Value, Value)> to a Python list of two-element lists.
    let py_list: Vec<Vec<Value>> = result.into_iter().map(|(x, y)| vec![x, y]).collect();
    value_to_py(py, &Value::Array(py_list.into_iter().map(Value::Array).collect()))
}

/// Python wrapper for intersection function.
#[pyfunction]
pub fn intersection(py_a: Bound<'_, PyAny>, py_b: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_a.py();
    let a = extract_vec(&py_a)?;
    let b = extract_vec(&py_b)?;
    let result = core_intersection(&a, &b);
    value_to_py(py, &Value::Array(result))
}
