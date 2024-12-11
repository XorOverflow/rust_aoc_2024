/*
https://adventofcode.com/2024/day/11
--- Day 11: Plutonian Pebbles ---
 */

use num::Integer;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

// Shoehorning functional maps/iter is still not working.
// just use a simple imperative manual vec construction.

//// // perform one iteration on a stone list from the rules
//// fn blink(input: &Vec::<usize>) -> usize {
////
////     // Use map() alone doesn't handle the case where one elements
////     // maps into 2, so flat_map() can help.
////
////     input.iter()
////         .flat_map(|&v|
////                   if v == 0 {
////                       std::iter::once(1)
////                   } else {
////                       let digits = format!("{v}");
////                       let l:usize = digits.len();
////                       if l.is_even() {
////                           let (one,two) = digits.split_at(l/2);
////                           let pair: [usize;2] = [usize::from_str(one).unwrap(),
////                                                  usize::from_str(two).unwrap()];
////                           pair.iter()
////                       } else {
////                           std::iter::once(v * 2024)
////                     // incompatible iter() types returned pair != once.
////                       }
////                   })
////         .collect()
//// }

// Perform one step of transformation
fn blink(input: &Vec<usize>) -> Vec<usize> {
    let mut result = Vec::<usize>::with_capacity(input.len());
    for v in input {
        let v = *v;
        if v == 0 {
            result.push(1);
        } else {
            let digits = format!("{v}");
            let l: usize = digits.len();
            if l.is_even() {
                let (one, two) = digits.split_at(l / 2);
                let (one, two) = (usize::from_str(one).unwrap(), usize::from_str(two).unwrap());
                result.push(one);
                result.push(two);
            } else {
                result.push(v * 2024);
            }
        }
    }

    result
}

fn count_1(input: &Vec<usize>) -> usize {
    let mut result = input.clone();
    for _ in 0..25 {
        result = blink(&result);
    }

    //eprintln!("Final blinked = {:?}", result);
    result.len()
}

// Process explodes in space and time and must be optimized.
// Most values will cycle and reuse the same digits (0 -> 1 -> 2024 -> 20|24 -> 2|0|2|4 -> 4096|1|....)
// Once one has been computed up to some iteration, its expanded size can be simply added to the others
// to get the size of a source value.

// A named pair of one value appearing in a particular expansion level.
// for exemple if expanding "0" to 4 blinks we have "2 0 2 4",
// there is a valueCount {value:2 count:2} and {value:0 count:1}
// (Fixme is there a "counted set" in rust std ?)
#[derive(Clone)]
struct ValueCount {
    value: usize,
    count: usize,
}

// The complete history of one value expanded to some iteration level.
// for "0", ValueExpansions[0] = simply (1, [ {value: 0 count:1} ])
//          ValueExpansions[1] =  (1, [ {value: 1 count:1} ])
//          ValueExpansions[2] =  (1, [ {value: 2024 count:1} ])
//          ValueExpansions[3] =  (2, [ {value: 20 count:1}, {value: 24 count:1} ])
//          ValueExpansions[4] =  (4, [ {value: 2 count:2}, {value: 0, count 1} {value: 4 count:1} ])
type ValueExpansions = Vec<(usize, Vec<ValueCount>)>;

// helper function to keep elements counts
fn increase_count(map: &mut HashMap<usize, usize>, val: usize, count: usize) {
    if let Some(c) = map.get_mut(&val) {
        *c += count;
    } else {
        map.insert(val, count);
    }
}

