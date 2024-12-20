/*
https://adventofcode.com/2024/day/19
--- Day 19: Linen Layout ---

 */

use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

// Naive search (with early pruning when prefix doesn't match) takes too
// long when redoing every matching test when backtracking.
// Memoization is required to avoid redoing the same thing over and over
// on disjoint parts of the string;
// For this problem, this is enough to get the answer in 0.8s.

// If this was still too long, it would be possible to split
// the search into smaller parts and get the final by combining them
// at the end:
// Since we don't care about the details, and matching of one half
// doesn't depend on the matching of the other half as long
// as they split at the same point, counting can be done
// almost independantly on multiple sub-parts and merged
// at the end by a simple multiplication.
// If two disjoint parts A and B have M and N possible combinations,
// then AB have at least M*N combinations. It also have additional ones
// where some "towel pattern" is crossing over from the end of A to the
// start of B; they would be counted on a different split
// A'+B' (A longer and B shorter) where the first combinations over A'
// do not exactly line up to the boundary of A.
// As the towel patterns have a max size (for example 6) the max A'
// extension required would be A+1 to A+6  to cover all possible
// cases.

// Return the total number of combinations of strings from substr[]
// that concatenate exactly into "p".
// This is the number of leaves of the tree where each node branch
// is one element of substr, where the path concat equals to p.
// Rust: hashmap of &str requires lifetime consistency. Would be simpler
// with HashMap<String> with duplicated data; here we know that we always reference
// slices of our starting string so lifetime annotation is manageable.
fn combination_count<'a>(p: &'a str, substr: &[&str], memo: &mut HashMap<&'a str, usize>) -> usize {
    if p.len() == 0 {
        // Leaf found, the stack reaching it equals 1 possible
        // combination
        return 1;
    }

    if let Some(c) = memo.get(p) {
        return *c;
    }

    let mut count = 0;
    for s in substr {
        if let Some(sub_p) = p.strip_prefix(s) {
            count += combination_count(sub_p, substr, memo);
        }
    }

    memo.insert(p, count);
    count
}

fn count_all_possible_combinations(p: &Vec<String>, substr: &Vec<String>) -> usize {
    let substr: Vec<&str> = substr.iter().map(|s| s.as_str()).collect();
    let mut count = 0;
    for pat in p {
        let mut memo = HashMap::<&str, usize>::new();
        let single_count = combination_count(pat, &substr, &mut memo);
        eprintln!("{pat} has {single_count} combinations");
        count += single_count;
    }

    count
}

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

    println!("Part 1 = {:?}", matching);

    println!(
        "Part 2 = {:?}",
        count_all_possible_combinations(&patterns, &towels)
    );
}
