//! Functions for enumerating rulers based on order, length, and the Golomb property

mod state;
mod iterators;

use std::vec;
use pyo3::prelude::*;
use thiserror::Error;

use crate::GInt;
use crate::GolombRuler;

use iterators::*;

#[derive(Error, Debug)]
pub enum GolombIterationError {
    #[error("The current golomb ruler index can't be contained in a u64")]
    IndexOverflow
}


/// Exhaustively enumerate all rulers up to length `max_length`
///
/// # Arguments
/// * `max_length`: The maximum length
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

#[pyfunction]
pub fn enumerate_pruned_rulers(order: usize, length: usize) -> Vec<GolombRuler> {
    if order == 2 { return vec![GolombRuler::new(&[length as GInt])] }
    GolombRulerPrunedIterator::new(order, length).collect()
}

/// Get all rulers with `order` marks. They do not have to have the golomb property.
#[pyfunction]
pub fn enumerate_rulers_with_order(order: usize, length: usize) -> Vec<GolombRuler> {
    let rulers = enumerate_rulers(length);
    rulers.into_iter().filter(|r| r.order() == order).collect()
}

/// Print out every possible golomb ruler of order `order`
///
#[pyfunction]
pub fn enumerate_golomb_rulers(order: usize, max_length: usize) -> Vec<GolombRuler> {

    let all_rulers: Vec<GolombRuler> = (2..max_length + 1).flat_map(|length| {
        RulerIterator::new(length)
    }).collect();

    all_rulers.into_iter().filter(|r| r.order() == order && r.is_golomb_ruler()).collect()
}


#[pyfunction]
pub fn enumerate_golomb_rulers_with_length(order: usize, length: usize) -> Vec<GolombRuler> {
    let all_rulers: Vec<GolombRuler> = RulerIterator::new(length).collect();
    all_rulers.into_iter().filter(|r| r.order() == order && r.is_golomb_ruler() && r.length() == length as GInt).collect()
}

#[pyfunction]
pub fn enumerate_golomb_rulers_pruned(order: usize, max_length: usize) -> Vec<GolombRuler> {

    let all_rulers: Vec<GolombRuler> = (2..max_length + 1).flat_map(|length| {
        GolombRulerPrunedIterator::new(order, length)
    }).collect();

    // all_rulers
    // let filtered: Vec<GolombRuler> = all_rulers.into_iter().filter(|r| r.is_golomb_ruler()).collect();
    // for ruler in filtered.clone().into_iter() {
    //     dbg!(ruler.length());
    // }

    // filtered
    all_rulers.iter().filter(|g| g.is_golomb_ruler()).cloned().collect()
}

/// Enumerate all rulers with order `order` and length `length`
#[pyfunction]
pub fn enumerate_golomb_rulers_pruned_with_length(order: usize, length: usize) -> Vec<GolombRuler> {
    GolombRulerPrunedIterator::new(order, length).collect()
}

/// For initial enumeration, check the golomb property at a depth of `depth`.
#[pyfunction]
fn enumerate_golomb_rulers_depth(order: usize, max_length: usize, depth: usize) -> Vec<GolombRuler> {

    let all_rulers: Vec<GolombRuler> = (2..max_length + 1).flat_map(|length| {
        GolombRulerDepthIterator::new(order, length, depth)
    }).collect();

    all_rulers
}

fn enumerate_golomb_rulers_depth_with_length(order: usize, length: usize, depth: usize) -> Vec<GolombRuler> {
    GolombRulerDepthIterator::new(order, length, depth).collect()
}


/* -------------------------------------------------------------------------- */
/*                              Public functions                              */
/* -------------------------------------------------------------------------- */
#[cfg(test)]
mod tests {

    use super::*;
    use crate::enumeration::state::RulerState;

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
            lengths.push((order, enumerate_golomb_rulers(order, 15).len()));

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

        for r in &rulers {
            println!("{}", r);
        }

        // let iter = GolombRulerPrunedIterator::new(order, length);




        // dbg!(iter.take(1).collect::<Vec<GolombRuler>>());


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

