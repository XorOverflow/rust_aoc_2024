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

fn get_2000_prices(seed: usize) -> Vec<usize> {
    let mut r = Vec::<usize>::with_capacity(2000);
    let mut i = seed;
    for _ in 0..2000 {
        let price = i % 10;
        r.push(price);
        i = iter_pseudorand(i);
    }

    r
}

fn get_changes(secrets: &Vec<usize>) -> Vec<isize> {
    secrets
        .windows(2)
        .map(|w| w[1] as isize - w[0] as isize)
        .collect()
}

fn get_sequences(changes: &Vec<isize>) -> Vec<[isize; 4]> {
    // Disapointingly, windows(4).collect() or even map(|w| *w)
    // gets a compilation error with "size not known at compile time"
    // despite windows(4) being clearly known as a &[isize;4] and not just a &[isize]
    changes
        .windows(4)
        .map(|w| [w[0], w[1], w[2], w[3]])
        .collect()
}

fn find_best_common_sequence(buyers: &Vec<usize>) -> usize {
    let buyers_prices: Vec<Vec<usize>> = buyers.iter().map(|b| get_2000_prices(*b)).collect();
    let buyers_changes: Vec<Vec<isize>> = buyers_prices
        .iter()
        .map(|prices| get_changes(prices))
        .collect();
    let buyers_sequences: Vec<Vec<[isize; 4]>> = buyers_changes
        .iter()
        .map(|changes| get_sequences(changes))
        .collect();
    /* XXX FIXME TODO */

    42
}

fn main() {
    let mut buyer_secrets = Vec::<usize>::new();

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        let secret = usize::from_str(&line).unwrap();
        buyer_secrets.push(secret);
    }

    println!("Part 1 = {}", sum_2000_secrets(&buyer_secrets));
    println!("Part 2 = {}", find_best_common_sequence(&buyer_secrets));
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
