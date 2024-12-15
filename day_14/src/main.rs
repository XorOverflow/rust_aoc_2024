/*
https://adventofcode.com/2024/day/14
--- Day 14: Restroom Redoubt ---
 */

//use num::integer::div_rem;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

// Sample:
//const GRID_WIDTH:isize = 11;
//const GRID_HEIGHT:isize = 7;
//
//const GRID_WIDTH_MIDDLE:isize = 5;
//const GRID_HEIGHT_MIDDLE:isize = 3;

// actual input:
const GRID_WIDTH: isize = 101;
const GRID_HEIGHT: isize = 103;

const GRID_WIDTH_MIDDLE: isize = 50;
const GRID_HEIGHT_MIDDLE: isize = 51;

#[derive(Clone, Debug)]
struct Robot {
    p: (isize, isize),
    v: (isize, isize),
}

// Return the POSITIVE modulo/remainder of n/d.
// for proper wrap-around.
fn positive_mod(n: isize, d: isize) -> isize {
    let r = n % d;
    if r < 0 {
        r + d
    } else {
        r
    }
}
// "count the robots in each quadrant after 100 seconds"
// We now all know what this means for part 2... don't iterate
// and directly do a multiply and modulo for warping.
fn count_quadrants(input: &Vec<Robot>, elapse: isize) -> usize {
    let evolved: Vec<Robot> = input
        .iter()
        .map(|r| Robot {
            p: (
                positive_mod(r.p.0 + r.v.0 * elapse, GRID_WIDTH),
                positive_mod(r.p.1 + r.v.1 * elapse, GRID_HEIGHT),
            ),
            v: r.v,
        })
        .collect();
    /*
    A | B
    --+--
    C | D
     */
    let mut quadrant_a = 0;
    let mut quadrant_b = 0;
    let mut quadrant_c = 0;
    let mut quadrant_d = 0;
    for r in &evolved {
        //eprintln!("Evolved = {:?}", r.p);
        if r.p.0 < GRID_WIDTH_MIDDLE && r.p.1 < GRID_HEIGHT_MIDDLE {
            quadrant_a += 1;
        } else if r.p.0 > GRID_WIDTH_MIDDLE && r.p.1 < GRID_HEIGHT_MIDDLE {
            quadrant_b += 1;
        } else if r.p.0 < GRID_WIDTH_MIDDLE && r.p.1 > GRID_HEIGHT_MIDDLE {
            quadrant_c += 1;
        } else if r.p.0 > GRID_WIDTH_MIDDLE && r.p.1 > GRID_HEIGHT_MIDDLE {
            quadrant_d += 1;
        }
    }
    eprintln!("quadrants:     {quadrant_a} * {quadrant_b} * {quadrant_c} * {quadrant_d} ");
    quadrant_a * quadrant_b * quadrant_c * quadrant_d
}

fn main() {
    let mut robots = Vec::<Robot>::new();

    // split on space, then on '=', then on ','.
    // (easier/faster or not than a 4 group regex ?)
    //p=27,64 v=24,1

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        let pv = line.split_once(" ").unwrap();
        let p = pv.0.split_once('=').unwrap().1;
        let v = pv.1.split_once('=').unwrap().1;
        let p = p.split_once(',').unwrap();
        let v = v.split_once(',').unwrap();
        let robot = Robot {
            p: (isize::from_str(p.0).unwrap(), isize::from_str(p.1).unwrap()),
            v: (isize::from_str(v.0).unwrap(), isize::from_str(v.1).unwrap()),
        };

        robots.push(robot);
    }

    println!("Part 1 = {}", count_quadrants(&robots, 100));
}
