/*
https://adventofcode.com/2024/day/1
--- Day 1: Historian Hysteria ---
 */
use std::collections::HashMap;
use std::io;
use std::iter::zip;
use std::str::FromStr;

fn main() {
    // Construct the two lists of location IDs.
    // Computing the difference cannot be done on the fly because lists
    // must be sorted first.
    let mut list_a = Vec::<i32>::new();
    let mut list_b = Vec::<i32>::new();

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
                // Yes for some reason the puzzle input uses 3 spaces as separator.
                let ids: Vec<i32> = input_clean
                    .split("   ")
                    .map(|i| i32::from_str(i).unwrap())
                    .collect();
                list_a.push(ids[0]);
                list_b.push(ids[1]);
            }
        }
        // must clear for next loop
        input = String::from("");
    }

    list_a.sort();
    list_b.sort();

    // Count the occurence of each unique "location ids" in each list
    let mut count_a: HashMap<i32, usize> = HashMap::new();
    for x in &list_a {
        *count_a.entry(*x).or_default() += 1;
    }

    let mut count_b: HashMap<i32, usize> = HashMap::new();
    for x in &list_b {
        *count_b.entry(*x).or_default() += 1;
    }

    let diffs = zip(list_a, list_b).map(|(a, b)| (a - b).abs());
    let total_distance: i32 = diffs.sum();

    println!("total difference = {total_distance}");

    // Could be done with a 1-liner fold() but too unreadable with
    // all the necessary type conversion
    let mut score: i64 = 0;
    for (k, v) in count_a.into_iter() {
        let m1: i64 = (k as i64) * (v as i64);
        let m2: i64 = *count_b.entry(k).or_default() as i64;
        score += m1 * m2;
    }

    println!("Similarity score = {score}");
}
