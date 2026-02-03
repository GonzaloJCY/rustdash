use crate::core::numbers::{max_, mean_, min_, round_, sum_};
use pyo3::prelude::*;
use pyo3::types::PyList;

// ─── Python wrappers ─────────────────────────────────────────────────────────

#[pyfunction]
pub fn sum(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    if let Ok(ints) = py_input.extract::<Vec<i64>>() {
        return Ok(sum_(&ints).into_py(py));
    }
    let floats = py_input
        .extract::<Vec<f64>>()
        .map_err(|_| pyo3::exceptions::PyTypeError::new_err("Expected a list of numbers"))?;
    Ok(sum_(&floats).into_py(py))
}

#[pyfunction]
pub fn max(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    if let Ok(ints) = py_input.extract::<Vec<i64>>() {
        return Ok(max_(&ints).into_py(py));
    }
    let floats = py_input
        .extract::<Vec<f64>>()
        .map_err(|_| pyo3::exceptions::PyTypeError::new_err("Expected a list of numbers"))?;
    Ok(max_(&floats).into_py(py))
}

#[pyfunction]
pub fn min(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    if let Ok(ints) = py_input.extract::<Vec<i64>>() {
        return Ok(min_(&ints).into_py(py));
    }
    let floats = py_input
        .extract::<Vec<f64>>()
        .map_err(|_| pyo3::exceptions::PyTypeError::new_err("Expected a list of numbers"))?;
    Ok(min_(&floats).into_py(py))
}

#[pyfunction]
pub fn mean(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let floats = py_input
        .extract::<Vec<f64>>()
        .map_err(|_| pyo3::exceptions::PyTypeError::new_err("Expected a list of numbers"))?;
    Ok(mean_(&floats).into_py(py))
}

#[pyfunction]
pub fn round(py_input: Bound<'_, PyAny>, decimals: u32) -> PyResult<PyObject> {
    let py = py_input.py();
    if let Ok(val) = py_input.extract::<f64>() {
        return Ok(round_(val, decimals).into_py(py));
    }
    let floats = py_input
        .extract::<Vec<f64>>()
        .map_err(|_| pyo3::exceptions::PyTypeError::new_err("Expected a number or list of numbers"))?;
    Ok(round_(&floats[..], decimals).into_py(py))
}

#[pyfunction]
pub fn sum_by(py_input: Bound<'_, PyAny>, py_func: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let list = py_input.downcast::<PyList>()?;
    let mut total: f64 = 0.0;
    for item in list.iter() {
        let val: f64 = py_func.call1((&item,))?.extract()?;
        total += val;
    }
    Ok(total.into_py(py))
}

#[pyfunction]
pub fn max_by(py_input: Bound<'_, PyAny>, py_func: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let list = py_input.downcast::<PyList>()?;
    let mut best: Option<(PyObject, f64)> = None;
    for item in list.iter() {
        let val: f64 = py_func.call1((&item,))?.extract()?;
        match &best {
            Some((_, best_val)) if val <= *best_val => {}
            _ => best = Some((item.unbind(), val)),
        }
    }
    match best {
        Some((obj, _)) => Ok(obj),
        None => Ok(py.None()),
    }
}

#[pyfunction]
pub fn min_by(py_input: Bound<'_, PyAny>, py_func: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let list = py_input.downcast::<PyList>()?;
    let mut best: Option<(PyObject, f64)> = None;
    for item in list.iter() {
        let val: f64 = py_func.call1((&item,))?.extract()?;
        match &best {
            Some((_, best_val)) if val >= *best_val => {}
            _ => best = Some((item.unbind(), val)),
        }
    }
    match best {
        Some((obj, _)) => Ok(obj),
        None => Ok(py.None()),
    }
}
