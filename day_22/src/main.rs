/*
https://adventofcode.com/2024/day/22
--- Day 22: Monkey Market ---
*/

use std::io;
use std::io::prelude::*;
use std::str::FromStr;

// Perform 1 round of RNG
fn iter_pseudorand(n: usize) -> usize {
    let n = ((n * 64) ^ n) % 16777216;
    let n = ((n / 32) ^ n) % 16777216;
    let n = ((n * 2048) ^ n) % 16777216;

    n
}

fn sum_2000_secrets(buyer_secrets: &Vec<usize>) -> usize {
    let mut total = 0;
    for s in buyer_secrets {
        let mut i = *s;
        for _ in 0..2000 {
            i = iter_pseudorand(i);
        }
        eprintln!("2000th iter of {s} is {i}");
        total += i;
    }
    total
}

fn main() {
    let mut buyer_secrets = Vec::<usize>::new();

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        let secret = usize::from_str(&line).unwrap();
        buyer_secrets.push(secret);
    }

    println!("Part 1 = {}", sum_2000_secrets(&buyer_secrets));
}

#[test]
fn check_basic_rng() {
    let seed = 123;
    let secrets = vec![
        15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254,
    ];
    let mut s = seed;
    for k in secrets {
        s = iter_pseudorand(s);
        assert_eq!(s, k);
    }
}
