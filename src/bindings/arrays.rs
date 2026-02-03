use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;

// ─── Helpers ────────────────────────────────────────────────────────────────

/// Recursively flatten nested lists into a flat Vec<PyObject>.
fn flatten_recursive(obj: &Bound<'_, PyAny>, result: &mut Vec<PyObject>) -> PyResult<()> {
    if let Ok(list) = obj.downcast::<PyList>() {
        for item in list.iter() {
            flatten_recursive(&item, result)?;
        }
    } else {
        result.push(obj.clone().unbind());
    }
    Ok(())
}

// ─── Python wrappers ─────────────────────────────────────────────────────────

#[pyfunction]
pub fn chunk(py_input: Bound<'_, PyAny>, size: usize) -> PyResult<PyObject> {
    let py = py_input.py();
    let list = py_input.downcast::<PyList>()?;
    let items: Vec<PyObject> = list.iter().map(|x| x.unbind()).collect();

    let result: Vec<PyObject> = items
        .chunks(size)
        .map(|c| PyList::new_bound(py, c).into())
        .collect();

    Ok(PyList::new_bound(py, &result).into())
}

#[pyfunction]
pub fn compact(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let list = py_input.downcast::<PyList>()?;
    let result: Vec<PyObject> = list
        .iter()
        .filter(|item| !item.is_none())
        .map(|item| item.unbind())
        .collect();
    Ok(PyList::new_bound(py, &result).into())
}

#[pyfunction]
pub fn flatten_deep(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let mut result = Vec::new();
    flatten_recursive(&py_input, &mut result)?;
    Ok(PyList::new_bound(py, &result).into())
}

#[pyfunction]
pub fn unique(py_input: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();

    // Fast path: integer list
    if let Ok(ints) = py_input.extract::<Vec<i64>>() {
        let mut seen = std::collections::HashSet::new();
        let result: Vec<i64> = ints.into_iter().filter(|x| seen.insert(*x)).collect();
        return Ok(PyList::new_bound(py, &result).into());
    }

    // Fast path: string list
    if let Ok(strings) = py_input.extract::<Vec<String>>() {
        let mut seen = std::collections::HashSet::new();
        let result: Vec<String> = strings
            .into_iter()
            .filter(|x| seen.insert(x.clone()))
            .collect();
        return Ok(PyList::new_bound(py, &result).into());
    }

    // Fallback: generic Python objects using hash/eq
    let list = py_input.downcast::<PyList>()?;
    let mut buckets: HashMap<isize, Vec<PyObject>> = HashMap::new();
    let mut result: Vec<PyObject> = Vec::new();

    for item in list.iter() {
        let hash = item.hash()?;
        let bucket = buckets.entry(hash).or_default();
        let is_dup = bucket
            .iter()
            .any(|existing| item.eq(existing.bind(py)).unwrap_or(false));
        if !is_dup {
            bucket.push(item.clone().unbind());
            result.push(item.unbind());
        }
    }

    Ok(PyList::new_bound(py, &result).into())
}

#[pyfunction]
pub fn group_by(py_input: Bound<'_, PyAny>, py_func: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let list = py_input.downcast::<PyList>()?;
    let result = PyDict::new_bound(py);

    for item in list.iter() {
        let key = py_func.call1((&item,))?;
        if let Ok(Some(existing)) = result.get_item(&key) {
            let existing_list = existing.downcast::<PyList>()?;
            existing_list.append(&item)?;
        } else {
            let new_list = PyList::new_bound(py, &[item.unbind()]);
            result.set_item(&key, new_list)?;
        }
    }

    Ok(result.into())
}

#[pyfunction]
pub fn map(py_input: Bound<'_, PyAny>, py_func: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let list = py_input.downcast::<PyList>()?;
    let mut result = Vec::with_capacity(list.len());
    for item in list.iter() {
        let mapped = py_func.call1((&item,))?;
        result.push(mapped.unbind());
    }
    Ok(PyList::new_bound(py, &result).into())
}

#[pyfunction]
pub fn filter(py_input: Bound<'_, PyAny>, py_func: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let list = py_input.downcast::<PyList>()?;
    let mut result = Vec::new();
    for item in list.iter() {
        let keep: bool = py_func.call1((&item,))?.extract()?;
        if keep {
            result.push(item.unbind());
        }
    }
    Ok(PyList::new_bound(py, &result).into())
}

