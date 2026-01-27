use pyo3::prelude::*;

/// Python wrapper for max function.
#[pyfunction]
pub fn max(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();

    Ok(py_input.extract::<Vec<f64>>()?.into_py(py))
}

/// Python wrapper for min function.
#[pyfunction]
pub fn min(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();

    Ok(py_input.extract::<Vec<f64>>()?.into_py(py))
}

/// Python wrapper for mean function.
#[pyfunction]
pub fn mean(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();

    Ok(py_input.extract::<Vec<f64>>()?.into_py(py))
}

/// Python wrapper for sum function.
#[pyfunction]
pub fn sum(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();

    Ok(py_input.extract::<Vec<f64>>()?.into_py(py))
}

/// Python wrapper for round function.
#[pyfunction]
pub fn round(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();

    Ok(py_input.extract::<Vec<f64>>()?.into_py(py))
}

#[pyfunction]
pub fn sum_by(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();

    Ok(py_input.extract::<Vec<f64>>()?.into_py(py))
}

#[pyfunction]
pub fn min_by(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();

    Ok(py_input.extract::<Vec<f64>>()?.into_py(py))
}

#[pyfunction]
pub fn max_by(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();

    Ok(py_input.extract::<Vec<f64>>()?.into_py(py))
}
