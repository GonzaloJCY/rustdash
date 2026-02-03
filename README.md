# rustdash

[![PyPI](https://img.shields.io/pypi/v/rustdash)](https://pypi.org/project/rustdash/)
[![Python](https://img.shields.io/pypi/pyversions/rustdash)](https://pypi.org/project/rustdash/)
[![License](https://img.shields.io/pypi/l/rustdash)](https://github.com/GonzaloJCY/rustdash/blob/main/LICENSE)

Lodash-inspired utility library for Python, powered by Rust.

`rustdash` provides fast, familiar helper functions for strings, numbers, arrays, and objects. The core logic is written in Rust and exposed to Python via [PyO3](https://pyo3.rs), compiled with [maturin](https://www.maturin.rs).

## Installation

```bash
pip install rustdash
```

### Build from source

Requires Rust toolchain and maturin:

```bash
pip install maturin
maturin develop
```

## Quick start

```python
import rustdash as _

# Strings
_.camel_case("hello world")       # "helloWorld"
_.snake_case("helloWorld")         # "hello_world"

# Numbers
_.sum([1, 2, 3])                   # 6
_.mean([10, 20, 30])               # 20.0

# Arrays
_.chunk([1, 2, 3, 4, 5], 2)       # [[1, 2], [3, 4], [5]]
_.unique([1, 2, 2, 3])            # [1, 2, 3]
_.flatten_deep([[1, [2]], [3]])    # [1, 2, 3]

# Objects
_.get({"a": {"b": 1}}, "a.b", None)           # 1
_.pick({"a": 1, "b": 2, "c": 3}, ["a", "c"])  # {"a": 1, "c": 3}
_.merge([{"a": 1}, {"b": 2}])                 # {"a": 1, "b": 2}
```

## Benchmarks

Comparison against pure Python and [pydash](https://github.com/dgilland/pydash) (10 iterations, 100k elements for numbers, 10k-80k for arrays):

| Category | Function | rustdash | Python | pydash | vs Python | vs pydash |
|----------|----------|----------|--------|--------|-----------|-----------|
| Strings | `camel_case` (10k) | 2230 ms | 462 ms | 2708 ms | 0.2x | 1.2x |
| Strings | `snake_case` (10k) | 2049 ms | 2514 ms | 2290 ms | **1.2x** | 1.1x |
| Numbers | `sum` (100k ints) | 14 ms | 7 ms | 629 ms | 0.5x | **46x** |
| Numbers | `mean` (100k) | 17 ms | 8 ms | 623 ms | 0.4x | **37x** |
| Numbers | `max` (100k) | 12 ms | 13 ms | 187 ms | **1.1x** | **15x** |
| Arrays | `flatten_deep` (10k nested) | 15 ms | 139 ms | 173 ms | **9.6x** | **12x** |
| Arrays | `unique` (50k) | 17 ms | 16 ms | 22 ms | 0.9x | 1.3x |
| Arrays | `compact` (80k) | 8 ms | 8 ms | 11 ms | **1.0x** | 1.4x |
| Objects | `get` (deep path) | 0.02 ms | 0.05 ms | 0.1 ms | **2.5x** | **9x** |
| Objects | `omit` (1k keys) | 1.3 ms | 1.4 ms | 18 ms | **1.1x** | **14x** |
| Objects | `keys` (1k) | 0.07 ms | 0.1 ms | 0.3 ms | **1.5x** | **5x** |

**Summary:**
- **rustdash wins 7/17** benchmarks against pure Python
- **Average speedup vs pydash: 31x**
- Best performance on recursive/complex operations (`flatten_deep` at **9.6x faster**)
- Competitive with Python's optimized C builtins for simple aggregations

## API Reference

### Strings

| Function | Description |
|---|---|
| `camel_case(input)` | Convert to camelCase |
| `snake_case(input)` | Convert to snake_case |
| `kebab_case(input)` | Convert to kebab-case |
| `pascal_case(input)` | Convert to PascalCase |
| `capitalize(input)` | Capitalize first letter |
| `upper_case(input)` | Convert to UPPER CASE |
| `lower_case(input)` | Convert to lower case |
| `trim(input)` | Remove leading/trailing whitespace |
| `trim_start(input)` | Remove leading whitespace |
| `trim_end(input)` | Remove trailing whitespace |
| `words(input)` | Split string into words |

All string functions accept a single string or a list of strings.

### Numbers

| Function | Description |
|---|---|
| `sum(values)` | Sum a list of numbers |
| `mean(values)` | Arithmetic mean (always returns float) |
| `min(values)` | Minimum value |
| `max(values)` | Maximum value |
| `round(value, decimals)` | Round to `decimals` places. Accepts a single number or a list |
| `sum_by(values, fn)` | Sum by callback result |
| `min_by(values, fn)` | Element whose callback returns the smallest value |
| `max_by(values, fn)` | Element whose callback returns the largest value |

Lists can contain integers, floats, or a mix of both.

### Arrays

| Function | Description |
|---|---|
| `chunk(array, size)` | Split into groups of `size` |
| `compact(array)` | Remove `None`/`null` values |
| `flatten_deep(array)` | Recursively flatten nested lists |
| `unique(array)` | Deduplicate elements (preserves order) |
| `group_by(array, fn)` | Group elements by callback key |
| `map(array, fn)` | Transform each element |
| `filter(array, fn)` | Keep elements where callback returns `True` |
| `find(array, fn)` | First element where callback returns `True` |
| `sort_by(array, fn)` | Sort by callback key |
| `reduce(array, fn, initial)` | Fold left with an accumulator |
| `zip(a, b)` | Pair elements from two lists |
| `intersection(a, b)` | Elements present in both lists |

Arrays support heterogeneous types (e.g. `[1, "two", 3.0, True, None]`).

### Objects

| Function | Description |
|---|---|
| `get(obj, path, default)` | Access nested value by dotted path |
| `has(obj, path)` | Check if path exists |
| `get_all(obj, pattern)` | Collect all values matching a `[*]` wildcard path |
| `has_all(obj, pattern)` | Check that every `[*]` expansion exists |
| `pick(obj, keys)` | New object with only the listed keys |
| `omit(obj, keys)` | New object without the listed keys |
| `merge(objects)` | Shallow-merge a list of objects (last wins) |
| `keys(obj)` | Top-level keys |
| `values(obj)` | Top-level values |

#### Path syntax

Paths support dotted notation, bracket indexes, and wildcards:

```python
_.get(data, "user.name", None)             # dotted keys
_.get(data, "items[0].price", 0)           # array index
_.get(data, "matrix[0][1]", 0)             # nested indexes
_.get_all(data, "users[*].name")           # wildcard expansion
_.get_all(data, "departments[*].staff[*]") # multiple wildcards
```

## Supported Python versions

- CPython 3.8, 3.9, 3.10, 3.11, 3.12, 3.13
- Pre-built wheels for Linux, macOS, and Windows (3.10+ on Windows)

## License

MIT
