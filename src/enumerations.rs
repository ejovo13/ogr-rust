//! Functions for enumerating all of the potential golomb rulers

use std::vec;
use pyo3::prelude::*;

use thiserror::Error;
use num_integer::binomial;

use crate::Ruler;
use crate::GInt;

#[derive(Error, Debug)]
pub enum GolombIterationError {
    #[error("The current golomb ruler index can't be contained in a u64")]
    IndexOverflow
}


/// Our ultimate goal is to create an enumerator that walks along all possible
/// golomb rulers of a certain size
///

// fn enumerate_rulers_with_length()

#[pyfunction]
pub fn enumerate_rulers(max_length: usize) -> Vec<GolombRuler> {
    let all_rulers: Vec<GolombRuler> = (2..max_length + 1).flat_map(|length| {
        RulerIterator::new(length)
    }).collect();

    all_rulers
}

#[pyfunction]
pub fn enumerate_rulers_with_length(length: usize) -> Vec<GolombRuler> {
    match length {
        0 => vec![GolombRuler::from_id(0)],
        1 => vec![GolombRuler::from_id(1)],
        _ => RulerIterator::new(length).collect()
    }
}

fn enumerate_pruned_rulers(order: usize, length: usize) -> Vec<GolombRuler> {
    GolombRulerPrunedIterator::new(order, length).collect()
}

/// Get all rulers with `order` marks. They do not have to have the golomb property.
fn enumerate_rulers_with_order(order: usize, length: usize) -> Vec<GolombRuler> {
    let rulers = enumerate_rulers(length);
    rulers.into_iter().filter(|r| r.order() == order).collect()
}

/// Print out every possible golomb ruler of order `order`
///
#[pyfunction]
#[pyo3(text_signature = "(order: int, max_length: ing)")]
pub fn enumerate_golomb_rulers(order: usize, max_length: usize) -> Vec<GolombRuler> {

    let all_rulers: Vec<GolombRuler> = (2..max_length + 1).flat_map(|length| {
        RulerIterator::new(length)
    }).collect();

    // all_rulers
    let filtered: Vec<GolombRuler> = all_rulers.into_iter().filter(|r| r.order() == order && r.is_golomb_ruler()).collect();
    // for ruler in filtered.clone().into_iter() {
    //     dbg!(ruler.length());
    // }

    filtered
}

fn enumerate_golomb_rulers_pruned(order: usize, max_length: usize) -> Vec<GolombRuler> {

    let all_rulers: Vec<GolombRuler> = (2..max_length + 1).flat_map(|length| {
        GolombRulerPrunedIterator::new(order, length)
    }).collect();

    // all_rulers
    let filtered: Vec<GolombRuler> = all_rulers.into_iter().filter(|r| r.is_golomb_ruler()).collect();
    // for ruler in filtered.clone().into_iter() {
    //     dbg!(ruler.length());
    // }

    filtered
}


// If we start with the smallest possible length, then the _very_ first ruler that we encounter
// is optional!

// Some facts about our object that we are working with:
// - there are an infinite number of golomb rulers
// - for a given length, there is only a certain number of golomb rulers possible


/// Iterator over all possible rulers with length `length`.
struct RulerIterator {
    state: Vec<bool>,
    length: usize
}

impl GolombRulerPrunedIterator {
    fn new(order: usize, length: usize) -> Self {
        // The initial state should return our starting point on next()
        // this pre-state accomplishes that
        let pre_state = vec![false; length - 2];
        GolombRulerPrunedIterator {
            state: pre_state,
            order,
            length,
        }
    }
}

/// Iterator that prunes the tree when the order has been hit or the golomb property is
#[derive(Debug)]
struct GolombRulerPrunedIterator {
    state: Vec<bool>,
    order: usize,
    length: usize
}

impl RulerIterator {
    fn new(length: usize) -> Self {
        // The initial state should return our starting point on next()
        // this pre-state accomplishes that
        let pre_state = vec![false; length - 2];
        RulerIterator {
            state: pre_state,
            length,
        }
    }
}

/// GolombRuler integer subtype

/// 0-implied GolombRuler.

#[derive(Clone, Debug)]
#[pyclass]
pub struct GolombRuler {
    marks: Vec<GInt>
}

#[derive(Debug, Clone)]
#[pyclass]
struct Distance {
    lhs: GInt,
    rhs: GInt,
    dist: GInt,
}

#[pymethods]
impl GolombRuler {

    fn __repr__(&self) -> String {
        self.to_string()
    }

    fn order(&self) -> usize {
        self.marks.len() + 1
    }

    fn length(&self) -> GInt {
        if self.marks.is_empty() {
            return 0
        }
        self.marks[self.marks.len() - 1]
    }

    fn is_golomb_ruler(&self) -> bool {
        Ruler::is_golomb_ruler(&self.marks)
    }

