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

fn count_2(input: &Vec<Equation>) -> usize {
    input[0].operands.len()
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

    println!("Part 2 = {}", count_2(&parsed));
}
