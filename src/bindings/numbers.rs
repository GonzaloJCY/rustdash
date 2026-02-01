use super::helpers::{extract_vec, value_to_py};
use crate::core::numbers::{max_, max_by_, mean_, min_, min_by_, round_, sum_, sum_by_};
use pyo3::prelude::*;
use serde_json::Value;

// ─── Helper functions ────────────────────────────────────────────────────────

/// Extract a `Vec<Value>` as `Vec<f64>`, failing if any element is not numeric.
fn values_to_f64(values: &[Value]) -> PyResult<Vec<f64>> {
    values
        .iter()
        .map(|v| {
            v.as_f64().ok_or_else(|| {
                pyo3::exceptions::PyTypeError::new_err("All elements must be numbers")
            })
        })
        .collect()
}

/// Returns true when every element in the slice is an integer (i64 or u64).
fn all_integers(values: &[Value]) -> bool {
    values.iter().all(|v| v.as_i64().is_some())
}

// ─── Python wrappers ─────────────────────────────────────────────────────────

/// Python wrapper for sum function.
/// Accepts a list of integers, floats, or a mix of both.
#[pyfunction]
pub fn sum(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let values = extract_vec(&py_input)?;
    if all_integers(&values) {
        let ints: Vec<i64> = values.iter().map(|v| v.as_i64().unwrap()).collect();
        Ok(sum_(&ints).into_py(py))
    } else {
        let floats = values_to_f64(&values)?;
        Ok(sum_(&floats).into_py(py))
    }
}

/// Python wrapper for max function.
/// Accepts a list of integers, floats, or a mix of both.
#[pyfunction]
pub fn max(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let values = extract_vec(&py_input)?;
    if all_integers(&values) {
        let ints: Vec<i64> = values.iter().map(|v| v.as_i64().unwrap()).collect();
        Ok(max_(&ints).into_py(py))
    } else {
        let floats = values_to_f64(&values)?;
        Ok(max_(&floats).into_py(py))
    }
}

/// Python wrapper for min function.
/// Accepts a list of integers, floats, or a mix of both.
#[pyfunction]
pub fn min(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let values = extract_vec(&py_input)?;
    if all_integers(&values) {
        let ints: Vec<i64> = values.iter().map(|v| v.as_i64().unwrap()).collect();
        Ok(min_(&ints).into_py(py))
    } else {
        let floats = values_to_f64(&values)?;
        Ok(min_(&floats).into_py(py))
    }
}

/// Python wrapper for mean function.
/// Always returns f64.
#[pyfunction]
pub fn mean(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let values = extract_vec(&py_input)?;
    let floats = values_to_f64(&values)?;
    Ok(mean_(&floats).into_py(py))
}

/// Python wrapper for round function.
/// Accepts either a single float or a list of floats.
#[pyfunction]
pub fn round(py_input: Bound<'_, PyAny>, decimals: u32) -> PyResult<PyObject> {
    let py = py_input.py();

    if let Ok(val) = py_input.extract::<f64>() {
        Ok(round_(val, decimals).into_py(py))
    } else {
        let values = extract_vec(&py_input)?;
        let floats = values_to_f64(&values)?;
        Ok(round_(&floats[..], decimals).into_py(py))
    }
}

/// Python wrapper for sum_by function.
/// Accepts any list; the callback must return a number.
#[pyfunction]
pub fn sum_by(py_input: Bound<'_, PyAny>, py_func: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let vec = extract_vec(&py_input)?;
    let result = sum_by_(&vec, |x: &Value| -> f64 {
        let py_x = value_to_py(py, x).unwrap();
        py_func.call1((py_x,)).unwrap().extract::<f64>().unwrap()
    });
    Ok(result.into_py(py))
}

/// Python wrapper for max_by function.
/// Accepts any list; the callback must return a number.
#[pyfunction]
pub fn max_by(py_input: Bound<'_, PyAny>, py_func: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let vec = extract_vec(&py_input)?;
    let result = max_by_(&vec, |x: &Value| -> f64 {
        let py_x = value_to_py(py, x).unwrap();
        py_func.call1((py_x,)).unwrap().extract::<f64>().unwrap()
    });
    match result {
        Some(val) => value_to_py(py, val),
        None => Ok(py.None()),
    }
}

/// Python wrapper for min_by function.
/// Accepts any list; the callback must return a number.
#[pyfunction]
pub fn min_by(py_input: Bound<'_, PyAny>, py_func: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let vec = extract_vec(&py_input)?;
    let result = min_by_(&vec, |x: &Value| -> f64 {
        let py_x = value_to_py(py, x).unwrap();
        py_func.call1((py_x,)).unwrap().extract::<f64>().unwrap()
    });
    match result {
        Some(val) => value_to_py(py, val),
        None => Ok(py.None()),
    }
}
