/*
https://adventofcode.com/2024/day/10
--- Day 10: Hoof It ---
 */

use std::io;
//use std::str::FromStr;
use std::io::prelude::*;

// Recursive sum of rest of trail or trail forks
fn trail_score(map: &Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    // Naive recursion will count the number of PATHs that
    // a trail will lead to an ending 9, but not the
    // number of singular end cell if they are reached
    // by multiple ways !
    // So keep track of unique path locations and dont take them
    // twice

    let mut locations = Vec::<(usize, usize)>::new();
    trail_score_internal(map, x, y, &mut locations)
}

// Recursive sum of rest of trail or trail forks
fn trail_score_internal(
    map: &Vec<Vec<usize>>,
    x: usize,
    y: usize,
    locations: &mut Vec<(usize, usize)>,
) -> usize {
    let mut score: usize = 0;
    let elevation = map[y][x];

    // Found end
    if elevation == 9 {
        //eprintln!("Found trail end at {x},{y}");
        return 1;
    }

    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let next_x = (x as isize + dx) as usize;
        let next_y = (y as isize + dy) as usize;

        if map[next_y][next_x] == elevation + 1 {
            // found potential path to follow
            if locations.contains(&(next_x, next_y)) {
                // already taken via a different fork
                continue;
            }
            locations.push((next_x, next_y));

            score += trail_score_internal(map, next_x, next_y, locations);
        }
    }
    score
}

fn trailhead_total_scores(map: &Vec<Vec<usize>>) -> usize {
    // Iterate on the useful interior, ignore borders
    let width = map[0].len() - 1;
    let height = map.len() - 1;

    let mut scores: usize = 0;

    for y in 1..height {
        for x in 1..width {
            // trail Head (starting point)
            if map[y][x] == 0 {
                let single_score = trail_score(map, x, y);
                //eprintln!("Found trail start at {x},{y}, of score {single_score}");
                scores += single_score;
            }
        }
    }

    scores
}

fn count_2(input: &Vec<Vec<usize>>) -> usize {
    input[0].len()
}

fn main() {
    // Interior [1..len-1]  is the parsed height value from the map,
    // and a 1 cell border is added all around with an
    // impossible 999 value to avoid doing constant
    // bound checking on coordinates.
    let mut parsed = Vec::<Vec<usize>>::new();

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        //eprintln!("Parsed one string '{line}'");
        if parsed.is_empty() {
            parsed.push(std::iter::repeat_n::<usize>(999, line.len() + 2).collect());
        }

        // Add barrier values at start and end with iterator chaining.
        // This is just a funny way to avoid concatenating a '*' to the source
        // string and avoid reallocation, purely for the fun of it.
        let values: Vec<usize> = "*"
            .chars()
            .chain(line.chars().chain("*".chars()))
            .map(|c| c.to_digit(10).unwrap_or(999) as usize)
            .collect();
        parsed.push(values);
    }
    // Add barrier to bottom of map
    parsed.push(parsed[0].clone());

    println!("Part 1 = {}", trailhead_total_scores(&parsed));

    println!("Part 2 = {}", count_2(&parsed));
}
