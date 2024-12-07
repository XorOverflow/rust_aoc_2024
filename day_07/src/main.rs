/*
https://adventofcode.com/2024/day/7
--- Day 7: Bridge Repair ---
 */

use std::io;
use std::str::FromStr;

const DEBUG: bool = true;

#[derive(Debug)]
struct Equation {
    value: usize,
    operands: Vec<usize>,
}

// Basic brute-force method: test all possible combination of "+" and
// "*" between operands.  Input data contains at most 10 operands so
// binary combinations would be at most 1024 which seems manageable.
// On input.txt with 850 lines, this takes 0.17s

// operators_map is a binary encoding of the + and * to perform:
// first bit is 0 for a + and 1 for a * between first operands,
// second bits is the same for the second operator, etc.
fn compute_equation_with_map(operands: &Vec<usize>, mut operators_map: u32) -> usize {
    let mut result: usize = operands[0];
    for v in operands.iter().skip(1) {
        let bit = operators_map & 0b1;
        match bit {
            0 => result = result + v,
            1 => result = result * v,
            _ => panic!("impossible bit value"),
        }
        operators_map = operators_map >> 1;
    }

    result
}

fn can_solve(eq: &Equation) -> bool {
    // Get the number of operators between operands ( N - 1)
    // This is the number of bits to iterate over.
    // Set all bits to 1 as the max value for a for-loop by getting
    // the next power of 2, minus 1.
    let operators_map: u32 = (1 << (eq.operands.len())) - 1;

    for k in 0..=operators_map {
        if compute_equation_with_map(&eq.operands, k) == eq.value {
            if DEBUG {
                eprintln!("Solved {:?} with operators map {:b}", eq, k);
            }
            return true;
        }
    }
    if DEBUG {
        eprintln!("Cannot solve {:?}", eq);
    }
    // No found combination
    false
}

fn sum_total_calibration(input: &Vec<Equation>) -> usize {
    input
        .iter()
        .filter(|e| can_solve(e))
        .fold(0_usize, |a, e| a + e.value)
}

// Part 2 is a slight variation of part 1.
// Code has been duplicated to keep the history of part 1 solving code.
// brute forcing is not twice as long but SQUARED as long.
// By skipping easy-to-solve equations from part 1, this still
// takes 2 minutes 17s (instead of 0.1s)

// One source of slowness is that each combination recomputes everything
// instead of using recursive call with tree exploration and keeping
// the start of the equation already computed.

fn concat_digits(a: usize, b: usize) -> usize {
    // Avoid the obvious hack of re-parsing the string comming from format!() both numbers.
    let digits = b.ilog10() + 1;
    let r = a * 10_usize.pow(digits) + b;

    //eprintln!("Concat {a} || {b} => {r}");
    r
}

// operators_map is now encoding +, * or "||", using 2 bits instead of 1.
// To avoid tedious generation of only valid bits, we map two different
// bit value to the same || operator when iterating over all possible u32
// But this makes the brute-forcing longer for nothing.
fn compute_equation_with_concat(operands: &Vec<usize>, mut operators_map: u32) -> usize {
    let mut result: usize = operands[0];
    for v in operands.iter().skip(1) {
        let bit = operators_map & 0b11;
        match bit {
            0b00 => result = result + v,
            0b01 => result = result * v,
            0b10 | 0b11 => result = concat_digits(result, *v),
            _ => panic!("impossible bit value"),
        }
        operators_map = operators_map >> 2;
    }

    result
}

fn can_solve_with_concat(eq: &Equation) -> bool {
    // First try the easy (fast) way of part 1
    let operators_map: u32 = (1 << (eq.operands.len())) - 1;
    for k in 0..=operators_map {
        if compute_equation_with_map(&eq.operands, k) == eq.value {
            return true;
        }
    }
    // If this fails, brute force more
    // Compared with part 1 we have twice as many bits.
    let operators_concat: u32 = (1 << 2 * (eq.operands.len())) - 1;
    for k in 0..=operators_concat {
        if compute_equation_with_concat(&eq.operands, k) == eq.value {
            if DEBUG {
                eprintln!("Solved by concat only {:?} with operators map {:b}", eq, k);
            }
            return true;
        }
    }

    if DEBUG {
        eprintln!("Cannot solve at all {:?}", eq);
    }
    // No found combination
    false
}

fn sum_total_with_concat(input: &Vec<Equation>) -> usize {
    input
        .iter()
        .filter(|e| can_solve_with_concat(e))
        .fold(0_usize, |a, e| a + e.value)
}

fn main() {
    let mut parsed = Vec::<Equation>::new();

    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Err(_) => {
                panic!("input error, exit");
            }
            Ok(0) => {
                break;
            }
            Ok(_) => {
                let input = input.trim();
                let (value, operands) = input.split_once(": ").unwrap();
                let value = usize::from_str(value).unwrap();
                let operands: Vec<usize> = operands
                    .split(' ')
                    .map(|i| usize::from_str(i).unwrap())
                    .collect();
                let equation = Equation { value, operands };
                parsed.push(equation);
            }
        }
        input = String::from("");
    }

    println!("Part 1 = {}", sum_total_calibration(&parsed));

    println!("Part 2 = {}", sum_total_with_concat(&parsed));
}
