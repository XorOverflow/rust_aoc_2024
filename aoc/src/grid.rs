//! "Grid" storage (2D array).

use crate::colors;
use std::boxed::Box;

// A custom 2D array more friendly than a Vec<Vec<T>>
#[derive(Clone)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    s: Box<[T]>,
}

impl<T: std::clone::Clone> Grid<T> {
    /// Allocate the low-level array for this grid with a default value
    pub fn new(width: usize, height: usize, t0: T) -> Self {
        Self {
            width,
            height,
            s: vec![t0; width * height].into_boxed_slice(),
        }
    }

    /// Convert a double-vector into a grid.
    /// Internal vectors are supposed to all be the same length;
    /// the first one is taken as the "width" of the final grid,
    /// if others are smaller gaps are filled with copies of
    /// the first element found, if they are bigger a panic will occur.
    pub fn from_vec(v: &[Vec<T>]) -> Self {
        let t0 = v[0][0].clone();
        let mut s = Self::new(v[0].len(), v.len(), t0);

        for (y,row) in v.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                s.set(x, y, val.clone());
            }
        }
        s
    }

    /// Get a value if inbound.
    /// Input coordinates are isize instead of usize on purpose,
    /// for easier calls from pratical implementations of AOC
    /// "puzzle" solvers where coordinates and vectors often need
    /// to be signed.
    pub fn checked_get(&self, x: isize, y: isize) -> Option<T> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            None
        } else {
            Some(self.s[x as usize + (y as usize) * self.width].clone())
        }
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        if x >= self.width || y >= self.height {
            panic!("array access {},{} out of bounds", x, y);
        } else {
            self.s[x + y * self.width].clone()
        }
    }

    // return a slice of size "width" of all elements of row Y
    pub fn get_row_slice(&self, y: usize) -> &[T] {
        if y >= self.height {
            panic!("array row {y} out of bounds");
        }
        &self.s[y * self.width..y * self.width + self.height]
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            None
        } else {
            Some(&mut self.s[x as usize + (y as usize) * self.width])
        }
    }

    // todo: provide a macro
    pub fn set(&mut self, x: usize, y: usize, t: T) {
        if x >= self.width || y >= self.height {
            panic!("array access {},{} out of bounds", x, y);
        } else {
            self.s[x + y * self.width] = t;
        }
    }
}

impl<T: PartialEq + std::clone::Clone> Grid<T> {
    /// Check if the grid values at the two different coordinates are equal.
    /// Any out-of-bound coordinates simply return false.
    pub fn values_equal(&self, x1: isize, y1: isize, x2: isize, y2: isize) -> bool {
        if let Some(v1) = self.checked_get(x1, y1) {
            if let Some(v2) = self.checked_get(x2, y2) {
                return v1 == v2;
            }
        }

        false
    }
}

impl<T: std::clone::Clone + std::fmt::Display> Grid<T> {
    /// Pretty-print the array with default Display trait
    pub fn pretty_print(&self) {
        eprintln!("[{},{}] = ", self.width, self.height);
        for y in 0..self.height {
            eprint!("[ ");
            for x in 0..self.width {
                eprint!("{} ", self.get(x, y));
            }
            eprintln!("]");
        }
    }
}

impl<T: std::clone::Clone> Grid<T> {
    /// Pretty-print the array with any user-supplied function to convert
    /// between the type and a single char (not simply a "Display" trait)
    pub fn pretty_print_lambda_char(&self, f: &dyn Fn(T) -> char) {
        eprintln!("[{},{}] = ", self.width, self.height);
        for y in 0..self.height {
            let s: String = (0..self.width).map(|x| f(self.get(x, y))).collect();
            eprintln!("[{}] ", s);
        }
    }

    /// Pretty-print the array with any user-supplied function to convert
    /// between the type and any string (should all be the same size for
    /// alignment)
    pub fn pretty_print_lambda(&self, f: &dyn Fn(T) -> String) {
        eprintln!("[{},{}] = ", self.width, self.height);
        for y in 0..self.height {
            let s: String = (0..self.width).map(|x| f(self.get(x, y))).collect();
            eprintln!("[{}] ", s);
        }
    }

    /// Pretty-print the array with any user-supplied function,
    /// using a second grid for additional information.
    /// The two grids must have the same dimension.
    /// Automatically emits the \esc[0m terminal color reset at end of line.
    pub fn pretty_print_lambda_with_overlay<T2: std::clone::Clone>(
        &self,
        overlay: &Grid<T2>,
        f: &dyn Fn(T, T2, (usize, usize)) -> String,
    ) {
        assert_eq!((self.width, self.height), (overlay.width, overlay.height));
        eprintln!("[{},{}] = ", self.width, self.height);
        for y in 0..self.height {
            let s: String = (0..self.width)
                .map(|x| f(self.get(x, y), overlay.get(x, y), (x, y)))
                .collect();
            eprintln!("[{}{}] ", s, colors::ANSI_RESET);
        }
    }
}

impl Grid<bool> {
    /// Pretty-print a boolean array, true maps to '*'
    pub fn pretty_print_bool(&self) {
        eprintln!("[{},{}] = ", self.width, self.height);
        for y in 0..self.height {
            eprint!("[");
            for x in 0..self.width {
                eprint!("{}", if self.get(x, y) { '*' } else { '.' });
            }
            eprintln!("]");
        }
    }
}