        let max_range = 1000;

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

    /// Verify that our `contains` function for `Vec<bool>` works properly
    #[test]
    fn test_contains() {

        let ruler = GolombRuler::from_id(10);
        println!("Ruler: {}", ruler);

        let state = ruler.to_state();
        assert!(state.contains(0));
        assert!(state.contains(2));
        assert!(state.contains(4));
        assert!(!state.contains(5));
        assert!(!state.contains(10));
        assert!(!state.contains(200));
        assert!(!state.contains(-2));

    }

    #[test]
    fn test_propery_order_1() {

        let ruler = GolombRuler::from_id(10);
        println!("Ruler: {}", ruler);

        let r_09 = GolombRuler::from_id(9);
        println!("Ruler: {}", r_09);

        assert!(!ruler.is_golomb_ruler_order_1());
        assert!(r_09.is_golomb_ruler_order_1());

    }

    #[test]
    fn add_mark_unit() {

        let ruler = GolombRuler::from_id(22528);

        // dbg!(ruler.to_state());
        // dbg!(ruler.to_state().next_pruned(4, 15).unwrap());

        // println!("====");

        // dbg!(ruler.to_string());
        // dbg!(ruler.to_state().next_pruned(4, 15).unwrap().to_ruler().to_string());
    }

    #[test]
    fn add_mark() {

        // let ruler = GolombRuler::from_id(18432);
        // let ruler = GolombRuler::from_id(576);

        // println!("R.state: {:?}, r: {}", ruler.to_state(), ruler);
        // let next = ruler.to_state().next_pruned(4, 15).unwrap().to_ruler();

        // let np = ruler.to_state().next_pruned(4, 15).unwrap();
        // dbg!(np);

        // println!("R.state.next(): {:?}, r: {}", next.to_state(), next);

        let rulers = enumerate_golomb_rulers_pruned_with_length(4, 15);
        for r in rulers {
            println!("{}", r);
        }

    }

    #[test]
    fn more_enums() {

        use itertools::*;
        let order = 2;
        let length = 11;
        let depth = 1;

        let rulers = enumerate_rulers_with_length(length);
        println!("N rulers: {} length: {}", rulers.len(), length);

        let rulers_pruned = enumerate_golomb_rulers_pruned_with_length(order, length);
        println!("N pruned rulers: {} (order: {}, length: {})", rulers_pruned.len(), order, length);

        let ruler_pruned_g = rulers_pruned.iter().filter(|r| r.is_golomb_ruler()).collect_vec();

        let rulers_depth = enumerate_golomb_rulers_depth_with_length(order, length, depth);
        println!("N depth rulers: {} (order: {}, length: {})", rulers_depth.len(), order, length);

        let golomb_rulers = enumerate_golomb_rulers_with_length(order, length);
        println!("N golomb rulers: {} (order: {}, length: {})", golomb_rulers.len(), order, length);

        for r in &rulers_pruned {
            println!("{}", r);
        }

        println!("========");

        for r in rulers_depth {
            println!("{}", r);
        }

        println!("=======");

        for r in golomb_rulers {
            println!("{}", r);
        }

        println!("=======");

        for r in ruler_pruned_g {
            println!("{}", r);
        }


    }

    #[test]
    fn tnp() {
        // Test next pruned
        // Generate all rulers then filter
        use itertools::*;

        fn test_pruned(length: usize) {

            let all_rulers = enumerate_rulers_with_length(length);

            for order in 1..length {

                // Generate all rulers then filter
                let filtered = all_rulers.iter().filter(|r| r.order() == order).collect_vec();
                let pruned = enumerate_pruned_rulers(order, length);

                assert_eq!(filtered.len(), pruned.len())
            }
        }

        test_pruned(10);
        test_pruned(12);
        test_pruned(20);
    }

    #[test]
    fn timing_tnp() {

        enumerate_pruned_rulers(10, 20);
    }

    #[test]
    fn timing_all() {

        // enumerate_pruned_rulers(10, 20);
        enumerate_rulers_with_order(10, 20);
    }


}