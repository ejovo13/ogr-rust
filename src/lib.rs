use pyo3::prelude::*;

mod rulers;
pub mod enumerations;

use rulers::*;


#[pymodule]
fn ogr_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_golomb_ruler_naive, m)?)?;
    m.add_function(wrap_pyfunction!(generate_golomb_ruler_improved, m)?)?;
    m.add_function(wrap_pyfunction!(enumerations::enumerate_rulers, m)?)?;
    m.add_function(wrap_pyfunction!(enumerations::enumerate_rulers_with_length, m)?)?;
    m.add_function(wrap_pyfunction!(enumerations::enumerate_golomb_rulers, m)?)?;
    m.add_class::<Ruler>()?;
    Ok(())
}
