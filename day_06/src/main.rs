/*
https://adventofcode.com/2024/day/6
--- Day 6: Guard Gallivant ---
 */

use std::boxed::Box;
use std::io;

// Grid struct copied as-is from my last-year aoc 2023 day 17

// A custom 2D array more friendly than a Vec<Vec<T>>
struct Grid<T> {
    width: usize,
    height: usize,
    s: Box<[T]>,
}

impl<T: std::clone::Clone> Grid<T> {
    // Allocate the low-level array for this grid
    fn new(width: usize, height: usize, t0: T) -> Self {
        Self {
            width: width,
            height: height,
            s: vec![t0; width * height].into_boxed_slice(),
        }
    }

    // consume and convert a double-vector
    fn from_vec(mut v: Vec<Vec<T>>) -> Self {
        let t0 = v[0][0].clone();
        let mut s = Self::new(v[0].len(), v.len(), t0);
        // Could probably be done with something like:
        // v.drain(..).drain(..)

        // Pop from the end of the vector(s) to avoid
        // realloc (drain data)
        for y in (0..s.height).rev() {
            let mut row = v.pop().unwrap();
            for x in (0..s.width).rev() {
                s.set(x, y, row.pop().unwrap());
            }
        }
        s
    }

    fn get(&self, x: usize, y: usize) -> &T {
        if x >= self.width || y >= self.height {
            panic!("array access {},{} out of bounds", x, y)
        } else {
            &self.s[x + y * self.width]
        }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        if x >= self.width || y >= self.height {
            panic!("array access {},{} out of bounds", x, y)
        } else {
            &mut self.s[x + y * self.width]
        }
    }

    // todo: provide a macro
    fn set(&mut self, x: usize, y: usize, t: T) {
        if x >= self.width || y >= self.height {
            panic!("array access {},{} out of bounds", x, y);
        } else {
            self.s[x + y * self.width] = t;
        }
    }
}

impl<T: std::clone::Clone + std::fmt::Display> Grid<T> {
    fn pretty_print(&self) {
        eprintln!("[{},{}] = ", self.width, self.height);
        for y in 0..self.height {
            eprint!("[");
            for x in 0..self.width {
                eprint!("{} ", &self.get(x, y));
            }
            eprintln!("]");
        }
    }
}

impl Grid<bool> {
    fn pretty_print_bool(&self) {
        eprintln!("[{},{}] = ", self.width, self.height);
        for y in 0..self.height {
            eprint!("[");
            for x in 0..self.width {
                eprint!("{}", if *self.get(x, y) { '*' } else { '.' });
            }
            eprintln!("]");
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use Direction::*;

impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Left => Up,
            Up => Right,
            Right => Down,
            Down => Left,
        }
    }
}

impl<T> Grid<T> {
    // Return Some(newx,newy) after moving by direction, else None if out-of-bounds
    fn get_next_coordinates(&self, p: (usize, usize), d: Direction) -> Option<(usize, usize)> {
        let x = p.0;
        let y = p.1;
        #[rustfmt_skip]
        match d {
            Left =>  if x == 0             { None } else { Some((x-1, y)) },
            Right => if x+1 >= self.width  { None } else { Some((x+1, y)) },
            Up =>    if y == 0             { None } else { Some((x, y-1)) },
            Down =>  if y+1 >= self.height { None } else { Some((x, y+1)) },
        }
    }
}

fn count_positions(map: &Grid<bool>, start: (usize, usize)) -> usize {
    let mut pos = start;
    let mut direction = Up;

    let mut count_visit = 1; // include starting point

    // Keep the visited positions marked, to not
    // count them double when re-visiting them.
    let mut travel_map = Grid::<bool>::new(map.width, map.height, false);
    travel_map.set(start.0, start.1, true);

    loop {
        if let Some(new_coord) = map.get_next_coordinates(pos, direction) {
            if *map.get(new_coord.0, new_coord.1) {
                // would hit an obstacle
                direction = direction.rotate_right();
            } else {
                pos = new_coord;
                // First time visiting this space ?
                if !*travel_map.get(pos.0, pos.1) {
                    travel_map.set(pos.0, pos.1, true);
                    count_visit += 1;
                }
            }
        } else {
            // went out of the map
            break;
        }
    }

    // if debug
    eprintln!("Travel path:");
    travel_map.pretty_print_bool();

    count_visit
}

fn main() {
    let mut map = Vec::<Vec<bool>>::new();
    let mut start: Option<(usize, usize)> = None;

    let mut input = String::new();
    let mut y = 0;
    loop {
        match io::stdin().read_line(&mut input) {
            Err(_) => {
                panic!("input error, exit");
            }
            Ok(0) => {
                break;
            }
            Ok(_) => {
                let input_clean = input.trim(); // remove the \n
                if start.is_none() {
                    if let Some(start_x) = input_clean.find('^') {
                        start = Some((start_x, y));
                    }
                }
                let line: Vec<bool> = input_clean
                    .chars()
                    .map(|c| match c {
                        '#' => true,
                        _ => false, // including the starting '^'
                    })
                    .collect();
                map.push(line);
                y += 1;
            }
        }
        // must clear for next loop
        input = String::from("");
    }

    let map = Grid::<bool>::from_vec(map);
    let debug = true;
    if debug {
        map.pretty_print_bool();
        eprintln!("Starting position is at {:?}", start);
    }

    println!("Part 1 = {}", count_positions(&map, start.unwrap()));
}
