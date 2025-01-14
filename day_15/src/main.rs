/*
https://adventofcode.com/2024/day/15
--- Day 15: Warehouse Woes ---
 */
use aoc::args;
use aoc::colors::*;
use aoc::grid::{Grid, GridBuilder};
use std::io;
use std::io::prelude::*;

#[derive(Clone, Copy)]
enum Movement {
    Up,
    Left,
    Down,
    Right,
}

use Movement::*;

impl Movement {
    fn from_char(c: char) -> Movement {
        match c {
            '^' => Up,
            '<' => Left,
            'v' => Down,
            '>' => Right,
            _ => panic!("Illegal movement char"),
        }
    }

    fn as_delta(&self) -> (isize, isize) {
        match self {
            Up => (0, -1),
            Left => (-1, 0),
            Down => (0, 1),
            Right => (1, 0),
        }
    }
}

fn move_once(
    robot: (isize, isize),
    m: Movement,
    maze: &Grid<bool>,
    boxes: &mut Grid<bool>,
) -> (isize, isize) {
    let d = m.as_delta();
    let next_pos = (robot.0 + d.0, robot.1 + d.1);
    // All inputs have walls on the border so we will
    // never fall into the _or() case of unwrap checking out-of-bound.
    if boxes.checked_get(next_pos.0, next_pos.1).unwrap_or(true) {
        // Box in the way. Push it if possible.
        let mut stack = next_pos;
        while boxes.checked_get(stack.0, stack.1).unwrap_or(true) {
            stack = (stack.0 + d.0, stack.1 + d.1);
        }
        // End of stack of box
        if maze.checked_get(stack.0, stack.1).unwrap_or(true) {
            // Wall in the way. Can't move anything.
            robot
        } else {
            // Free space to push the box stack.
            // Update head and tail of the stack
            boxes.set(next_pos.0 as usize, next_pos.1 as usize, false);
            boxes.set(stack.0 as usize, stack.1 as usize, true);
            next_pos
        }
    } else if maze.checked_get(next_pos.0, next_pos.1).unwrap_or(true) {
        // Wall in the way. Can't move.
        robot
    } else {
        // Free space. Move.
        next_pos
    }
}

// Process each robot movement, pushing as needed.
// Return the final robot position, as well as a trace
// of all its visited locations in a new grid.
fn process_all_movements(
    robot: (usize, usize),
    moves: &[Movement],
    maze: &Grid<bool>,
    boxes: &mut Grid<bool>,
) -> ((usize, usize), Grid<bool>) {
    let mut trace = Grid::<bool>::new(maze.width, maze.height, false);
    trace.set(robot.0, robot.1, true);
    let mut robot: (isize, isize) = (robot.0 as isize, robot.1 as isize);

    for m in moves {
        robot = move_once(robot, *m, maze, boxes);
        trace.set(robot.0 as usize, robot.1 as usize, true);
    }

    ((robot.0 as usize, robot.1 as usize), trace)
}

fn sum_gps_coordinates(boxes: &Grid<bool>) -> usize {
    let mut s = 0;
    for y in 0..boxes.height {
        for x in 0..boxes.width {
            if boxes.get(x, y) {
                s += x + 100 * y;
            }
        }
    }

    s
}

fn print_maze(robot: (usize, usize), maze: &Grid<bool>, boxes: &Grid<bool>) {
    if args::is_verbose() {
        maze.pretty_print_bool();
        boxes.pretty_print_bool();
    }
    maze.pretty_print_lambda_with_overlay(&boxes, &|w, b, xy| {
        if w {
            // wall
            format!("{}#", FG_COLORS[BLUE])
        } else if b {
            // box
            format!("{}O", FG_BRIGHT_COLORS[WHITE])
        } else if xy == robot {
            format!("{}@", FG_COLORS[RED])
        } else {
            " ".to_string()
        }
    });
}

fn main() {
    // Here I try to use two different booleans maps:
    // one, static, for the maze walls (#)
    // and another one just for the boxes (O)
    // A different method would be to merge
    // them into one map with 3 different states (and it would
    // make impossible to have a box and a wall at the same place)
    let mut mazebuild = GridBuilder::<bool>::new();
    let mut boxbuild = GridBuilder::<bool>::new();

    let mut robot: (usize, usize) = (0, 0);

    let mut lines = io::stdin().lock().lines();
    let mut y = 0;

    // parsing the map
    while let Some(Ok(line)) = lines.next() {
        if line.len() == 0 {
            break;
        }
        if robot == (0, 0) {
            let vs: Vec<char> = line.chars().collect();
            if let Some(s) = vs.iter().position(|&c| c == '@') {
                robot = (s, y);
            }
        }
        mazebuild.append_char_map(&line, '#');
        boxbuild.append_char_map(&line, 'O');
        y += 1;
    }

    let maze = mazebuild.to_grid();
    let mut boxes = boxbuild.to_grid();
    assert_eq!(maze.width, boxes.width);
    assert_eq!(maze.height, boxes.height);

    // parsing the movements
    let mut moves = Vec::<Movement>::new();
    while let Some(Ok(line)) = lines.next() {
        let mut m: Vec<Movement> = line.chars().map(|c| Movement::from_char(c)).collect();
        moves.append(&mut m);
    }

    // Debug print
    if args::is_debug() {
        print_maze(robot, &maze, &boxes)
    }

    let (robot, trace) = process_all_movements(robot, &moves, &maze, &mut boxes);

    if args::is_debug() {
        trace.pretty_print_bool();
        print_maze(robot, &maze, &boxes);
    }

    let gps_total = sum_gps_coordinates(&boxes);

    println!("Part 1 = {gps_total}");
}
