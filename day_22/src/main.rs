/*
https://adventofcode.com/2024/day/22
--- Day 22: Monkey Market ---
*/

use std::collections::{HashMap, HashSet};
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

/*
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
 */

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct SequenceAndPrice {
    seq: [isize; 4],
    price: usize,
}

// Return the list of 4-prices-change, along with their final actual price
fn get_sequences_and_prices(prices: &Vec<usize>) -> Vec<SequenceAndPrice> {
    prices
        .windows(5)
        .map(|w| {
            let p0 = w[0] as isize;
            let p1 = w[1] as isize;
            let p2 = w[2] as isize;
            let p3 = w[3] as isize;
            let p4 = w[4] as isize;
            SequenceAndPrice {
                seq: [p1 - p0, p2 - p1, p3 - p2, p4 - p3],
                price: w[4],
            }
        })
        .filter(|sp| sp.price != 0) // We don't care about 0 prices
        .collect()
}

// Bidding will stop at the first finding of a specific sequence on a buyer list.
// Next sequence from the same buyer with a different price will be ignored, so purge them.
fn keep_first_sequence_occurences(sp: &Vec<SequenceAndPrice>) -> Vec<SequenceAndPrice> {
    let mut first_res = Vec::<SequenceAndPrice>::new();
    let mut seq_seen = HashSet::<[isize; 4]>::new();

    for s in sp {
        if seq_seen.contains(&s.seq) {
            continue;
        }
        seq_seen.insert(s.seq.clone());
        first_res.push(s.clone());
    }

    //eprintln!("Simplified buyer list from total sequences {} to first-seq only {}",
    //          sp.len(), first_res.len());
    first_res
}

fn find_best_common_sequence(buyers: &[usize]) -> usize {
    eprintln!("starting pre-processing");
    let buyers_prices: Vec<Vec<usize>> = buyers.iter().map(|b| get_2000_prices(*b)).collect();
    let buyers_sequences: Vec<Vec<SequenceAndPrice>> = buyers_prices
        .iter()
        .map(|prices| get_sequences_and_prices(prices))
        .collect();
    let buyers_sequences: Vec<Vec<SequenceAndPrice>> = buyers_sequences
        .iter()
        .map(|sp| keep_first_sequence_occurences(sp))
        .collect();

    /* XXX FIXME TODO */

    eprintln!("pre-processing done");
    let mut total_first_seq = 0;
    for bs in &buyers_sequences {
        total_first_seq += bs.len();
    }
    eprintln!("Filtered first-sequences count = {total_first_seq}");

    // There is no more than 18^4 different [isize;4] ranging from -9 to +9; actually a lot less
    // before their final sum from a starting 0..9 price must also be in 0..9, so any
    // [a,b,c,d] with a+b+c+d < -9  or > 9 are impossible.
    // anyway, the total map will be less then 18^4 = 104976, which is not too big.
    // Actual input processing generate exactly 9000 different sequences+prices.

    // Get a unique sample of each sequences from all buyers and all times

    /*
    This is only to get a sense of the scale of shared sequences among multiple buyers,
    during dev.
    Exactly 9000 sequences+prices for the input ? Strangely round value.

    let mut all_sequences: Vec<SequenceAndPrice> = buyers_sequences.into_iter().flatten().collect();

    eprintln!("sequence flattened done");

    all_sequences.sort();
    let mut unique_sequences = all_sequences.clone();
    unique_sequences.dedup();

    eprintln!("unique sequences: {}", unique_sequences.len());

     */

    // Lets put everyting into a hashmap with their total cumulated prices.

    // Hash to tuple of (number of buyers, total price). Number of buyers is just for debug.
    let mut cumulative_bidding = HashMap::<[isize; 4], (usize, usize)>::new();

    // Maintain the best result, no need to sort() the hashmap at the end.
    let mut max_price = 0;
    let mut max_sequence: [isize; 4] = [0, 0, 0, 0];
    let mut bidders = 0;

    for k in buyers_sequences.iter().flatten() {
        let count_price = cumulative_bidding.entry(k.seq).or_default();
        (*count_price).0 += 1;
        (*count_price).1 += k.price;
        if (*count_price).1 > max_price {
            max_price = (*count_price).1;
            max_sequence = k.seq;
            bidders = (*count_price).0;
        }
    }

    // This gives the correct result for official problem Input.
    // But not for the simpler sample !!

    eprintln!(
        "Max seq/price = {:?} => {max_price} ({bidders} different buyers)",
        max_sequence
    );

    max_price
}

fn main() {
    let mut buyer_secrets = Vec::<usize>::new();

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        let secret = usize::from_str(&line).unwrap();
        buyer_secrets.push(secret);
    }

    println!("Part 1 = {}", sum_2000_secrets(&buyer_secrets));
    // for dev speedup use only a subset of buyers
    //let buyer_secrets = &buyer_secrets[0..100];
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
