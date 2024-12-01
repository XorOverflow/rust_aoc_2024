/*
https://adventofcode.com/2024/day/1
--- Day 1: Historian Hysteria ---
 */
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

    let diffs = zip(list_a, list_b).map(|(a, b)| (a - b).abs());
    let total_distance: i32 = diffs.sum();

    print!("total = {:?}", total_distance);
}
