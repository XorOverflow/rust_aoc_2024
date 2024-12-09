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

// compared to DiskMap, each block also maintain
// the number of other blocks on its left and on its right
// of the same contigous file/empty. (so 0,0 for a span
// of size 1)

#[derive(Clone, Copy, Debug)]
enum DiskMapLen {
    // A file, with its ID, and its left/right remaining size
    File(usize, usize, usize),
    // Empty space, and its remaining size
    Empty(usize, usize),
}

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

// Convert the run-length encoding of file/empty space, into an explicit
// list of multiple blocks and indexed files.
fn rle_to_blocks_length(input: &Vec<usize>) -> Vec<DiskMapLen> {
    let mut blocks = Vec::<DiskMapLen>::new();

    let mut is_file = true;
    let mut id: usize = 0;
    for length in input {
        if is_file {
            for len in 0..*length {
                let block = DiskMapLen::File(id, len, *length - len - 1);
                blocks.push(block);
            }
            id += 1;
        } else {
            for len in 0..*length {
                let block = DiskMapLen::Empty(len, *length - len - 1);
                blocks.push(block);
            }
        }
        is_file = !is_file;
    }

    blocks
}

// This is horrible.
fn defrag_contiguous(input: &Vec<usize>) -> Vec<DiskMap> {
    let mut blocks = rle_to_blocks_length(input);

    let mut scan_move: usize = blocks.len() - 1;

    loop {
        match blocks[scan_move] {
            DiskMapLen::Empty(llen, _) => {
                scan_move -= llen + 1;
                continue;
            }
            DiskMapLen::File(id, llen, _) => {
                // File id 0 is at the start of disk, no need to move, end condition.
                if id == 0 {
                    eprintln!("File 0, exit");
                    break;
                }

                // we found the last block of a file, of total span size
                // llen + 1
                let minspan = llen + 1;
                let file_start = scan_move - llen;

                //eprintln!("Checking file id {id} @{file_start}+{minspan}");

                // Need to search free space from the start each time
                let mut scan_free: usize = 0;

                loop {
                    while let DiskMapLen::File(_, _, rlen) = blocks[scan_free] {
                        scan_free += rlen + 1;
                    }

                    if scan_free >= scan_move {
                        // already went too far. This file will not move.
                        break;
                    }
                    if let DiskMapLen::Empty(_, rlen) = blocks[scan_free] {
                        if rlen + 1 >= minspan {
                            // Found space; move file
                            for k in 0..minspan {
                                blocks[scan_free + k] = blocks[file_start + k];
                                blocks[file_start + k] = DiskMapLen::Empty(0, 0);
                            }
                            // note: here File(id,llen,rlen) is valid.
                            // But the new Empty() to delete the old file space does
                            // not contain valid span information, and the remaining free space
                            // which is reduced contains corrupt "llen" information.
                            // For the exercise it has no effect because we will not
                            // parse or use those specific blocks information anymore but
                            // it's buggy in principle.

                            break;
                        } else {
                            scan_free += rlen + 1;
                        }
                    } else {
                        panic!("No Empty block after all File blocks");
                    }
                }
                // Check for next (previous) file to move.
                scan_move -= minspan;
            }
        }
    }
    //eprintln!("defrag end; Next block to test {scan_move}, current possible free {scan_free}");

    blocks
        .into_iter()
        .map(|b| match b {
            DiskMapLen::File(id, _, _) => File(id),
            DiskMapLen::Empty(_, _) => Empty,
        })
        .collect()
}

fn checksum(defragged: &Vec<DiskMap>) -> usize {
    //eprintln!("Defrag =  {:?} ", defragged);
    let mut checksum: usize = 0;

    for k in 0..defragged.len() {
        match defragged[k] {
            File(id) => {
                let part = k * id;
                checksum += part;
                //eprintln!("Defrag [{k}] = {:?} => chk + {part} = {checksum}", defragged[k]);
            }
            // continue to next block
            Empty => (),
        }
    }
    checksum
}

fn defrag_checksum(input: &Vec<usize>) -> usize {
    let defragged = defrag(input);
    checksum(&defragged)
}

fn defrag_contiguous_checksum(input: &Vec<usize>) -> usize {
    let defragged = defrag_contiguous(input);
    checksum(&defragged)
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

    println!("Part 2 = {}", defrag_contiguous_checksum(&parsed));
}
