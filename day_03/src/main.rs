/*
https://adventofcode.com/2024/day/3
--- Day 3: Mull It Over ---
 */

use regex::Regex;
use std::io;
use std::str::FromStr;

fn scan_muls(inputs: &Vec<String>) -> u64 {
    // Strictly speaking the regex crate matches any
    // unicode digits on \d, not just ascii 0-9, so be explicit.
    // We use Capture groups to return the two numbers arguments
    // directly.
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut result: u64 = 0;
    for s in inputs {
        for (_, [arg1, arg2]) in re.captures_iter(s).map(|c| c.extract()) {
            let arg1 = u64::from_str(arg1).unwrap();
            let arg2 = u64::from_str(arg2).unwrap();
            // eprintln!("parsed mul {arg1} * {arg2}");
            result += arg1 * arg2;
        }
    }

    result
}

fn main() {
    let mut lines = Vec::<String>::new();

    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Err(_) => {
                panic!("input error, exit");
            }
            Ok(0) => {
                eprintln!("Eof detected");
                break;
            }
            Ok(_) => {
                // remove the \n
                let input_clean = input.trim();
                lines.push(input_clean.to_string());
            }
        }
        // must clear for next loop
        input = String::from("");
    }

    // Part 1
    let mulsum: u64 = scan_muls(&lines);
    println!("mul = {mulsum}");

    // Part 2
}
