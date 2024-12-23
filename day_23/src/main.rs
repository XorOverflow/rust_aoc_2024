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

// Between two row of the adjacency matrix,
// return the number of elements (columns) where they differ.
fn get_row_distance<T: std::cmp::PartialEq>(a: &[T], b: &[T]) -> usize {
    let mut diff = 0;
    if a.len() != b.len() {
        panic!("Can't compare slices of different len");
    }

    for k in 0..a.len() {
        if a[k] != b[k] {
            diff += 1;
        }
    }

    diff
}

fn find_biggest_tuple(matrix: &Grid<bool>, names: &Vec<String>) -> Vec<usize> {
    let m = matrix.width;

    // Count how much connectivy each node has
    // XXX Funny inputs... they all have the exact same connectivity count.
    // 4 for the sample, 13 for the problem input.
    /*
        let mut connectivy_count = Vec::<usize>::with_capacity(matrix.width);



        for a in 0..m {
            let mut connect = 0;
            for b in 0..m {
                if matrix.get(a,b) {
                    connect += 1;
                }
            }
            connectivy_count.push(connect);
        }

        let mut max_connectivity = connectivy_count.clone();
        max_connectivity.sort();

        eprintln!("Connects = {:?}", max_connectivity);
    */
    // Add the diagonal (self-connectivity) for easier processing
    let mut matrix = matrix.clone();
    for a in 0..m {
        matrix.set(a, a, true);
    }

    matrix.pretty_print_bool();

    // We suppose that the biggest connected group will be connected
    // only to itself, except for one outside connection for each member
    // (all other groups will have more outside connections)

    // This group will have the property that all their rows
    // (or columns) will be identical in the matrix, except for 1 element.

    // (Initially the assumption was that the group did not have any
    // external connection at all but this failed)

    'search: for a in 0..m {
        let row_a = matrix.get_row_slice(a);
        let mut outliers = 0;
        // Construct the connected group by omiting outliers.
        let mut group = Vec::<usize>::new();
        group.push(a);
        for b in 0..m {
            if a != b && row_a[b] {
                // a and b are connected
                let row_b = matrix.get_row_slice(b);
                let diff = get_row_distance(row_a, row_b);
                let na = &names[a];
                let nb = &names[b];
                eprintln!("diff {a}/{b} ({na}/{nb}) = {diff}");
                // We accept at most 1 difference in the group (double it
                // because if some ma in a is missing in b,
                // then another mb in b is missing in a too.)
                if diff > 2 {
                    // but a and b don't have the same exact connection set
                    outliers += 1;
                    if outliers >= 2 {
                        continue 'search;
                    }
                } else {
                    group.push(b);
                    eprintln!("{a} and {b} are similar");
                }
            }
        }
        // stable group found

        return group;
    }

    panic!("Error: did not find any stable group");
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

    eprintln!("Names index = {:?}", computers_names);

    let max_tuple = find_biggest_tuple(&matrix, &computers_names);
    eprintln!("Biggest tuple is {:?}", max_tuple);
    let mut names: Vec<String> = max_tuple
        .iter()
        .map(|i| computers_names[*i].clone())
        .collect();
    names.sort();
    eprintln!("Names = {:?}", names);

    let password: String = names.join(",");
    println!("Part 2 = {password}");
}
