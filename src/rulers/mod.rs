//! Define the core Ruler structure and some associated functions.

mod golomb_ruler;
mod ruler;

use std::collections::HashSet;

use pyo3::{exceptions, prelude::*};
use pyo3::create_exception;

pub use ruler::Ruler;
pub use golomb_ruler::GolombRuler;

create_exception!(ogr_rust, MyError, pyo3::exceptions::PyException);



#[derive(Debug, Clone)]
#[pyclass]
struct Distance {
    lhs: GInt,
    rhs: GInt,
    dist: GInt,
}

pub(crate) type GInt = i128;

pub(crate) fn dist(a: GInt, b: GInt) -> GInt{
    GInt::abs(a - b)
}

/// Compute the pairwise distances of `sequence` and return the results in a set.
fn compute_distances(sequence: &[i128]) -> HashSet<i128> {

    let mut distances: HashSet<i128> = HashSet::new();
    let n = sequence.len();

    for (lhs_index, lhs) in sequence.iter().enumerate() {
        for rhs in &sequence[(lhs_index + 1)..n] {
            distances.insert(dist(*lhs, *rhs));
        }
    }

    distances
}

/// Naively generate a new golomb ruler with `order` marks
#[pyfunction]
pub(crate) fn generate_golomb_ruler_naive(order: u32) -> PyResult<Vec<i128>> {

    match order {
        0 => Err(exceptions::PyValueError::new_err("Order must be greater than 0!")),
        1 => Ok(vec![0i128]),
        _ => {
            let mut prev = generate_golomb_ruler_naive(order - 1).unwrap();
            let next = i128::pow(2, order - 1) - 1;
            prev.push(next);
            Ok(prev)
        }
    }
}

/// Utility function used to check if a candidate should be accepted.
fn should_accept_candidate(candidate: i128, distances: &HashSet<i128>, prev: &[i128], order: u32) -> bool {
    for i in 0usize..(order - 1) as usize {
        let gap = dist(candidate, prev[i]);
        if distances.contains(&gap) {
            return false
        };
    }

    true
}

/// Slightly improved version of our Golomb Ruler function
#[pyfunction]
pub(crate) fn generate_golomb_ruler_improved(order: u32) -> PyResult<Vec<i128>> {

    match order {
        0 => Err(exceptions::PyValueError::new_err("Order must be greater than 0!")),
        1 => Ok(vec![0i128]),
        2 => Ok(vec![0i128, 1i128]),
        3 => Ok(vec![0i128, 1i128, 3i128]),
        _ => {

            let mut prev = generate_golomb_ruler_improved(order - 1).unwrap();
            let prev_last = prev.last().unwrap();

            let distances = compute_distances(&prev);
            let candidate_upper_bound = 2 * prev_last + 1;

            for c in *prev_last..candidate_upper_bound + 1 {
                if prev.contains(&c) {
                    continue
                };

                if should_accept_candidate(c, &distances, &prev, order) {
                    prev.push(c);
                    prev.sort();
                    return Ok(prev);
                };
            };

            Err(exceptions::PyBaseException::new_err("Implementation Error on behalf of the programmer!"))
        }
    }
}
