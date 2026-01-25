//! Python bindings for rustdash library.
//! This module organizes all Python-facing functions by category.

mod strings;

use pyo3::prelude::*;

/// Main Python module entry point for rustdash.
/// All exported functions are registered here.
#[pymodule]
pub fn rustdash(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register string functions
    m.add_function(wrap_pyfunction!(strings::camel_case, m)?)?;

    Ok(())
}
