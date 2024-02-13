use pyo3::prelude::*;

mod rulers;

use rulers::*;


#[pymodule]
fn ogr_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_golomb_ruler_naive, m)?)?;
    m.add_function(wrap_pyfunction!(generate_golomb_ruler_improved, m)?)?;
    m.add_class::<Ruler>()?;
    Ok(())
}
