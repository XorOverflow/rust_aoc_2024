/*
https://adventofcode.com/2024/day/23
--- Day 23: LAN Party ---
*/

use aoc::grid::Grid;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

// The network map will be modeled as a NxN symetrical matrix,
// where the [n,m] elements is true if computer n is linked
// with computer m. (Adjacency matrix)

fn indices_of_t_computers(computers: &HashMap<String, usize>) -> Vec<usize> {
    computers
        .iter()
        .filter(|(k, _v)| k.starts_with('t'))
        .map(|(_k, v)| *v)
        .collect()
}

// Get list of 3-computers all connected to each-others.
// Returns tuples (a,b,c) sorted in the way that a<b<c for unicity
fn tuples_of_3_computers(matrix: &Grid<bool>) -> Vec<(usize, usize, usize)> {
    // a,b,c where m[a,b] = m[a,c] = m[b,c] = true
    // FIXME: Is there some magical matrix operation to find it directly ?
    // In graph theory, M^n gives the number of indirect connection of length n
    // between two nodes, for example. Does not seem to apply here.

    assert_eq!(matrix.width, matrix.height);
    let m = matrix.width;
    let mut result = Vec::<(usize, usize, usize)>::new();

    // O(n^3) algo...
    for a in 0..m - 2 {
        for b in a + 1..m - 1 {
            if !matrix.get(a, b) {
                continue;
            }
            for c in b + 1..m {
                if !matrix.get(a, c) || !matrix.get(b, c) {
                    continue;
                }
                result.push((a, b, c));
            }
        }
    }

    result
}

fn main() {
    let mut computers = HashMap::<String, usize>::new();
    let mut computers_names = Vec::<String>::new(); // reverse of hash
    let mut netmap = Vec::<(usize, usize)>::new();

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        let (a, b) = line.split_once("-").unwrap();
        let (a, b) = (a.to_string(), b.to_string());
        let a_idx;
        let b_idx;
        // nightly try_insert() would be simpler here.
        if let Some(ai) = computers.get(&a) {
            a_idx = *ai;
        } else {
            a_idx = computers_names.len();
            computers_names.push(a.clone());
            computers.insert(a, a_idx);
        }
        // b
        if let Some(bi) = computers.get(&b) {
            b_idx = *bi;
        } else {
            b_idx = computers_names.len();
            computers_names.push(b.clone());
            computers.insert(b, b_idx);
        }

        netmap.push((a_idx, b_idx));
    }

    let mut matrix = Grid::<bool>::new(computers.len(), computers.len(), false);
    for (a, b) in netmap {
        matrix.set(a, b, true);
        matrix.set(b, a, true);
    }
    // This is a very sparse matrix, not sure if it's more efficient
    // than just comparing a linear list...

    matrix.pretty_print_bool();

    let t_computers = indices_of_t_computers(&computers);
    let triplets = tuples_of_3_computers(&matrix);

    let mut count_triplet_with_t = 0;
    // The list will be sorted by the arbitrary internal
    // numeric index, not alphabetically like in the sample.
    for k in &triplets {
        let triplet = format!(
            "{},{},{}",
            computers_names[k.0], computers_names[k.1], computers_names[k.2],
        );
        // we COULD do it just by triplet.starts_with('t') || triplets.contains(",t") ...
        if t_computers.contains(&k.0) || t_computers.contains(&k.1) || t_computers.contains(&k.2) {
            count_triplet_with_t += 1;
            eprintln!("Found {}", triplet);
        }
    }

    println!("Part 1 = {count_triplet_with_t}");
}
