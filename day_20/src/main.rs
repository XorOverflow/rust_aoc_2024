/*
https://adventofcode.com/2024/day/20
--- Day 20: Race Condition ---
 */

use aoc::grid::{Grid, GridBuilder};
use std::io;
use std::io::prelude::*;

// Preprocessing:
// Follow the single-path track and updates the picosecond time taken at each
// grid point.
// Start point starts at time '0', End points receives total time + 1,
// and unpassable walls stay at 0.
fn map_to_track_time(m: &Grid<char>, start: (usize, usize), end: (usize, usize)) -> Grid<usize> {
    let mut track = Grid::<usize>::new(m.width, m.height, 0);
    let mut pos = start;
    let mut time = 1;
    track.set(pos.0, pos.1, time);

    let dir: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    // Don't backtrack (avoid checking the value in track[],
    let mut coming_from: (isize, isize) = (0, 0);
    while pos != end {
        time += 1;
        for d in &dir {
            if *d == coming_from {
                continue;
            }
            let next = (
                pos.0.checked_add_signed(d.0).unwrap(),
                pos.1.checked_add_signed(d.1).unwrap(),
            );
            if m.get(next.0, next.1) == '#' {
                // wall
                continue;
            }
            track.set(next.0, next.1, time);
            pos = next;
            coming_from = (-d.0, -d.1);
            break;
            // We expect the input to be well formed and always
            // reach the exit, else infinite loop
        }
    }

    track
}

// To find valid cheats, and to know how much time they save, simply take
// the annotated time-track.
// For each wall position "1" in the map adjacent to a valid track ,
// test the 4 possible "->2" end of cheat position. If this 2 is on a
// track, compute the difference time between the adjacent start and the "2"
// track time, and add 1 for the new cheated position.
// This is the time saved by this shortcut.
// The text problem is a little ambiguous, but I suppose only "1" is a wall-hack,
// and "2" is back to normal track (not a different wall)
fn find_cheat_cuts(track: &Grid<usize>, min_time: usize) -> usize {
    // Ignore side borders, we suppose that 1-2 cannot
    // make cuts through this part.
    let mut count = 0;

    for x in 1..track.width - 1 {
        for y in 1..track.height - 1 {
            if track.get(x, y) != 0 {
                continue;
            }
            for p in get_adjacent_tracks(track, x, y) {
                // Too many off-by-one or off-by-two errors in the formula
                // I don't understand why it's -2 instead of +1,
                // but that's what needed to match the sample.
                let saved_time = p.0.abs_diff(p.1) - 2;
                if saved_time >= min_time {
                    //eprintln!("Shortcut at {x},{y} saves {saved_time}");
                    count += 1;
                }
            }
        }
    }
    count
}

// Returns 0, 1 or 2 pairs of points on the track adjacent to xy.
// returns the their track times, not their coordinates.
fn get_adjacent_tracks(track: &Grid<usize>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut list = Vec::<(usize, usize)>::new();

    let h0 = track.get(x - 1, y);
    let h1 = track.get(x + 1, y);
    let v0 = track.get(x, y - 1);
    let v1 = track.get(x, y + 1);
    if h0 > 0 && h1 > 0 {
        list.push((h0, h1));
    }
    if v0 > 0 && v1 > 0 {
        list.push((v0, v1));
    }

    list
}

// part 2 is an extension of part 1 on the max distance
// (Manhattan) between cheat entry (on the track) and exit.
// Since only start and end defines a cheat, unicity is get by
// forcing the timestamp of start<end (compared to get_adjacent_tracks
// that was unordered around a center)

// From a track starting point x,y, count exit of cheat at distance at most
// 'dist' (2 in part1, 20 in part 2) that save at least "save" time.
// Does not count a "start" in double if the x,y is an exit:
// computed save time must be positive.
fn get_valid_ends_count(
    track: &Grid<usize>,
    x: usize,
    y: usize,
    dist_max: usize,
    save_min: usize,
) -> usize {
    let start_time = track.get(x, y);
    if start_time == 0 {
        // This starts from inside the walls.
        return 0;
    }

    let mut total_valid = 0;
    let x = x as isize;
    let y = y as isize;
    let half: isize = dist_max as isize;
    for kx in (x - half)..=(x + half) {
        for ky in (y - half)..=(y + half) {
            let cheat_dist = manhattan_dist(x, y, kx, ky);
            // We could make a more intelligent loop
            // that covers exactly the distance instead
            // of checking and eliminating the 4 corners,
            // but it would be more tedious
            if cheat_dist > dist_max {
                continue;
            }
            // This will often overflow the boundary near the border,
            // so expected to check.
            if let Some(end_time) = track.checked_get(kx, ky) {
                if end_time == 0 {
                    // ends into a wall
                    continue;
                }
                if end_time >= start_time + cheat_dist + save_min {
                    //let saved_time = end_time - cheat_dist - start_time;
                    //eprintln!("({x},{y}) -> ({kx},{ky}) : saves {saved_time} at distance {cheat_dist}");
                    total_valid += 1;
                }
            }
        }
    }

    total_valid
}

fn manhattan_dist(x1: isize, y1: isize, x2: isize, y2: isize) -> usize {
    x2.abs_diff(x1) + y2.abs_diff(y1)
}

fn find_super_cheat_cuts(track: &Grid<usize>, max_cheat: usize, min_time: usize) -> usize {
    let mut count = 0;

    for x in 1..track.width - 1 {
        for y in 1..track.height - 1 {
            count += get_valid_ends_count(track, x, y, max_cheat, min_time);
        }
    }
    count
}

fn main() {
    let mut mapbuild = GridBuilder::<char>::new();

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    let mut lines = io::stdin().lock().lines();
    let mut y = 0;
    while let Some(Ok(line)) = lines.next() {
        let mut vs: Vec<char> = line.chars().collect();
        if let Some(s) = vs.iter().position(|&c| c == 'S') {
            start = (s, y);
            vs[s] = '.';
        }
        if let Some(e) = vs.iter().position(|&c| c == 'E') {
            end = (e, y);
            vs[e] = '.';
        }
        mapbuild.append_line(&vs);
        y += 1;
    }

    let map = mapbuild.to_grid();
    map.pretty_print();
    eprintln!("Starts at {:?}, ends at {:?}", start, end);

    let track = map_to_track_time(&map, start, end);
    track.pretty_print_lambda(&|d: usize| {
        if d == 0 {
            ". ".to_string()
        } else {
            format!("{} ", d % 10)
        }
    });
    eprintln!("Total track time is {}", track.get(end.0, end.1) - 1);

    // different settings for sample and real input
    let pico_to_save = if track.width > 15 { 100 } else { 15 };
    println!("Part 1 = {}", find_cheat_cuts(&track, pico_to_save));

    // unit test: algo for part 2 should find the same result
    // for part 1 with adequate settings
    println!(
        "Part 1bis = {}",
        find_super_cheat_cuts(&track, 2, pico_to_save)
    );

    let pico_to_save = if track.width > 15 { 100 } else { 72 };
    println!(
        "Part 2 = {}",
        find_super_cheat_cuts(&track, 20, pico_to_save)
    );
}