    fn distances(&self) -> Vec<Distance> {

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

    fn to_id(&self) -> Option<usize> {

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

    #[staticmethod]
    fn from_id(id: usize) -> GolombRuler {

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
    fn from_ids(start_idx: usize, end_idx: usize) -> Vec<GolombRuler> {
        (start_idx..end_idx).map(GolombRuler::from_id).collect::<Vec<GolombRuler>>()
    }


    fn to_state(&self) -> Vec<bool> {

        let l = self.length();

        if self.order() == 1 || self.length() == 1 {
            return vec![]
        }


        // if self.
        // if self.length() == 0 {
        //     return vec![]
        // }

        // if self.length()

        // Initialize with all false, dropping the 0 and length
        let mut state = vec![false; (l - 1) as usize];

        // Drop the 0
        for m in &self.marks[0..self.marks.len() - 1] {
            state[*m as usize - 1] = true;
        }

        state
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

/// Simple trait to convert a vector of booleans to a u64
trait RulerState {
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
    fn next_with_order(&self, order: usize, length: usize) -> Option<Vec<bool>>;

}

const TWO: u64 = 2;

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

    /// Improved implementation of `next` by pruning trees that have too many points
    fn next_pruned(&self, order: usize, length: usize) -> Option<Vec<bool>> {

        if self.len() > length - 1 {
            return None
        }

        if self.len() < length - 1 {

            // Always 'go to the left'
            // println!("Trying to go left!");
            let mut left = self.go_left();
            // println!("left: {:?}", left);
            while left.len() < length - 1 {
                println!("left: {:?}", left);
                left = left.go_left();
            }
            Some(left)

        } else {

            // If our final element is 0, then we simply bounce back one and go to the right
            if !self[self.len() - 1] {


                // If we have too many marks, we can't continue to the right.
                let n_marks = self.total_marks();
                if n_marks == order {

                    // If our vector starts with 1, then we are totally done.
                    if self[0] {
                        return None
                    } else {
                        return Some(self.jump_back())
                    }
                }

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

    fn next_with_order(&self, order: usize, length: usize) -> Option<Vec<bool>> {

        // We basically want to navigate along the tree until we have a node with order <= order.
        // our normal navifation method is perefectly fine, the only difference is we want to stop
        // exploring nodes once they have too many marks.
        let mut next: Vec<bool>;

        // If i currenly have too many nodes then I should backtrack!




        todo!()
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
        for i in (0..out.len()).rev() {
            if out[i] {
                out[i] = false;
                out[i - 1] = true;
                break
            }
        }

        out
    }

}

impl Iterator for RulerIterator {
    type Item = GolombRuler;

    fn next(&mut self) -> Option<Self::Item> {

        let next_state = self.state.next(self.length);
        self.state = next_state?;
        Some(self.state.to_ruler())
    }
}

impl Iterator for GolombRulerPrunedIterator {
    type Item = GolombRuler;

    fn next(&mut self) -> Option<Self::Item> {

        let next_state = self.state.next_pruned(self.order, self.length);
        self.state = next_state?;
        Some(self.state.to_ruler())
    }
}


fn count_permutations(n: GInt, r: GInt) -> GInt {
    (n - r + 1..=n).product()
}


/* -------------------------------------------------------------------------- */
/*                              Public functions                              */
/* -------------------------------------------------------------------------- */
#[pymethods]
impl Ruler {

    /// Create a new ruler from a given id.
    #[staticmethod]
    #[pyo3(text_signature = "(id: int)")]
    fn from_id(id: usize) -> GolombRuler {
        GolombRuler::from_id(id)
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {

        let my_bool: Vec<bool> = vec![true, true, false];
        dbg!(my_bool.to_u64());
        assert_eq!(my_bool.count_marks(), 2);
        dbg!(my_bool.to_string());

    }


    #[test]
    fn test_enum() {

        // let my_bool: Vec<bool> = vec![true, true, false];
        // dbg!(my_bool.to_u64());
        // assert_eq!(my_bool.count_marks(), 2);
        // dbg!(my_bool.to_string());

        // dbg!(my_bool.to_ruler());


        // dbg!(enumerate_golomb_rulers(3, 4));

        let order = 3;

        // let len_1 = enumerate_golomb_rulers(order, 1);

        // dbg!(len_1);

        // let len_2 = enumerate_golomb_rulers(order, 2);
        // dbg!(len_2);

        // let len_3 = enumerate_golomb_rulers(order, 3);
        // dbg!(len_3);


        let r_o3_l3 = enumerate_golomb_rulers(2, 3);

        for r in r_o3_l3 {
            println!("{}", r);
            // dbg!(r.distances());
        }

        let rulers_2_3 = enumerate_rulers(3);
        for r in rulers_2_3 {
            println!("{}", r);
        }


        let r2 = enumerate_rulers_with_length(2);
        println!("========= Length 2 ===========");
        for r in r2 {
            println!("{}", r);
        }


        // let r_o2_3 = enumerate_golomb_rulers(2, 3);
        let r3 = enumerate_rulers_with_length(3);
        println!("========= Length 3 ===========");
        for r in r3 {
            println!("{}", r);
        }

        let r4 = enumerate_rulers_with_length(4);
        println!("========= Length 4 ===========");
        for r in r4 {
            println!("{}", r);
        }


        println!("========= G 3_10 ===========");
        let g4 = enumerate_golomb_rulers(3, 10);
        for r in g4 {
            println!("{}", r)
        }

        println!("======== Ruler 3_10 ========");
        let r_3_10 = enumerate_rulers_with_order(3, 10);
        for r in r_3_10.iter() {
            println!("{}", r);
        }

        // println!("Num rulers: {}", r_3_10.len());
        // println!("10 choose 3: {}", count_permutations(10, 3));

        // println

        // for r in r_o2_3 {
        //     println!("{}", r);
        //     dbg!(r.distances());
        // }


        // For a single length, let's count the number of values at each order

        let mut lengths: Vec<(usize, usize)> = Vec::new();
        let order_range = 2..10;
        for order in order_range.clone() {
            lengths.push((order, enumerate_golomb_rulers(order, 20).len()));
        }


        for r in lengths {
            println!("{:?}", r)
        }
        // dbg!(lengths);

    }

    /// Verify that all rulers are being enumerated
    #[test]
    fn gen_rulers() {

        let length = 10;
        let rulers = enumerate_rulers_with_length(length);

        let rulers: Vec<&GolombRuler> = rulers.iter().filter(|r| r.is_golomb_ruler()).collect();


        for r in &rulers {
            println!("{}", r)
        }
        println!("N rulers: {}", rulers.len());

    }

    #[test]
    fn gen_pruned() {

        let length = 5;
        let order = 3;
        let rulers = enumerate_pruned_rulers(order, length);

        for r in rulers {
            println!("{}", r);
        }

        let iter = GolombRulerPrunedIterator::new(order, length);



        dbg!(iter.take(1).collect::<Vec<GolombRuler>>());


    }

    #[test]
    fn test_jump_back() {

        let test: Vec<bool> = vec![false, true, false];

        dbg!(&test);
        dbg!(test.jump_back());


    }

    #[test]
    fn test_ruler() {

        let rulers_3 = enumerate_rulers_with_length(3);
        for r in rulers_3 {
            println!("{}", r);
        }

    }


    #[test]
    fn test_ids() {

        let rulers_3 = enumerate_rulers(3);
        for r in rulers_3 {
            println!("[{:?}] Ruler: {};", r.to_id().unwrap(), r);
        }
    }

    #[test]
    fn test_from_id() {

        let first = GolombRuler::from_id(0);
        println!("{}", first);

        let second = GolombRuler::from_id(1);
        println!("{}", second);

        let third = GolombRuler::from_id(2);
        println!("{}", third);

        let third = GolombRuler::from_id(3);
        println!("{}", third);

        let fourth = GolombRuler::from_id(4);
        println!("{}", fourth);

        // let test = GolombRuler::from_id(1000234);
        // println!("{}", test);

        for i in 0..10 {
            println!("[{:02}] {}", i, GolombRuler::from_id(i));
        }

    }

    #[test]
    fn sample_rulers() {

        let max_range = 100000;

        let rulers = (0..max_range).map(GolombRuler::from_id).collect::<Vec<GolombRuler>>();
        let rulers_is_golom: Vec<(&GolombRuler, bool)> = rulers.iter().map(|r| (r, r.is_golomb_ruler())).collect();

        let mut golomb_rulers: Vec<usize> = Vec::new();

        for (idx, pair) in rulers_is_golom.iter().enumerate() {
            println!("{}, {:?}", idx, pair);
            if pair.1 {
                golomb_rulers.push(idx)
            }
        }

        println!("Only {} golomb rulers out of {}!", golomb_rulers.len(), max_range);

        println!("Golomb Rulers: {:?}", golomb_rulers);

    }

    #[test]
    fn stats() {

        let max_range = 100;
        let rulers = GolombRuler::from_ids(0, max_range);
        let golomb_rulers = rulers.iter().filter(|r| r.is_golomb_ruler()).collect::<Vec<&GolombRuler>>();

        // Find the average order?
        // Actually I just want the _distribution_ of heights.

        println!("Num golomb rulers {}/{}", golomb_rulers.len(), max_range);
        println!("{:?}", golomb_rulers);

        for g in golomb_rulers {
            println!("g: {}, order: {}", &g, g.order());
        }



    }



}