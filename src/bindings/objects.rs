use crate::core::objects::{parse_path, Token};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

// ─── Helpers ────────────────────────────────────────────────────────────────

/// Walk a Python object along parsed path tokens, returning the value or None.
fn walk_path<'py>(
    py: Python<'py>,
    obj: &Bound<'py, PyAny>,
    tokens: &[Token],
) -> Option<PyObject> {
    let mut current: PyObject = obj.clone().unbind();
    for token in tokens {
        let bound = current.bind(py);
        match token {
            Token::Key(key) => match bound.get_item(key.as_str()) {
                Ok(next) => current = next.unbind(),
                Err(_) => return None,
            },
            Token::Index(i) => match bound.get_item(*i) {
                Ok(next) => current = next.unbind(),
                Err(_) => return None,
            },
            Token::Wildcard => return None,
        }
    }
    Some(current)
}

/// Recursively resolve a wildcard path, collecting all matching PyObjects.
fn resolve_path_py<'py>(
    py: Python<'py>,
    current: &Bound<'py, PyAny>,
    tokens: &[Token],
    result: &mut Vec<PyObject>,
) -> PyResult<()> {
    if tokens.is_empty() {
        result.push(current.clone().unbind());
        return Ok(());
    }

    let first = &tokens[0];
    let rest = &tokens[1..];

    match first {
        Token::Key(key) => {
            if let Ok(next) = current.get_item(key.as_str()) {
                resolve_path_py(py, &next, rest, result)?;
            }
        }
        Token::Index(i) => {
            if let Ok(next) = current.get_item(*i) {
                resolve_path_py(py, &next, rest, result)?;
            }
        }
        Token::Wildcard => {
            if let Ok(list) = current.downcast::<PyList>() {
                for item in list.iter() {
                    resolve_path_py(py, &item, rest, result)?;
                }
            }
        }
    }
    Ok(())
}

/// Recursively check that ALL wildcard expansions resolve to existing entries.
fn has_all_paths_py(py: Python<'_>, current: &Bound<'_, PyAny>, tokens: &[Token]) -> bool {
    if tokens.is_empty() {
        return true;
    }

    let first = &tokens[0];
    let rest = &tokens[1..];

    match first {
        Token::Key(key) => match current.get_item(key.as_str()) {
            Ok(next) => has_all_paths_py(py, &next, rest),
            Err(_) => false,
        },
        Token::Index(i) => match current.get_item(*i) {
            Ok(next) => has_all_paths_py(py, &next, rest),
            Err(_) => false,
        },
        Token::Wildcard => {
            if let Ok(list) = current.downcast::<PyList>() {
                if list.is_empty() {
                    return false;
                }
                list.iter().all(|item| has_all_paths_py(py, &item, rest))
            } else {
                false
            }
        }
    }
}

// ─── Python wrappers ─────────────────────────────────────────────────────────

#[pyfunction]
pub fn get(py_input: Bound<'_, PyAny>, path: &str, default: PyObject) -> PyResult<PyObject> {
    let py = py_input.py();
    let tokens = parse_path(path);
    match walk_path(py, &py_input, &tokens) {
        Some(val) => Ok(val),
        None => Ok(default),
    }
}

#[pyfunction]
pub fn has(py_input: Bound<'_, PyAny>, path: &str) -> PyResult<bool> {
    let py = py_input.py();
    let tokens = parse_path(path);
    Ok(walk_path(py, &py_input, &tokens).is_some())
}

#[pyfunction]
pub fn get_all(py_input: Bound<'_, PyAny>, path: &str) -> PyResult<PyObject> {
    let py = py_input.py();
    let tokens = parse_path(path);
    let mut result = Vec::new();
    resolve_path_py(py, &py_input, &tokens, &mut result)?;
    Ok(PyList::new_bound(py, &result).into())
}

#[pyfunction]
pub fn has_all(py_input: Bound<'_, PyAny>, path: &str) -> PyResult<bool> {
    let py = py_input.py();
    let tokens = parse_path(path);
    Ok(has_all_paths_py(py, &py_input, &tokens))
}

#[pyfunction]
pub fn pick(py_input: Bound<'_, PyAny>, py_keys: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let keys: Vec<String> = py_keys.extract()?;

    if let Ok(dict) = py_input.downcast::<PyDict>() {
        let result = PyDict::new_bound(py);
        for key in &keys {
            if let Ok(Some(val)) = dict.get_item(key) {
                result.set_item(key, val)?;
            }
        }
        Ok(result.into())
    } else {
        Ok(PyDict::new_bound(py).into())
    }
}

#[pyfunction]
pub fn omit(py_input: Bound<'_, PyAny>, py_keys: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let keys: Vec<String> = py_keys.extract()?;
    let exclude: std::collections::HashSet<&str> = keys.iter().map(|s| s.as_str()).collect();

    if let Ok(dict) = py_input.downcast::<PyDict>() {
        let result = PyDict::new_bound(py);
        for (k, v) in dict.iter() {
            let key_str: String = k.extract()?;
            if !exclude.contains(key_str.as_str()) {
                result.set_item(k, v)?;
            }
        }
        Ok(result.into())
    } else {
        Ok(PyDict::new_bound(py).into())
    }
}

#[pyfunction]
pub fn merge(py_objects: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_objects.py();
    let list = py_objects.downcast::<PyList>()?;
    let result = PyDict::new_bound(py);

    for item in list.iter() {
        if let Ok(dict) = item.downcast::<PyDict>() {
            for (k, v) in dict.iter() {
                result.set_item(k, v)?;
            }
        }
    }

    Ok(result.into())
}

#[pyfunction]
pub fn keys(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    if let Ok(dict) = py_input.downcast::<PyDict>() {
        Ok(dict.keys().into_any().unbind())
    } else {
        Ok(PyList::new_bound(py, Vec::<PyObject>::new()).into())
    }
}

#[pyfunction]
pub fn values(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    if let Ok(dict) = py_input.downcast::<PyDict>() {
        Ok(dict.values().into_any().unbind())
    } else {
        Ok(PyList::new_bound(py, Vec::<PyObject>::new()).into())
    }
}
