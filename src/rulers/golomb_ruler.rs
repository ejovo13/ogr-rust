
use std::collections::HashSet;
use pyo3::prelude::*;
use crate::{GInt, dist};
use super::{Ruler, Distance};

#[derive(Clone, Debug)]
#[pyclass]
pub struct GolombRuler {
    pub(crate) marks: Vec<GInt>
}

impl GolombRuler {
    pub fn new(marks: &[GInt]) -> Self {
        GolombRuler {
            marks: marks.to_vec()
        }
    }
}

impl std::fmt::Display for GolombRuler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        // prepend a zero to our vector
        let mut out: Vec<GInt> = vec![0];
        out.append(&mut self.marks.clone());
        write!(f, "{:?}", out)
    }
}

#[pymethods]
impl GolombRuler {

    fn __repr__(&self) -> String {
        self.to_string()
    }

    pub fn order(&self) -> usize {
        self.marks.len() + 1
    }

    pub fn length(&self) -> GInt {
        if self.marks.is_empty() {
            return 0
        }
        self.marks[self.marks.len() - 1]
    }

    /// Check if the data in `self.marks` actual admits the Golomb Property.
    pub fn is_golomb_ruler(&self) -> bool {
        Ruler::is_golomb_ruler(&self.marks)
    }

    /// Return the marks as a set
    pub fn as_set(&self) -> HashSet<GInt> {
        let mut out = HashSet::<GInt>::new();

        for m in &self.marks {
            out.insert(*m);
        }

        out
    }


    /// Used in enumeration algorithms. Check if the first order of distances are unique.
    ///
    /// Consider the following array
    ///
    /// `[1, 3, 4]`
    ///
    /// 0th order: `[1, 3, 4]`
    /// 1st order: `[2, 3]`
    /// 2nd order: `[1]`
    ///
    /// The idea is to prune our enumeration tree based on a O(2n) or O(n) check; n the number of marks on our ruler
    /// We can later iterate over the returned rulers to actually filter them.
    pub fn is_golomb_ruler_order_1(&self) -> bool {
        // start with the last element of our ruler and subtract every other element
        let set = self.as_set();
        let base = self.length(); // 0 is implied, our first element

        for m in &self.marks[0..(self.marks.len() - 1)] {
            let d = dist(*m, base);
            if set.contains(&d) {
                return false;
            }
        }
        true
    }

    pub fn distances(&self) -> Vec<Distance> {

        let mut out: Vec<Distance> = Vec::new();

        for (idx, lhs) in self.marks.iter().enumerate() {
            out.push(Distance {
                lhs: 0,
                rhs: *lhs,
                dist: *lhs
            });
            for rhs in &self.marks[idx + 1..] {
                out.push(Distance {
                    lhs: *lhs,
                    rhs: *rhs,
                    dist: GInt::abs(rhs - lhs)
                });
            }
        }

        out
    }

    pub fn to_id(&self) -> Option<usize> {

        // [0]
        if self.marks.is_empty() {
            Some(0)
        } else if self.length() == 1 {
        // [0, 1]
            Some(1)
        } else if self.length() > 64 {
            None
        } else {
            let mut val = 1 << (self.length() - 1);
            // println!("Starting value: {}", val);
            let state = self.to_state();
            self.marks[0..self.marks.len() - 1].iter().for_each(|m| if state[(*m - 1) as usize] { val += 1 << (m - 1)} );

            Some(val)
        }
    }


    pub fn to_state(&self) -> Vec<bool> {

        let l = self.length();

        if self.order() == 1 || self.length() == 1 {
            return vec![]
        }

        // Initialize with all false, dropping the 0 and length
        let mut state = vec![false; (l - 1) as usize];

        // Drop the 0
        for m in &self.marks[0..self.marks.len() - 1] {
            state[*m as usize - 1] = true;
        }

        state
    }

}