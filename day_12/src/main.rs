/*
https://adventofcode.com/2024/day/12
--- Day 12: Garden Groups ---
 */

use aoc::grid::{Grid, GridBuilder};
use std::io;
use std::io::prelude::*;

/// Flood-fill a contiguous region of the same letter starting at coordinates.
/// Marks the corresponding "Region[x,y]" with the passed value v.
/// Recursive.
fn floodfill(map: &Grid<char>, region: &mut Grid<u32>, x: usize, y: usize, v: u32) {
    if region.get(x, y) != 0 {
        panic!("Recursing into already visited region");
    }

    let c = map.get(x, y);
    region.set(x, y, v);

    // Fill horizontal line
    let mut x1 = x;
    let mut x2 = x;

    while x1 >= 1 {
        x1 -= 1;
        if map.get(x1, y) == c {
            region.set(x1, y, v);
        } else {
            break;
        }
    }

    while x2 < map.width - 1 {
        x2 += 1;
        if map.get(x2, y) == c {
            region.set(x2, y, v);
        } else {
            break;
        }
    }

    // Recurse into top and bottom lines (if not already done)
    if y >= 1 {
        let top = y - 1;
        for x in x1..=x2 {
            if region.get(x, top) == 0 && map.get(x, top) == c {
                floodfill(map, region, x, top, v);
            }
        }
    }

    if y < map.height - 1 {
        let bot = y + 1;
        for x in x1..=x2 {
            if region.get(x, bot) == 0 && map.get(x, bot) == c {
                floodfill(map, region, x, bot, v);
            }
        }
    }
}

// Convert a map of plants letter, into a map of
// unique contiguous regions with different numerical ids
// (two disconnected plots of land with same plant letter
// will create two different ids).
// Return also the max ID found.
fn map_to_unique_regions(map: &Grid<char>) -> (Grid<u32>, u32) {
    let mut max: u32 = 0;
    let mut regions = Grid::<u32>::new(map.width, map.height, max);

    for x in 0..map.width {
        for y in 0..map.height {
            if regions.get(x, y) == 0 {
                max += 1;
                floodfill(&map, &mut regions, x, y, max);
                /*
                For debugging construction of the floodfill (it's ok)
                eprintln!("=========================");
                regions.pretty_print_lambda(&|v| if v == 0 { "  ".to_string() } else { format!("{}.", v%10) } );
                 */
            }
        }
    }

    (regions, max)
}

#[derive(Clone)]
struct Region {
    area: usize,
    perimeter: usize,
}

fn fence_cost(map: &Grid<u32>, max: u32) -> usize {
    let mut regions = Vec::<Region>::new();
    regions.resize(
        1 + max as usize,
        Region {
            area: 0,
            perimeter: 0,
        },
    );

    // cardinal directions
    let cards = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    for y in 0..map.height {
        for x in 0..map.width {
            let v = map.get(x, y);

            let r = &mut regions[v as usize];
            r.area += 1;
            for dir in &cards {
                let x = x as isize;
                let y = y as isize;

                if let Some(v2) = map.checked_get(x + dir.0, y + dir.1) {
                    // different plot
                    if v != v2 {
                        r.perimeter += 1;
                    }
                } else {
                    // side of map
                    r.perimeter += 1;
                }
            }
        }
    }

    let mut cost = 0;
    for k in 1..=max {
        let r = &regions[k as usize];
        eprintln!("Region {k} area {}, perimeter {}", r.area, r.perimeter);
        cost += r.area * r.perimeter;
    }

    cost
}

fn main() {
    let mut gb = GridBuilder::<char>::new();

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        let vs: Vec<char> = line.chars().collect();
        gb.append_line(&vs);
    }

    let map = gb.to_grid();
    let (regions, max) = map_to_unique_regions(&map);

    //regions.pretty_print_lambda(&|v| format!("{:03}.", if v > 610 { v } else { 0 } ));
    regions.pretty_print_lambda(&|v| format!("{:03}.", v));

    eprintln!("Map has {max} contiguous regions");
    println!("Part 1 = {}", fence_cost(&regions, max));

    // "Your answer is too high"
    // ¯\_(ツ)_/¯
}
