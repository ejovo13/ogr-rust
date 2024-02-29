
use pyo3::pyclass;
use crate::{GInt, dist};
use std::collections::HashSet;

/// Implementation of a Golomb Ruler.
#[pyclass]
pub struct Ruler {

}

// #[pymethods]
impl Ruler {

    /// Verify if a sequence of integers satisfies the golomb property.
    // #[staticmethod]
    pub fn is_golomb_ruler(sequence: &[GInt]) -> bool {

        let mut distances: HashSet<GInt> = HashSet::new();
        let n = sequence.len();

        for (lhs_index, lhs) in sequence.iter().enumerate() {
            // for rhs in sequence[]
            let diff = *lhs;
            if distances.contains(&diff) {
                return false
            } else {
                distances.insert(diff)
            };

            for rhs in &sequence[(lhs_index + 1)..n] {
                let diff = dist(*lhs, *rhs);
                if distances.contains(&diff) {
                    return false
                } else {
                    distances.insert(diff)
                };
            }
        }

        true
    }

}

