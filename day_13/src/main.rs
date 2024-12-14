/*
https://adventofcode.com/2024/day/13
--- Day 13: Claw Contraption ---
 */

use num::integer::div_rem;
use regex::Regex;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

const A_COST: isize = 3;
const B_COST: isize = 1;

#[derive(Clone, Debug)]
struct ClawMachine {
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
}

// Returns minimal cost to reache the prize, if possible
fn get_token_cost(claw: &ClawMachine) -> Option<isize> {
    // Find A and B positive integer such that:
    // prize.x = A * button_a.x + B * button_b.x
    // prize.y = A * button_a.y + B * button_b.y
    // and minimal (A * cost_A + B * cost_B)

    // Linear equations system:
    // A (ax by - ay bx) = by Px - bx Py
    // B (ax by - ay bx) = ax Py - ay Px

    // if integer divisible:
    // A = (byPx - bxPy) / (ax by - ay bx)
    // B = (axPy - ayPx) / (ax by - ay bx)

    // There is only 1 solution (coordinates into a 2D vector base
    // of (button_a, button_b) so no need to solve any
    // additional "minimal cost" constraint.

    let ax = claw.button_a.0;
    let ay = claw.button_a.1;
    let bx = claw.button_b.0;
    let by = claw.button_b.1;
    let px = claw.prize.0;
    let py = claw.prize.1;

    let factor = ax * by - ay * bx;
    let a_numerator = by * px - bx * py;
    let b_numerator = ax * py - ay * px;

    let a_n = div_rem(a_numerator, factor);
    let b_n = div_rem(b_numerator, factor);

    // Remainder must be 0
    if a_n.1 != 0 || b_n.1 != 0 {
        //eprintln!("Claw {:?} has no integer result: {:?}, {:?}", claw, a_n, b_n);
        return None;
    }

    if a_n.0 < 0 || b_n.0 < 0 {
        // This should not happen
        eprintln!("Claw {:?} has negative result", claw);
        return None;
    }

    Some(a_n.0 * A_COST + b_n.0 * B_COST)
}

fn minimal_tokens_for_prizes(machines: &Vec<ClawMachine>) -> isize {
    let mut tokens = 0;
    for m in machines {
        if let Some(t) = get_token_cost(m) {
            tokens += t;
        }
    }

    tokens
}

fn minimal_tokens_for_prizes_mega(machines: &Vec<ClawMachine>) -> isize {
    let mut tokens = 0;
    for m in machines {
        let mut mega = m.clone();
        mega.prize.0 += 10000000000000;
        mega.prize.1 += 10000000000000;
        if let Some(t) = get_token_cost(&mega) {
            tokens += t;
        }
    }

    tokens
}

fn main() {
    // This regex covers both the Button and the Prize subtly different formats
    let re_xy = Regex::new(r".*: X.([0-9]+), Y.([0-9]+)").unwrap();

    let lines = io::stdin().lock().lines();

    // Parse all lines with the re_xy (if matching), without
    // bothering with grouping by 3 (blank lines will be skipped)
    let xy_iter = lines.filter_map(|line /*: Option<io::Result<String>>*/| {
        let line = line.ok()?;
        let caps = re_xy.captures(&line)?;
        let x = isize::from_str(caps.get(1).unwrap().as_str()).unwrap();
        let y = isize::from_str(caps.get(2).unwrap().as_str()).unwrap();
        Some((x, y))
    });

    // There is no .chunks() on iterators, only slices.
    // (.array_chunks() is still in Experimental)
    // So just collect(), no need to optimize for lazy evaluation anyways.
    let xy: Vec<(isize, isize)> = xy_iter.collect();
    let claw_machines: Vec<ClawMachine> = xy
        .chunks_exact(3)
        .map(|t| ClawMachine {
            button_a: t[0],
            button_b: t[1],
            prize: t[2],
        })
        .collect();

    //eprintln!("claw machines are: {:?}", claw_machines);

    // Small sanity-check of the input.
    // No button has a 0 displacement value, which avoids checks for divisions by 0.
    for c in &claw_machines {
        if c.button_a.0 == 0 || c.button_a.1 == 0 || c.button_b.0 == 0 || c.button_b.1 == 0 {
            eprintln!("Warning: {:?} has a 0 vector", c);
        }
    }

    // Part 1
    println!("Part 1 = {}", minimal_tokens_for_prizes(&claw_machines));

    // Part 2
    println!("Part 2 = {}", minimal_tokens_for_prizes_mega(&claw_machines));
}