fn expand_value_at_level(
    v: usize,
    level: usize,
    expansions: &mut HashMap<usize, ValueExpansions>,
) -> usize {
    // If first time, create the trivial "level 0" of just itself of size 1.
    let exp = expansions.entry(v).or_insert({
        let mut x = ValueExpansions::new();
        x.push((1, vec![ValueCount { value: v, count: 1 }]));
        x
    });

    // We already memoized this value's expansion up to this nth blink level
    if exp.len() > level {
        let (size, _): (usize, Vec<ValueCount>) = exp[level];
        return size;
    }

    //  recursive strategies:
    // * get the expansion components at the previous level, and ask for
    // those components expansion at level "1"
    // * get the expansion components at the level 1, and ask for
    // those components expansion at level "level - 1"
    // * Get hight expansion level known  "n", and ask for their
    // component expansion at level "level - n". We build our own level "n+1".

    // We do this 3rd one:

    let highest = exp.len() - 1;
    let (_, components): &(usize, Vec<ValueCount>) = &exp[highest];

    let mut follow = Vec::<usize>::new();

    let mut expansion_size = 0;
    let mut expansion_components_count = HashMap::<usize, usize>::new();

    for vcount in components {
        // For each unique item in the list of components at this level,
        // we compute (once) its next iteration, and count the duplicated copies
        // created by the number of occurences on this level.
        // Different items may create identlical next-iteration so they must be
        // counted globally.
        let v = vcount.value;
        if v == 0 {
            let new_val = 1;
            follow.push(new_val);
            expansion_size += vcount.count;
            increase_count(&mut expansion_components_count, new_val, vcount.count);
        } else {
            let digits = format!("{v}");
            let l: usize = digits.len();
            if l.is_even() {
                let (one, two) = digits.split_at(l / 2);
                let (one, two) = (usize::from_str(one).unwrap(), usize::from_str(two).unwrap());
                follow.push(one);
                follow.push(two);
                expansion_size += 2 * vcount.count;
                increase_count(&mut expansion_components_count, one, vcount.count);
                increase_count(&mut expansion_components_count, two, vcount.count);
            } else {
                let new_val = v * 2024;
                follow.push(new_val);
                expansion_size += vcount.count;
                increase_count(&mut expansion_components_count, new_val, vcount.count);
            }
        }
    }

    let expansion_components: Vec<ValueCount> = expansion_components_count
        .into_iter()
        .map(|(value, count)| ValueCount { value, count })
        .collect();

    // We update now the global HashMap with our own new level.
    // This ensure we makes progress before recursing into the (level-n) step of the sub-components,
    // which could call back to our own value at some point.

    let next_level_expansion = (expansion_size, expansion_components.clone());

    // This exp is a mut reference inside the map, updated for other recursive calls.
    exp.push(next_level_expansion);

    let highest = highest + 1;
    //eprintln!("Computing new intermediate value: {v} at level {highest} is expansion size {expansion_size}");

    // We now know our full "level N" size and expansion, we were asked for "level LEVEL".
    // Iterate (again) on the expanded components and ask for their size at iteration "LEVEL - N"
    // and sum them.
    // (Technically this will give us directly our size at level LEVEL, but NOT our full expansion
    // details, nor any intermediate level between N and LEVEL-1. So we can't simply update
    // the exp array.
    // However a partial memoization with only this result could be saved for improving a bit ?

    let delta_level = level - highest;
    let mut level_size: usize = 0;
    for vcount in expansion_components {
        level_size += vcount.count * expand_value_at_level(vcount.value, delta_level, expansions)
    }

    level_size
}

// This does not converge.
//fn count_2_brute_force(input: &Vec<usize>) -> usize {
//    let mut result = input.clone();
//    for _ in 0..75 {
//        result = blink(&result);
//    }
//
//    result.len()
//}

// Takes 2.0s for input (result is on the order of 259593838000000 )
fn count_2(input: &Vec<usize>) -> usize {
    let mut expansions = HashMap::<usize, ValueExpansions>::new();
    let mut size = 0;
    for k in input {
        size += expand_value_at_level(*k, 75, &mut expansions);
    }

    size
}

fn main() {
    let stdin = io::stdin();
    // There is only one line in the input.
    let line: String = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .expect("input should contain one line");

    let parsed: Vec<usize> = line
        .split(' ')
        .map(|s| usize::from_str(s).unwrap())
        .collect();

    println!("Part 1 = {}", count_1(&parsed));

    println!("Part 2 = {}", count_2(&parsed));
}
