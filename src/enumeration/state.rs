//! This module defines operations on the `state` of a Golomb Ruler.
//!
//! A ruler's `state` is a `Vec<bool>` that indicates which lengths are marks on the ruler.
//!
//! # Example
//!
//! Consider the ruler with marks `[0, 1, 4]`. The corresponding state vector is:
//! ```text
//! [true, false, false]
//!    1     2      3
//! ````
//!
//! We don't need to store true values for mark 0 and the total
//! length of a ruler, because
//!
//! - 0 is trivially always a mark on our ruler
//! - The `length` mark can always be recovered by inspecting the `.len()` of our `state`.

use crate::{Ruler, GolombRuler, GInt, dist};
use pyo3::prelude::*;

const TWO: u64 = 2;

/// Trait to add functionality to a vector of booleans
pub(super) trait RulerState {
    fn jump_back(&self) -> Vec<bool>;
    fn to_u64(&self) -> Option<u64>;
    /// Count the number of true values in this iterator state
    fn count_marks(&self) -> usize;
    /// Return the next state with max number of marks `order` and max length `length`
    fn next(&self, length: usize) -> Option<Vec<bool>>;
    /// Test if every bool is true in this RulerState
    fn all(&self) -> bool;
    /// Append a false to the end of this vector
    fn go_left(&self) -> Vec<bool>;
    /// Bounce back one level of ancestry, then go right
    fn back_one_then_right(&self) -> Vec<bool>;
    /// Traverse upwards until we've reached an untouched node
    fn backtrack(&self) -> Vec<bool>;
    fn to_string(&self) -> String;
    /// Convert the current state into a golomb ruler
    fn to_ruler(&self) -> GolombRuler;
    fn total_marks(&self) -> usize;
    fn next_pruned(&self, order: usize, length: usize) -> Option<Vec<bool>>;
    fn next_golomb_depth_1(&self, order: usize, length: usize) -> Option<Vec<bool>>;
    fn contains(&self, value: GInt) -> bool;
    fn is_golomb_ruler_order_1(&self) -> bool;
    fn add_mark(&self) -> Option<Vec<bool>>;
    fn pruned_propose_next(&self, order: usize) -> Option<Vec<bool>>;
}

impl RulerState for Vec<bool> {

