//! Implementations of different enumeration strategies to explore the Golomb Ruler space

use crate::GolombRuler;
use super::state::RulerState;


/// Iterator over all possible rulers with length `length`.
pub(super) struct RulerIterator {
    state: Vec<bool>,
    length: usize
}

/// Iterator that prunes the tree when the order has been hit or the golomb property is
#[derive(Debug)]
pub(super) struct GolombRulerPrunedIterator {
    state: Vec<bool>,
    order: usize,
    length: usize
}

/// Iterator that initially only checks the golomb property up until a certain depth
#[derive(Debug)]
pub(super) struct GolombRulerDepthIterator {
    state: Vec<bool>,
    order: usize,
    length: usize,
    depth: usize,
}

/* -------------------------------------------------------------------------- */
/*                             New Implementations                            */
/* -------------------------------------------------------------------------- */
impl GolombRulerPrunedIterator {
    pub(super) fn new(order: usize, length: usize) -> Self {
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

impl RulerIterator {
    pub(super) fn new(length: usize) -> Self {
        // The initial state should return our starting point on next()
        // this pre-state accomplishes that
        let pre_state = vec![false; length - 2];
        RulerIterator {
            state: pre_state,
            length,
        }
    }
}



impl GolombRulerDepthIterator {
    pub(super) fn new(order: usize, length: usize, depth: usize) -> Self {
        let pre_state = vec![false; length - 2];
        GolombRulerDepthIterator {
            state: pre_state,
            order,
            length,
            depth,
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                          Iterator implementations                          */
/* -------------------------------------------------------------------------- */
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

impl Iterator for GolombRulerDepthIterator {
    type Item = GolombRuler;

    fn next(&mut self) -> Option<Self::Item> {
        let next_state = self.state.next_golomb_depth_1(self.order, self.length);
        self.state = next_state?;
        Some(self.state.to_ruler())
    }
}