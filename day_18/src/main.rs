/*
https://adventofcode.com/2024/day/18
--- Day 18: RAM Run ---
 */

use aoc::dijkstra::*;
use aoc::grid::{Grid, GridBuilder};
use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct Maze {
    // Original, read-only map of the input data.
    // The value is "0" for empty cell, or a positive
    // "generation" which is the order of the
    // "corrupted byte" falling on the RAM zone.
    map: Grid<u16>,
    paths: Grid<Option<(isize, isize)>>,
    start: (isize, isize),
    exit: (isize, isize),
    generation: u16,
}

impl Maze {
    fn new_from_map(map: &Grid<u16>) -> Self {
        let (width, height) = (map.width, map.height);
        let start = (0, 0);
        let exit = (width as isize - 1, height as isize - 1);

        Maze {
            map: map.clone(),
            paths: Grid::<Option<(isize, isize)>>::new(width, height, None),
            start,
            exit,
            generation: 0,
        }
    }
    fn set_generation(&mut self, gen: u16) {
        self.generation = gen;
        self.paths.fill(None);
    }

    fn get_bool_map_from_generation(&self) -> Grid<bool> {
        let mut bool_builder = GridBuilder::<bool>::new();
        for y in 0..self.map.height {
            let bools: Vec<bool> = self
                .map
                .get_row_slice(y)
                .iter()
                .map(|gen| *gen <= self.generation)
                .collect();
            bool_builder.append_line(&bools);
        }
        bool_builder.to_grid()
    }
}

impl DijkstraController for Maze {
    // Just the X,Y coordinates in the grid
    type Node = (isize, isize);

    fn get_starting_node(&self) -> Self::Node {
        self.start
    }

    fn get_target_node(&self) -> Self::Node {
        self.exit
    }

    fn get_neighbors_distances(&self, node: &Self::Node) -> Vec<(Self::Node, usize)> {
        let (x, y) = node;
        let mut neighbs = Vec::<(Self::Node, usize)>::with_capacity(4);
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            match self.map.checked_get(x + dx, y + dy) {
                None => (),
                Some(gen) => {
                    if gen > self.generation {
                        // If a corrupted byte is created later
                        // than the current generation we consider,
                        // it's as if it is not here.
                        neighbs.push(((x + dx, y + dy), 1));
                    } else {
                        ()
                    }
                }
            }
        }

        neighbs
    }

    fn mark_visited_distance(
        &mut self,
        node: Self::Node,
        _distance: usize,
        previous: Option<Self::Node>,
    ) {
        self.paths.set(node.0 as usize, node.1 as usize, previous);
    }
}

fn main() {
    // ----
    let start_parse = Instant::now();
    // To simplify algo, "empty" cells (non corrupted)
    // are represented as "infinite" generation number
    // instead of 0.
    // This way, "cell is free <==> cell > tested_generation"
    // without any special case for 0.
    let mut map = Grid::<u16>::new(71, 71, u16::MAX);
    let mut coords = Vec::<(usize, usize)>::new();
    let mut generation: u16 = 0;
    let mut max_generation = 12;

    let mut lines = io::stdin().lock().lines();
    let mut max_coord = 0;

    // Parse the input and fill the map by recording
    // the order each cell is corrupted ("generation")
    while let Some(Ok(line)) = lines.next() {
        if let Some((x, y)) = line.split_once(',') {
            let x = usize::from_str(x).unwrap();
            let y = usize::from_str(y).unwrap();
            generation += 1;
            map.set(x, y, generation);
            coords.push((x, y));

            // Distinguish samples and actual prod input,
            // for different algo parameters
            max_coord = std::cmp::max(max_coord, x);
            max_coord = std::cmp::max(max_coord, y);
            if max_coord > 6 {
                max_generation = 1024;
            }
        } else {
            panic!("invalid input format {line}");
        }
    }

    if max_coord <= 6 {
        println!("Using 'sample' small coordinates");
        let mut map2 = Grid::<u16>::new(7, 7, 0);
        for x in 0..7 {
            for y in 0..7 {
                map2.set(x, y, map.get(x, y));
            }
        }
        map = map2;
    }

    let elapsed_parse: Duration = Instant::now() - start_parse; // Calculate elapsed time.

    // ---- Part 1
    let start_process = Instant::now(); // Start measuring time.

    let mut maze = Maze::new_from_map(&map);
    maze.set_generation(max_generation);

    let distance = dijkstra(&mut maze, false);
    if aoc::args::is_verbose() {
        let generation_map = maze.get_bool_map_from_generation();
        generation_map.pretty_print_bool_half();
        println!("shortest path at generation {max_generation}:");
        // Reconstruct (one of the possible) shortest path by walking back from the exit
        // on the finalized nodes set on the dijkstracontroller
        let mut shortpath = Grid::<bool>::new(maze.map.width, maze.map.height, false);
        let mut walknode = maze.exit;
        while let Some(Some(prevnode)) = maze.paths.checked_get(walknode.0, walknode.1) {
            walknode = prevnode;
            shortpath.set(walknode.0 as usize, walknode.1 as usize, true);
        }
        shortpath.pretty_print_bool_half();
    }

    println!("Part 1 = {}", distance);

    // ---- Part 2

    // Dichotomize the generation to find blocking/non blocking states.
    // "bisect_prev" is not blocking, and "bisect_next" is blocking.
    // expect from the problem input that the starting generation (12 or 1024)
    // is never blocking, and the final generation is always blocking.
    let mut bisect_prev = max_generation;
    let mut bisect_next = generation;

    if aoc::args::is_debug() {
        println!(
            "Starting bisecting to find blocking gen, between {bisect_prev} and {bisect_next}"
        );
    }

    while bisect_next > bisect_prev + 1 {
        let bisect_test;
        if bisect_next > bisect_prev + 2 {
            bisect_test = (bisect_prev + bisect_next) / 2;
        } else {
            bisect_test = bisect_prev + 1;
        }
        if aoc::args::is_debug() {
            println!("bisect: {bisect_test}");
        }
        maze.set_generation(bisect_test);
        let test_distance = dijkstra(&mut maze, false);

        if test_distance == usize::MAX {
            if aoc::args::is_debug() {
                println!("bisect: Maze was impossible to solve at generation {bisect_test}");
            }

            // Bisect: found "bad"
            bisect_next = bisect_test;
        } else {
            if aoc::args::is_debug() {
                println!("bisect: Maze was ok to solve at generation {bisect_test}");
            }
            // Bisect: found "good"
            bisect_prev = bisect_test;
        }
    }

    if bisect_next == bisect_prev + 1 {
        // Found the exact blocking generation
        let blocking_cell = coords.get((bisect_next - 1) as usize); // array is 0-indexed
        if aoc::args::is_debug() {
            println!(
                "Part 2: Maze was blocked on generation {bisect_next} at cell coordinate {:?}",
                blocking_cell
            );
        }
        let coordinates = blocking_cell.unwrap();
        println!("Part 2: {},{}", coordinates.0, coordinates.1);
    } else {
        panic!("Part2 : bisect didn't converge");
    }

    let elapsed_process: Duration = Instant::now() - start_process; // Calculate elapsed time.

    eprintln!("Time taken for parsing: {:?}", elapsed_parse);
    eprintln!("Time taken for processing: {:?}", elapsed_process);
    eprintln!("Total time: {:?}", elapsed_process + elapsed_parse);
}
