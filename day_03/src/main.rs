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

fn scan_muls_do_dont(inputs: &Vec<String>) -> u64 {
    // Add a capture group matching the conditional command.
    // Use named capture groups to distinguish the different cases.
    // We must use a single regex and not multiple, to be able to iterate
    // in order between the conditionals and the muls.
    let re = Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|mul\((?<arg1>[0-9]+),(?<arg2>[0-9]+)\)")
        .unwrap();

    let mut enabled: bool = true;
    let mut result: u64 = 0;

    for s in inputs {
        for cap in re.captures_iter(s) {
            if let Some(_) = cap.name("do") {
                enabled = true;
            } else if let Some(_) = cap.name("dont") {
                enabled = false;
            } else if enabled {
                if let Some(arg1) = cap.name("arg1") {
                    if let Some(arg2) = cap.name("arg2") {
                        let arg1 = arg1.as_str();
                        let arg2 = arg2.as_str();
                        //eprintln!("parsed enabled mul  {arg1} * {arg2}");
                        let arg1 = u64::from_str(arg1).unwrap();
                        let arg2 = u64::from_str(arg2).unwrap();
                        result += arg1 * arg2;
                    } else {
                        panic!("regex matched neither do, dont or arg2");
                    }
                } else {
                    panic!("regex matched neither do, dont or arg1");
                }
            }
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
    let mulsum: u64 = scan_muls_do_dont(&lines);
    println!("mul do/don't= {mulsum}");
}
