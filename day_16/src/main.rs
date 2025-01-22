/*
https://adventofcode.com/2024/day/16
--- Day 16: Reindeer Maze ---
 */

use aoc::colors::*;
use aoc::dijkstra::*;
use aoc::grid::{Grid, GridBuilder};
use std::io;
use std::io::prelude::*;
use std::time::{Duration, Instant};

struct Maze {
    map: Grid<bool>,
    // tuple of (distance, (prev-coordinate))
    path: Grid<(usize, Option<(usize, usize)>)>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn as_delta(&self) -> (isize, isize) {
        match self {
            Up => (0, -1),
            Left => (-1, 0),
            Down => (0, 1),
            Right => (1, 0),
        }
    }
}
use Direction::*;

impl Maze {
    const CONTINUE_FRONT: usize = 1;
    const ROTATE_90: usize = 1000;
}

impl DijkstraController for Maze {
    // The X,Y coordinates in the grid + The current orientation.
    type Node = (usize, usize, Direction);

    fn get_starting_node(&self) -> Self::Node {
        // bottom-left +1 corner, facing east
        (1, self.map.height - 2, Right)
    }

    fn get_target_node(&self) -> Self::Node {
        // top-right -1 corner, arbitrary direction
        (self.map.width - 2, 1, Up)
    }

    // The possible neighbors are the next node in front of the current direction
    // (if no wall obstructs) and the two 90° rotation at same x,y.
    fn get_neighbors_distances(&self, node: &Self::Node) -> Vec<(Self::Node, usize)> {
        let mut neighbs = Vec::<(Self::Node, usize)>::with_capacity(3);

        // Special case when we are already at the "end node" but not in the right direction:
        // provide 0-cost transition
        let target = self.get_target_node();
        if node.0 == target.0 && node.1 == target.1 {
            neighbs.push((target, 0));
            return neighbs;
        }

        let signed_node: (isize, isize) = (node.0 as isize, node.1 as isize);
        let direction = node.2;
        let delta = direction.as_delta();
        // Continue same direction, no wall in front
        let n = (signed_node.0 + delta.0, signed_node.1 + delta.1);
        if let Some(is_wall) = self.map.checked_get(n.0, n.1) {
            if !is_wall {
                let nnode = (n.0 as usize, n.1 as usize, direction);
                neighbs.push((nnode, Self::CONTINUE_FRONT));
            }
        }
        // Two possible rotations on place
        match direction {
            Up | Down => {
                neighbs.push(((node.0, node.1, Left), Self::ROTATE_90));
                neighbs.push(((node.0, node.1, Right), Self::ROTATE_90));
            }
            Left | Right => {
                neighbs.push(((node.0, node.1, Up), Self::ROTATE_90));
                neighbs.push(((node.0, node.1, Down), Self::ROTATE_90));
            }
        }

        neighbs
    }

    fn mark_visited_distance(
        &mut self,
        node: Self::Node,
        distance: usize,
        previous: Option<Self::Node>,
    ) {
        // Only mark a node the first time it is visited;
        // So this will mark a (x,y) only when coming from
        // a different cell coordinates, and not when rotating
        // on itself (for fill_backward_path)
        let existing = self.path.get(node.0, node.1);
        if existing.0 != 0 {
            return;
        }
        if let Some(previous) = previous {
            let previous = (previous.0, previous.1);
            self.path.set(node.0, node.1, (distance, Some(previous)));
        } else {
            self.path.set(node.0, node.1, (distance, None));
        }
    }
}

fn fill_backward_path(
    start: <Maze as DijkstraController>::Node,
    end: <Maze as DijkstraController>::Node,
    path: &mut Grid<char>,
    full: &Grid<(usize, Option<(usize, usize)>)>,
) {
    let mut node = (end.0, end.1);
    while node != (start.0, start.1) {
        let (_, prev) = full.get(node.0, node.1);
        let Some(prev) = prev else {
            panic!("Following path from end doesn't reach start");
        };
        let c = if node.0 > prev.0 {
            '>'
        } else if node.0 < prev.0 {
            '<'
        } else if node.1 > prev.1 {
            'v'
        } else if node.1 < prev.1 {
            '^'
        } else {
            '?'
        };
        path.set(node.0, node.1, c);
        node = prev;
    }
    path.set(node.0, node.1, 'S');
}

fn main() {
    // ----
    let start_parse = Instant::now(); // Start measuring time.
    let mut gb: GridBuilder<bool> = Default::default();

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        gb.append_char_map(&line, '#');
    }

    let map = gb.to_grid();
    let (width, height) = (map.width, map.height);
    let mut graph = Maze {
        map,
        path: Grid::<(usize, Option<(usize, usize)>)>::new(width, height, (0, None)),
    };
    let elapsed_parse: Duration = Instant::now() - start_parse; // Calculate elapsed time.

    // ----
    let start_process = Instant::now(); // Start measuring time.

    let distance = dijkstra(&mut graph);

    println!("Part 1 = {}", distance);
    let elapsed_process: Duration = Instant::now() - start_process; // Calculate elapsed time.

    if aoc::args::is_debug() {
        let mut path = Grid::<char>::new(width, height, ' ');
        let start = graph.get_starting_node();
        let target = graph.get_target_node();

        fill_backward_path(start, target, &mut path, &graph.path);
        graph
            .map
            .pretty_print_lambda_with_overlay(&path, &|w, c, xy| {
                if w {
                    // wall
                    format!("{}#", FG_COLORS[BLUE])
                } else {
                    let color = if xy == (start.0, start.1) {
                        FG_BRIGHT_COLORS[GREEN]
                    } else if xy == (target.0, target.1) {
                        FG_BRIGHT_COLORS[RED]
                    } else {
                        FG_BRIGHT_COLORS[WHITE]
                    };
                    // path, or blank
                    format!("{}{c}", color)
                }
            });
    }

    eprintln!("Time taken for parsing: {:?}", elapsed_parse);
    eprintln!("Time taken for processing: {:?}", elapsed_process);
    eprintln!("Total time: {:?}", elapsed_process + elapsed_parse);
}
