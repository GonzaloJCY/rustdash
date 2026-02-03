//! Python bindings for rustdash library.
//! This module organizes all Python-facing functions by category.

mod arrays;
mod numbers;
mod objects;
mod strings;
mod strings_helpers;

use pyo3::prelude::*;

/// Main Python module entry point for rustdash.
/// All exported functions are registered here.
#[pymodule]
pub fn _rustdash(m: &Bound<'_, PyModule>) -> PyResult<()> {
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

    // Register arrays functions
    m.add_function(wrap_pyfunction!(arrays::chunk, m)?)?;
    m.add_function(wrap_pyfunction!(arrays::compact, m)?)?;
    m.add_function(wrap_pyfunction!(arrays::flatten_deep, m)?)?;
    m.add_function(wrap_pyfunction!(arrays::unique, m)?)?;
    m.add_function(wrap_pyfunction!(arrays::group_by, m)?)?;
    m.add_function(wrap_pyfunction!(arrays::map, m)?)?;
    m.add_function(wrap_pyfunction!(arrays::filter, m)?)?;
    m.add_function(wrap_pyfunction!(arrays::find, m)?)?;
    m.add_function(wrap_pyfunction!(arrays::sort_by, m)?)?;
    m.add_function(wrap_pyfunction!(arrays::reduce, m)?)?;
    m.add_function(wrap_pyfunction!(arrays::zip, m)?)?;
    m.add_function(wrap_pyfunction!(arrays::intersection, m)?)?;

    // Register object functions
    m.add_function(wrap_pyfunction!(objects::get, m)?)?;
    m.add_function(wrap_pyfunction!(objects::get_all, m)?)?;
    m.add_function(wrap_pyfunction!(objects::pick, m)?)?;
    m.add_function(wrap_pyfunction!(objects::omit, m)?)?;
    m.add_function(wrap_pyfunction!(objects::values, m)?)?;
    m.add_function(wrap_pyfunction!(objects::keys, m)?)?;
    m.add_function(wrap_pyfunction!(objects::has, m)?)?;
    m.add_function(wrap_pyfunction!(objects::has_all, m)?)?;
    m.add_function(wrap_pyfunction!(objects::merge, m)?)?;

    Ok(())
}
