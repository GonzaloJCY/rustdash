//! Python bindings for rustdash library.
//! This module organizes all Python-facing functions by category.

mod numbers;
mod strings;
mod strings_helpers;

use pyo3::prelude::*;

/// Main Python module entry point for rustdash.
/// All exported functions are registered here.
#[pymodule]
pub fn rustdash(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register string functions
    m.add_function(wrap_pyfunction!(strings::camel_case, m)?)?;
    m.add_function(wrap_pyfunction!(strings::kebab_case, m)?)?;
    m.add_function(wrap_pyfunction!(strings::pascal_case, m)?)?;
    m.add_function(wrap_pyfunction!(strings::snake_case, m)?)?;
    m.add_function(wrap_pyfunction!(strings::capitalize, m)?)?;
    m.add_function(wrap_pyfunction!(strings::upper_case, m)?)?;
    m.add_function(wrap_pyfunction!(strings::lower_case, m)?)?;
    m.add_function(wrap_pyfunction!(strings::trim, m)?)?;
    m.add_function(wrap_pyfunction!(strings::trim_start, m)?)?;
    m.add_function(wrap_pyfunction!(strings::trim_end, m)?)?;
    m.add_function(wrap_pyfunction!(strings::words, m)?)?;

    // Register number functions
    m.add_function(wrap_pyfunction!(numbers::sum, m)?)?;
    m.add_function(wrap_pyfunction!(numbers::mean, m)?)?;
    m.add_function(wrap_pyfunction!(numbers::min, m)?)?;
    m.add_function(wrap_pyfunction!(numbers::max, m)?)?;
    m.add_function(wrap_pyfunction!(numbers::round, m)?)?;
    m.add_function(wrap_pyfunction!(numbers::sum_by, m)?)?;
    m.add_function(wrap_pyfunction!(numbers::min_by, m)?)?;
    m.add_function(wrap_pyfunction!(numbers::max_by, m)?)?;

    Ok(())
}