#[pyfunction]
pub fn find(py_input: Bound<'_, PyAny>, py_func: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let list = py_input.downcast::<PyList>()?;
    for item in list.iter() {
        let matches: bool = py_func.call1((&item,))?.extract()?;
        if matches {
            return Ok(item.unbind());
        }
    }
    Ok(py.None())
}

#[pyfunction]
pub fn sort_by(py_input: Bound<'_, PyAny>, py_func: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_input.py();
    let list = py_input.downcast::<PyList>()?;

    let mut items: Vec<(PyObject, PyObject)> = Vec::with_capacity(list.len());
    for item in list.iter() {
        let key = py_func.call1((&item,))?;
        items.push((item.unbind(), key.unbind()));
    }

    items.sort_by(|a, b| {
        let a_key = a.1.bind(py);
        let b_key = b.1.bind(py);
        a_key.compare(b_key).unwrap_or(std::cmp::Ordering::Equal)
    });

    let result: Vec<PyObject> = items.into_iter().map(|(item, _)| item).collect();
    Ok(PyList::new_bound(py, &result).into())
}

#[pyfunction]
pub fn reduce(
    py_input: Bound<'_, PyAny>,
    py_func: Bound<'_, PyAny>,
    initial: PyObject,
) -> PyResult<PyObject> {
    let py = py_input.py();
    let list = py_input.downcast::<PyList>()?;
    let mut acc = initial;
    for item in list.iter() {
        acc = py_func.call1((acc.bind(py), &item))?.unbind();
    }
    Ok(acc)
}

#[pyfunction]
pub fn zip(py_a: Bound<'_, PyAny>, py_b: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_a.py();
    let list_a = py_a.downcast::<PyList>()?;
    let list_b = py_b.downcast::<PyList>()?;

    let len = std::cmp::min(list_a.len(), list_b.len());
    let mut result = Vec::with_capacity(len);
    for i in 0..len {
        let a = list_a.get_item(i)?;
        let b = list_b.get_item(i)?;
        let pair = PyList::new_bound(py, &[a.unbind(), b.unbind()]);
        result.push(pair.into_any().unbind());
    }
    Ok(PyList::new_bound(py, &result).into())
}

#[pyfunction]
pub fn intersection(py_a: Bound<'_, PyAny>, py_b: Bound<'_, PyAny>) -> PyResult<PyObject> {
    let py = py_a.py();

    // Fast path: integers
    if let (Ok(a), Ok(b)) = (py_a.extract::<Vec<i64>>(), py_b.extract::<Vec<i64>>()) {
        let set_b: std::collections::HashSet<i64> = b.into_iter().collect();
        let mut seen = std::collections::HashSet::new();
        let result: Vec<i64> = a
            .into_iter()
            .filter(|x| set_b.contains(x) && seen.insert(*x))
            .collect();
        return Ok(PyList::new_bound(py, &result).into());
    }

    // Fallback: generic using Python hash/eq
    let list_a = py_a.downcast::<PyList>()?;
    let list_b = py_b.downcast::<PyList>()?;

    // Build lookup from b
    let mut b_buckets: HashMap<isize, Vec<PyObject>> = HashMap::new();
    for item in list_b.iter() {
        let hash = item.hash()?;
        b_buckets.entry(hash).or_default().push(item.unbind());
    }

    let mut seen: HashMap<isize, Vec<PyObject>> = HashMap::new();
    let mut result: Vec<PyObject> = Vec::new();

    for item in list_a.iter() {
        let hash = item.hash()?;

        // Check if in b
        let in_b = b_buckets
            .get(&hash)
            .map(|bucket| {
                bucket
                    .iter()
                    .any(|b_item| item.eq(b_item.bind(py)).unwrap_or(false))
            })
            .unwrap_or(false);

        if !in_b {
            continue;
        }

        // Dedup
        let seen_bucket = seen.entry(hash).or_default();
        let is_dup = seen_bucket
            .iter()
            .any(|existing| item.eq(existing.bind(py)).unwrap_or(false));
        if !is_dup {
            seen_bucket.push(item.clone().unbind());
            result.push(item.unbind());
        }
    }

    Ok(PyList::new_bound(py, &result).into())
}