/// A builder to construct a Grid by parsing lines
/// one by one (without knowing the final size)
/// (Note: this is not strictly the Builder Pattern, needs a better name ?)
#[derive(Clone)]
pub struct GridBuilder<T> {
    width: usize,
    height: usize,
    s: Vec<T>,
}

impl<T: std::clone::Clone> Default for GridBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::clone::Clone> GridBuilder<T> {
    /// Create an initially empty builder.
    pub fn new() -> Self {
        GridBuilder {
            width: 0,
            height: 0,
            s: Vec::<T>::new(),
        }
    }

    /// Add a new row at the end of the builder.
    /// When it's the first time, this row defines the width
    /// of the grid. All other rows must have the same width
    /// else a panic is emitted.
    pub fn append_line(&mut self, line: &[T]) {
        if self.height == 0 {
            self.width = line.len();
        } else if self.width != line.len() {
            panic!(
                "Row of len {} appended to GridBuilder of width {}",
                line.len(),
                self.width
            );
        }

        self.height += 1;
        self.s.extend_from_slice(line);
    }

    /// Convert into the final Grid when nothing else needs appending.
    pub fn to_grid(self) -> Grid<T> {
        if self.s.is_empty() {
            panic!("GridBuilder is still empty");
        }
        Grid::<T> {
            width: self.width,
            height: self.height,
            s: self.s.into_boxed_slice(),
        }
    }
}

impl GridBuilder<bool> {
    /// Add a new row at the end of the builder, converting
    /// chars into a boolean according to a match.
    /// When it's the first time, this row defines the width
    /// of the grid. All other rows must have the same width
    /// else a panic is emitted.
    pub fn append_char_map(&mut self, line: &str, true_char: char) {
        let mut bools = Vec::<bool>::with_capacity(self.width);
        for c in line.chars() {
            bools.push(c == true_char);
        }
        if self.height == 0 {
            self.width = bools.len();
        } else if self.width != bools.len() {
            panic!(
                "Row of len {} appended to GridBuilder of width {}",
                bools.len(),
                self.width
            );
        }

        self.height += 1;
        self.s.extend_from_slice(&bools);
    }
}

impl GridBuilder<usize> {
    /// Add a new row at the end of the builder, converting
    /// chars into single digits.
    /// When it's the first time, this row defines the width
    /// of the grid. All other rows must have the same width
    /// else a panic is emitted.
    pub fn append_char_map(&mut self, line: &str) {
        let mut digits = Vec::<usize>::with_capacity(self.width);
        for c in line.chars() {
            digits.push(c.to_digit(10).unwrap() as usize);
        }
        if self.height == 0 {
            self.width = digits.len();
        } else if self.width != digits.len() {
            panic!(
                "Row of len {} appended to GridBuilder of width {}",
                digits.len(),
                self.width
            );
        }

        self.height += 1;
        self.s.extend_from_slice(&digits);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn grid_builder_1() {
        let mut gb = GridBuilder::<usize>::new();
        gb.append_line(&vec![1, 2, 3, 4]);
        gb.append_line(&vec![10, 20, 30, 40]);
        gb.append_line(&vec![100, 200, 300, 400]);
        let mut grid = gb.to_grid();

        assert_eq!(grid.width, 4);
        assert_eq!(grid.height, 3);

        assert_eq!(grid.get(1, 2), 200);
        assert_eq!(grid.checked_get(-1, 0), None);
        assert_eq!(grid.checked_get(0, -1), None);
        assert_eq!(grid.checked_get(0, 4), None);
        assert_eq!(grid.checked_get(4, 0), None);
        assert_eq!(grid.checked_get(0, 0), Some(1));

        assert_eq!(grid.get(1, 1), 20);
        grid.set(1, 1, 999);
        assert_eq!(grid.get(1, 1), 999);

        assert_eq!(grid.get(3, 2), 400);
        if let Some(x) = grid.get_mut(3, 2) {
            assert_eq!(*x, 400);
            *x = 888;
        } else {
            panic!("could not get_mut(3,2)");
        }
        assert_eq!(grid.get(3, 2), 888);
    }

    #[derive(Clone, PartialEq, Eq, Debug)]
    struct Elmt {
        v: usize,
        dir: isize,
    }

    #[test]
    fn grid_from_vec() {
        let eltz = Elmt { v: 100, dir: -4 };

        let elt1: Vec<Elmt> = vec![
            Elmt { v: 42, dir: -1 },
            Elmt { v: 12, dir: 1 },
            Elmt { v: 0, dir: 2 },
        ];
        let elt2: Vec<Elmt> = vec![Elmt { v: 98, dir: 4 }, Elmt { v: 99, dir: 0 }, eltz.clone()];
        let list = vec![elt1, elt2];
        let grid = Grid::<Elmt>::from_vec(&list);

        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 2);

        assert_eq!(grid.checked_get(4, 0), None);
        assert_eq!(grid.checked_get(2, 1), Some(eltz));

        // To show the output in cargo test, run with
        // cargo test --lib -- --nocapture
        // Should display:
        // [<>2]
        // [A*V]

        // & before lambda required for compilation, as its a &dyn,
        // passed borrowed and not copied.
        grid.pretty_print_lambda_char(&|e: Elmt| match e.dir {
            0 => '*',
            1 => '>',
            -1 => '<',
            2 => '2',
            4 => 'A',
            -4 => 'V',
            _ => '?',
        });

        grid.pretty_print_lambda(&|e: Elmt| format!("{:02}_{}|", e.v, e.dir));
    }
}
