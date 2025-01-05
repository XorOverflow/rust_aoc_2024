/*
https://adventofcode.com/2024/day/12
--- Day 12: Garden Groups ---
 */

use aoc::args;
use aoc::colors;
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
        if map.get(x1 - 1, y) == c {
            x1 -= 1;
            region.set(x1, y, v);
        } else {
            break;
        }
    }

    while x2 < map.width - 1 {
        if map.get(x2 + 1, y) == c {
            x2 += 1;
            region.set(x2, y, v);
        } else {
            break;
        }
    }

    // x1 and x2 are now exactly the first and last x of this line
    // with the same plot character (no overrun)

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
            }
        }
    }

    (regions, max)
}

fn region_to_color(r: u32) -> &'static str {
    let reg = (r % 15) as usize;
    if reg < 7 {
        colors::FG_COLORS[reg + 1]
    } else {
        colors::FG_BRIGHT_COLORS[reg - 7]
    }
}

// Print the map in color
fn debug_print_regions(map: &Grid<char>, regions: &Grid<u32>) {
    // Each original "char" of the map is colored with the specifig region number
    // of this plot (can be different than a similar char from a different plot).
    // Just avoid dark black for my black background terminal; still use pure white.
    // FIXME: using a 4-color theorem or other to pick distinct colors on contiguous
    // regions is a different problem !
    let formatter = &|c, r| {
        let color = region_to_color(r);
        format!("{color}{c}")
    };
    map.pretty_print_lambda_with_overlay(regions, formatter);
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
    let mut check_area = 0;
    let verbose: bool = args::is_verbose();
    for k in 1..=max {
        let r = &regions[k as usize];
        if verbose {
            eprintln!(
                "Region {}{k}{} area {}, perimeter {}",
                region_to_color(k),
                colors::ANSI_RESET,
                r.area,
                r.perimeter
            );
        }
        cost += r.area * r.perimeter;
        check_area += r.area;

        if r.area == 1 {
            assert_eq!(r.perimeter, 4);
        }
        if r.area == 2 {
            assert_eq!(r.perimeter, 6);
        }
    }

    assert_eq!(check_area, map.width * map.height);

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
    if args::is_debug() {
        debug_print_regions(&map, &regions);
    }

    //regions.pretty_print_lambda(&|v| format!("{:03}.", if v > 610 { v } else { 0 } ));
    //regions.pretty_print_lambda(&|v| format!("{:03}.", v));

    eprintln!("Map has {max} contiguous regions");
    println!("Part 1 = {}", fence_cost(&regions, max));
}
