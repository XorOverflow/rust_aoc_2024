/*
https://adventofcode.com/2024/day/19
--- Day 19: Linen Layout ---

 */

use regex::Regex;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut towels = Vec::<String>::new();
    let mut patterns = Vec::<String>::new();

    let mut lines = io::stdin().lock().lines();

    if let Some(Ok(line)) = lines.next() {
        towels = line.split(", ").map(|s| s.to_string()).collect();
    }
    lines.next(); // blank
    while let Some(Ok(line)) = lines.next() {
        patterns.push(line);
    }

    // Construct the regex match dynamically from the towels stripes
    // Something like "^(rg|wub|rr)*$".
    // We suppose that the input is simple letters and will not need
    // to validate input.
    let reg: String = "^(".to_string() + &towels.join("|") + ")*$";
    let re = Regex::new(&reg).unwrap();

    eprintln!("Matching with regex:");
    eprintln!("{reg}");

    let matching = patterns.iter().filter(|p| re.is_match(p)).count();

    eprintln!("Part 1 = {:?}", matching);
}