    fn to_u64(&self) -> Option<u64> {

        let n = self.len();
        if n > 64 {
            None
        } else {
            let mut int: u64 = 0;
            for i in 0..n {
                if self[n - 1 - i] {
                    int += TWO.pow((i).try_into().unwrap())
                }
            }
            Some(int)
        }
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
    fn is_golomb_ruler_order_1(&self) -> bool {
        // start with the first element of our ruler and subtract every other element
        let ruler = self.to_ruler();
        let base = ruler.marks[ruler.marks.len() - 1]; // 0 is implied, our first element

        if ruler.marks.is_empty() || ruler.marks.len() == 1 {
            return true
        }

        for m in &ruler.marks[0..(ruler.marks.len() - 1)] {
            let d = dist(*m, base);
            if self.contains(d) {
                return false;
            }
        }
        true
    }

    fn count_marks(&self) -> usize {
        let mut count = 0;
        self.iter().for_each(|b| if *b { count += 1 });
        count
    }

    /// Return the total number of marks (including the implied start and end points).
    fn total_marks(&self) -> usize {
        self.count_marks() + 2
    }

    fn all(&self) -> bool {
        for b in self {
            if !b { return false }
        }
        true
    }

    fn go_left(&self) -> Vec<bool> {
        let mut out = self.clone();
        out.push(false);
        out
    }

    fn back_one_then_right(&self) -> Vec<bool> {
        let mut out = self.clone();
        out[self.len() - 1] = true;
        out
    }

    /// 0111 -> 1000
    fn backtrack(&self) -> Vec<bool> {

        let mut out = self.clone();
        // Iterate backwards
        for i in (0..self.len()).rev() {
            if out[i] {
                out[i] = false;
            } else {
                out[i] = true;
                break;
            }
        }

        out
    }

    fn to_string(&self) -> String {
        self.iter().map(|b| if *b { '1' } else { '0' } ).collect()
    }

    /// Convert the current RulerState into a full-fledged GolombRuler
    fn to_ruler(&self) -> GolombRuler {
        // 0 is implied and the final element is implied as well!
        let length_of_ruler = self.len() + 1;
        let mut marks: Vec<GInt> = self.iter().enumerate().filter_map(|(idx, b)| {
            if *b {
                Some((idx + 1) as GInt)
            } else {
                None
            }
        }).collect();
        marks.push(length_of_ruler as GInt);

        GolombRuler {
            marks,
        }
    }

    fn next(&self, length: usize) -> Option<Vec<bool>> {

        if self.len() > length - 1 {
            return None
        }

        if self.len() < length - 1 {

            // Always 'go to the left'
            let mut left = self.go_left();
            while left.len() != length - 1 {
                left = left.go_left();
            }
            Some(left)

        } else {

            // If our final element is 0, then we simply bounce back one and go to the right
            if !self[self.len() - 1] {
                Some(self.back_one_then_right())
            } else {
                // We need to back track!
                // .. unless we should end iteration!
                if self.all() {
                    None
                } else {
                    Some(self.backtrack())
                }
            }
        }
    }

    /// Attempt to add one mark to this state vector.
    ///
    /// If we can't increase the order tally, return None.
    fn add_mark(&self) -> Option<Vec<bool>> {

        let mut out = self.clone();

        // Iterating backwards, find the first zero
        for i in (0..self.len()).rev() {
            if !self[i] {
                out[i] = true;
                return Some(out);
            }
        }

        // And return None if our state is full of ones
        None
    }

    /// Improved implementation of `next` by pruning trees that have too many points
    fn next_pruned(&self, order: usize, length: usize) -> Option<Vec<bool>> {

        if self.len() > length - 1 {
            return None
        }

        if self.len() < length - 1 {
            return self.go_left().next_pruned(order, length);
        }

        let mut next = self.pruned_propose_next(order)?;

        while next.total_marks() != order {
            next = next.pruned_propose_next(order)?;
        }

        Some(next)

    }

    fn next_golomb_depth_1(&self, order: usize, length: usize) -> Option<Vec<bool>> {
        // First get next ruler
        let mut next_pruned = self.next_pruned(order, length)?;
        // if next_pruned.to_ruler().

        while !next_pruned.is_golomb_ruler_order_1() {
            next_pruned = next_pruned.next_pruned(order, length)?;
        }

        Some(next_pruned)
    }

    /// Skip over elements who are saturated
    ///
    /// With 2 total true values, send 010 -> 100 (instead of 011)
    ///
    /// This function should be called when we are about to add an additional mark
    /// but we actually already have enough marks
    /// The caller must protect against the case where the first boolean is 1.
    fn jump_back(&self) -> Vec<bool> {

        // Just roll back the first one that we encounter
        let mut out = self.clone();
        let mut j: usize = 0;
        for i in (0..out.len()).rev() {
            if out[i] {
                out[i] = false;
                j = i;
                // out[i - 1] = true;
                break
            }
        }

        // Now go through consecutive ones
        for i in (0..j).rev() {
            // If our consecutive ones have ended,
            if !out[i] {
                out[i] = true;
                break
            } else {
                out[i] = false;
            }
        }

        out
    }


    /// Given a vector of booleans, propose the next pruned ruler.
    ///
    /// This function helps us implement an iterative version of `next_pruned` that doesn't use recursion
    fn pruned_propose_next(&self, order: usize) -> Option<Vec<bool>> {

        // If our final element is 0, then we simply bounce back one and go to the right
        if !self[self.len() - 1] {

            let n_marks = self.total_marks();

            // If we have too many marks, we can't continue to the right.
            if n_marks == order {

                // If our vector starts with 1, then we are totally done.
                // This condition needs to change to: if the first (order - 2) elements are 1
                if self.iter().take(order - 2).all(|b| *b) {
                // if self[0] {
                    None
                } else {
                    Some(self.jump_back())
                }
            } else {
                self.add_mark()
            }

        } else {
            // We need to back track!
            // .. unless we should end iteration!
            if self.all() {
                None
            } else {
                Some(self.backtrack())
            }
        }
    }

    /// Check if self contains the value `value`
    fn contains(&self, value: GInt) -> bool {

        let length = self.len() + 1;

        if value < 0 {
            return false
        }

        if value == 0 || value == length as GInt {
            true
        } else if value as usize > length  {
            false
        } else {
            self[(value - 1) as usize]
        }
    }

}

#[pymethods]
impl GolombRuler {

    #[staticmethod]
    pub fn from_id(id: usize) -> GolombRuler {

        if id == 0 {
            GolombRuler {
                marks: vec![],
            }
        } else if id == 1 {
            GolombRuler {
                marks: vec![1]
            }
        } else {

            // Break the id into binary
            // for bit in id.view_bits() {

            // }
            let length = id.ilog2() as usize + 1;

            // Now that's the _length_ of our bit vector!
            // println!("Magnitude: {}", magnitude);
            let mut state = vec![false; length - 1];

            for (i, b) in state.iter_mut().enumerate().take(length - 1) {

                let mask = 1usize << i;
                if (id & mask) == mask {
                    *b = true
                }
            }

            state.to_ruler()
        }
    }

    #[staticmethod]
    pub fn from_ids(start_idx: usize, end_idx: usize) -> Vec<GolombRuler> {
        (start_idx..end_idx).map(GolombRuler::from_id).collect::<Vec<GolombRuler>>()
    }

    /// Return the next _RULER_ with order `order` and length `length`, not necessarily the next golomb ruler
    pub fn next_pruned(&self, order: usize, length: usize) -> Option<GolombRuler> {
        Some(self.to_state().next_pruned(order, length)?.to_ruler())
    }



}


#[pymethods]
impl Ruler {

    /// Create a new ruler from a given id.
    #[staticmethod]
    #[pyo3(text_signature = "(id: int)")]
    fn from_id(id: usize) -> GolombRuler {
        GolombRuler::from_id(id)
    }

}