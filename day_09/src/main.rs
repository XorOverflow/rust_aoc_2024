/*
https://adventofcode.com/2024/day/9
--- Day 9: Disk Fragmenter ---
 */

use std::io;
use std::io::prelude::*;

#[derive(Clone, Copy, Debug)]
enum DiskMap {
    // A file, with its ID
    File(usize),
    // Empty space
    Empty,
}

use DiskMap::*;

// Convert the run-length encoding of file/empty space, into an explicit
// list of multiple blocks and indexed files.
fn rle_to_blocks(input: &Vec<usize>) -> Vec<DiskMap> {
    let mut blocks = Vec::<DiskMap>::new();

    let mut is_file = true;
    let mut id: usize = 0;
    for length in input {
        let block = if is_file { File(id) } else { Empty };
        if is_file {
            id += 1;
        }
        is_file = !is_file;
        blocks.extend(std::iter::repeat(block).take(*length));
    }

    blocks
}

fn defrag(input: &Vec<usize>) -> Vec<DiskMap> {
    let mut blocks = rle_to_blocks(input);
    // let reverse = blocks.clone().reverse(); // actually not useful

    let mut scan_free: usize = 0;
    let mut scan_move: usize = blocks.len() - 1;

    // Why does it double-cross itself when using normal check scan_move > scan_free ??
    while scan_move >= scan_free + 2 {
        match blocks[scan_move] {
            Empty => {
                scan_move -= 1;
                continue;
            }
            File(id) => {
                while let File(_) = blocks[scan_free] {
                    scan_free += 1;
                }
                blocks[scan_free] = File(id);
                blocks[scan_move] = Empty;
                //scan_move -= 1;
            }
        }
    }
    //eprintln!("defrag end; Next block to test {scan_move}, current possible free {scan_free}");

    blocks
}

fn defrag_checksum(input: &Vec<usize>) -> usize {
    let defragged = defrag(input);

    //eprintln!("Defrag =  {:?} ", defragged);
    let mut checksum: usize = 0;

    for k in 0..defragged.len() {
        match defragged[k] {
            File(id) => {
                let part = k * id;
                checksum += part;
                //eprintln!("Defrag [{k}] = {:?} => chk + {part} = {checksum}", defragged[k]);
            }
            // after defrag, no other file block is
            // expected after a first empty space is encountered.
            Empty => break,
        }
    }
    checksum
}

fn count_2(input: &Vec<usize>) -> usize {
    input.len()
}

fn main() {
    let stdin = io::stdin();
    // There is only one big line in the input.
    let line: String = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .expect("input should contain one line");

    let parsed: Vec<usize> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    println!("Part 1 = {}", defrag_checksum(&parsed));

    println!("Part 2 = {}", count_2(&parsed));
}
