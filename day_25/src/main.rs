/*
https://adventofcode.com/2024/day/25
--- Day 25: Code Chronicle ---
"This is Lockpicking Lawyer"
 */

use std::io;
use std::io::prelude::*;

#[derive(Clone, Debug)]
struct Pins {
    height: [isize; 5],
}

fn check_fit(key: &Pins, lock: &Pins) -> bool {
    for i in 0..5 {
        if key.height[i] + lock.height[i] > 5 {
            return false;
        }
    }

    true
}

fn count_fitting_pairs(keys: &Vec<Pins>, locks: &Vec<Pins>) -> usize {
    let mut pairs = 0;

    for k in keys {
        for l in locks {
            if check_fit(k, l) {
                pairs += 1;
                eprintln!("Key {:?} fits in lock {:?}", k, l);
            }
        }
    }

    pairs
}

// Parse stdio and returns a vec of keys
// and a vec of locks.
fn parse_input() -> (Vec<Pins>, Vec<Pins>) {
    let mut locks = Vec::<Pins>::new();
    let mut keys = Vec::<Pins>::new();

    let mut lines = io::stdin().lock().lines();

    let mut parsed_pins = Pins { height: [0; 5] };
    let mut is_lock: Option<bool> = None;

    while let Some(Ok(line)) = lines.next() {
        // Blank line separator
        if line.len() == 0 {
            match is_lock {
                Some(true) => locks.push(parsed_pins.clone()),
                Some(false) => keys.push(parsed_pins.clone()),
                None => (),
            }
            is_lock = None;
            continue;
        }

        // First line of new entry
        if is_lock == None {
            // First line of a lock is always
            // full of #
            if line == "#####" {
                is_lock = Some(true);
                parsed_pins.height = [-1; 5];
                // We will increase when seeing a #
            } else {
                is_lock = Some(false);
                parsed_pins.height = [6; 5];
                // We will decrease when seeing a .
            }
        }

        for (i, c) in line.chars().enumerate() {
            if is_lock == Some(true) {
                if c == '#' {
                    parsed_pins.height[i] += 1;
                }
            } else {
                if c == '.' {
                    parsed_pins.height[i] -= 1;
                }
            }
        }
    }

    // Don't forget last block at eof without a separator line
    match is_lock {
        Some(true) => locks.push(parsed_pins.clone()),
        Some(false) => keys.push(parsed_pins.clone()),
        None => (),
    }

    (keys, locks)
}

fn main() {
    let (keys, locks) = parse_input();

    eprintln!("Parsed locks: {:?}", locks);
    eprintln!("Parsed keyss: {:?}", keys);

    println!("Part 1 = {}", count_fitting_pairs(&keys, &locks));
}
