/*
https://adventofcode.com/2024/day/8
--- Day 8: Resonant Collinearity ---
 */

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::ops::Add;
use std::ops::Sub;

// Use signed because also used as a signed vector
// and intermediate results could be negative (out of map)
#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Coord(isize, isize);

impl Sub for Coord {
    type Output = Coord;
    fn sub(self, other: Coord) -> Self::Output {
        Coord(self.0 - other.0, self.1 - other.1)
    }
}
impl Add for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Self::Output {
        Coord(self.0 + other.0, self.1 + other.1)
    }
}

fn in_map_bound(c: Coord, bound: Coord) -> bool {
    (0..bound.0).contains(&c.0) && (0..bound.1).contains(&c.1)
}

// Set all antinodes locations for 1 frequency
fn set_freq_antinodes_locations(
    antennas: &Vec<Coord>,
    locations: &mut HashSet<Coord>,
    bound: Coord,
) {
    // iterate over all (unordered) pairs of antenna
    for a in 0..antennas.len() - 1 {
        for b in a + 1..antennas.len() {
            let ca = antennas[a];
            let cb = antennas[b];
            let delta = cb - ca;
            let antinode_1 = ca - delta;
            let antinode_2 = cb + delta;
            if in_map_bound(antinode_1, bound) {
                locations.insert(antinode_1);
            }
            if in_map_bound(antinode_2, bound) {
                locations.insert(antinode_2);
            }
        }
    }
}

fn count_antinode_locations(input: &HashMap<char, Vec<Coord>>, bound: Coord) -> usize {
    // Set of all locations, without duplicates
    let mut locations = HashSet::<Coord>::new();

    for freq in input.values() {
        set_freq_antinodes_locations(freq, &mut locations, bound);
    }

    //eprintln!("antinodes locations = {:?}", locations);

    locations.len()
}

fn count_2() -> usize {
    0
}

fn main() {
    // each Frequency (1-letter input) maps to a list
    // of coordinates of each antenna.
    let mut antenna_map = HashMap::<char, Vec<Coord>>::new();

    let mut input = String::new();
    let mut y: isize = 0;

    let mut width: isize = 0;
    loop {
        match io::stdin().read_line(&mut input) {
            Err(_) => {
                panic!("input error, exit");
            }
            Ok(0) => {
                break;
            }
            Ok(_) => {
                let input_clean = input.trim();
                width = input_clean.len() as isize;
                for ic in input_clean.char_indices() {
                    let freq = ic.1;
                    if freq == '.' {
                        continue;
                    }
                    let coord = Coord(ic.0 as isize, y);
                    let freq_map = antenna_map.get_mut(&freq);
                    if let Some(list) = freq_map {
                        list.push(coord);
                    } else {
                        let mut list = Vec::<Coord>::new();
                        list.push(coord);
                        antenna_map.insert(freq, list);
                    }
                }
                y += 1;
            }
        }
        input = String::from("");
    }
    let bound = Coord(width, y);

    //eprintln!("Antenna map = {:?}", antenna_map);

    println!("Part 1 = {}", count_antinode_locations(&antenna_map, bound));

    println!("Part 2 = {}", count_2());
}
