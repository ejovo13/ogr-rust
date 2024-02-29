//! Library for fast-enumeration of Golomb Rulers.
//!
//! # Golomb Rulers
//! From [Wikipedia]:
//!
//! > In mathematics, a Golomb ruler is a set of marks at integer positions along a ruler
//! > such that no two pairs of marks are the same distance apart.
//!
//! For example, the following set of integers: `[0, 1, 3, 7]` is a Golomb Ruler of _order_ 4 and _length_ 7.
//! _Order_ refers to the number of marks whereas _length_ refers to magnitude of the largest element. On the contrary,
//! the set `[0, 1, 2]` is _not_ a Golomb Ruler, as the distance between `1` and `2` is a duplicate of the
//! distance between `0` and `1`.
//!
//! This straightforward formulation has led to a an extremely difficult optimization problem: Finding
//! Golomb Rulers of order `k` with minimal length. Such a ruler is called an **O**ptimal **G**olomb **R**uler (OGR) and is the namesake of this library.
//!
//! # OGR Rust
//! This library primarily provides a set of routines used to enumerate Golomb Rulers. Inspired by [Polars] and powered by [PyO3], `ogr_rust` is the backend of the user-facing python API [`ogr-py`].
//! As there are currently no better methods than enumeration to determine if a given Golomb Ruler is indeed optimal, this project aims to develop robust enumeration algorithms to explore the problem space.
//!
//! # Vocabulary
//!
//! * `Ruler` - A set of unique natural numbers. Does not need to satisfy the `Golomb Property`.
//! * `mark` - A single number in our ruler.
//! * `order` - The number of marks of a ruler.
//! * `length` - The magnitude of the largest mark.
//! * `Golomb Property` - categorized by the fact that no two pairs of marks are the same distance apart.
//! * `Golomb Ruler` - A `Ruler` satisfying the `Golomb Property`
//!
//! [Wikipedia]: https://en.wikipedia.org/wiki/Golomb_ruler
//! [Polars]: https://pola.rs/
//! [PyO3]: https://pyo3.rs/v0.20.3/
//! [`ogr-py`]: https://pypi.org/project/ogr-py/


use pyo3::prelude::*;

mod rulers;
pub mod enumeration;

use rulers::*;


#[pymodule]
fn ogr_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_golomb_ruler_naive, m)?)?;
    m.add_function(wrap_pyfunction!(generate_golomb_ruler_improved, m)?)?;
    m.add_function(wrap_pyfunction!(enumeration::enumerate_rulers, m)?)?;
    m.add_function(wrap_pyfunction!(enumeration::enumerate_rulers_with_length, m)?)?;
    m.add_function(wrap_pyfunction!(enumeration::enumerate_golomb_rulers, m)?)?;
    m.add_function(wrap_pyfunction!(enumeration::enumerate_pruned_rulers, m)?)?;
    m.add_function(wrap_pyfunction!(enumeration::enumerate_rulers_with_order, m)?)?;
    m.add_function(wrap_pyfunction!(enumeration::enumerate_golomb_rulers_pruned, m)?)?;
    m.add_class::<Ruler>()?;
    Ok(())
}
