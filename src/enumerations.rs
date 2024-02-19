//! Functions for enumerating all of the potential golomb rulers

use thiserror::Error;

use crate::Ruler;

#[derive(Error, Debug)]
pub enum GolombIterationError {
    #[error("The current golomb ruler index can't be contained in a u64")]
    IndexOverflow
}


/// Our ultimate goal is to create an enumerator that walks along all possible
/// golomb rulers of a certain size


/// Print out every possible golomb ruler of order `order`
fn enumerate_golomb_rulers(order: usize) {

}

// If we start with the smallest possible length, then the _very_ first ruler that we encounter
// is optional!

// Some facts about our object that we are working with:
// - there are an infinite number of golomb rulers
// - for a given length, there is only a certain number of golomb rulers possible


struct GolombRulerIterator {
    state: Vec<bool>,
    order: usize,
    length: usize
}

/// GolombRuler integer subtype
type GInt = u128;

/// 0-implied GolombRuler.
struct GolombRuler {
    marks: Vec<GInt>
}

/// Simple trait to convert a vector of booleans to a u64
trait RulerState {
    fn to_u64(&self) -> Option<u64>;
    /// Count the number of true values in this iterator state
    fn count_marks(&self) -> usize;
    /// Return the next state with max number of marks `order` and max length `length`
    fn next(&self, order: usize, length: usize) -> Option<Vec<bool>>;
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
        todo!()
    }

    fn next(&self, order: usize, length: usize) -> Option<Vec<bool>> {

        // Room to grab one more
        if self.len() < length {

            // Always 'go to the left'
            Some(self.go_left())

        } else {

            // If our final element is 0, then we simply bounce back one and go to the right
            if !self[self.len() - 1] {
                Some(self.back_one_then_right())
            } else {
                // We need to back track!
                Some(self.backtrack())
            }
        }
    }

}

impl Iterator for GolombRulerIterator {
    type Item = Ruler;

    fn next(&mut self) -> Option<Self::Item> {
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
}