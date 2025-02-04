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

        for (y, row) in v.iter().enumerate() {
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

    /// Fill the grid by replacing all cells by a single value
    pub fn fill(&mut self, t: T) {
        self.s.fill(t);
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

/// Characters to display a 2x2 boolean map.
/// For 4 booleans arranged as:
/// 0 1
/// 2 3
/// the block character is at index N where the 4 bits of N are '3210'
const HALF_BLOCKS: [char; 16] = [
    ' ', '▘', '▝', '▀', '▖', '▌', '▞', '▛', '▗', '▚', '▐', '▜', '▄', '▙', '▟', '█',
];

// A similar table for braille would be 256 character long.
// Too annoying to type and order properly by hand, this is done
// dynamically by a convesion function (It could be made into a macro maybe)

// Note 1: the first char is not a space but braille 0x2800 "dots-0", for consistency.
// Note 2: Due to the split in unicode between the 2x3 and the 2x4 patterns, there is
// not clean binary progression. So I kept the numbering style of HALF_BLOCK (bits increase
// first on the X axis) instead of mapping directly the braille order (bits increase
// first on the Y axis until the 3rd/6th bit, and the 7/8th bit are horizontal in a different block)
// https://en.wikipedia.org/wiki/Braille_Patterns#Block
/*
For reference, the "original" order
6-dots block
'⠀','⠁','⠂','⠃','⠄','⠅','⠆','⠇','⠈','⠉','⠊','⠋','⠌','⠍','⠎','⠏',
'⠐','⠑','⠒','⠓','⠔','⠕','⠖','⠗','⠘','⠙','⠚','⠛','⠜','⠝','⠞','⠟',
'⠠','⠡','⠢','⠣','⠤','⠥','⠦','⠧','⠨','⠩','⠪','⠫','⠬','⠭','⠮','⠯',
'⠰','⠱','⠲','⠳','⠴','⠵','⠶','⠷','⠸','⠹','⠺','⠻','⠼','⠽','⠾','⠿',
8-dots block
7
'⡀','⡁','⡂','⡃','⡄','⡅','⡆','⡇','⡈','⡉','⡊','⡋','⡌','⡍','⡎','⡏',
'⡐','⡑','⡒','⡓','⡔','⡕','⡖','⡗','⡘','⡙','⡚','⡛','⡜','⡝','⡞','⡟',
'⡠','⡡','⡢','⡣','⡤','⡥','⡦','⡧','⡨','⡩','⡪','⡫','⡬','⡭','⡮','⡯',
'⡰','⡱','⡲','⡳','⡴','⡵','⡶','⡷','⡸','⡹','⡺','⡻','⡼','⡽','⡾','⡿',
8
'⢀','⢁','⢂','⢃','⢄','⢅','⢆','⢇','⢈','⢉','⢊','⢋','⢌','⢍','⢎','⢏',
'⢐','⢑','⢒','⢓','⢔','⢕','⢖','⢗','⢘','⢙','⢚','⢛','⢜','⢝','⢞','⢟',
'⢠','⢡','⢢','⢣','⢤','⢥','⢦','⢧','⢨','⢩','⢪','⢫','⢬','⢭','⢮','⢯',
'⢰','⢱','⢲','⢳','⢴','⢵','⢶','⢷','⢸','⢹','⢺','⢻','⢼','⢽','⢾','⢿',
78
'⣀','⣁','⣂','⣃','⣄','⣅','⣆','⣇','⣈','⣉','⣊','⣋','⣌','⣍','⣎','⣏',
'⣐','⣑','⣒','⣓','⣔','⣕','⣖','⣗','⣘','⣙','⣚','⣛','⣜','⣝','⣞','⣟',
'⣠','⣡','⣢','⣣','⣤','⣥','⣦','⣧','⣨','⣩','⣪','⣫','⣬','⣭','⣮','⣯',
'⣰','⣱','⣲','⣳','⣴','⣵','⣶','⣷','⣸','⣹','⣺','⣻','⣼','⣽','⣾','⣿'
*/

/// Use the bit representation of the Unicode Braille Block for
/// conversion instead of mapping into a table.
#[inline]
fn u8_to_braille(v: u8) -> char {
    let base: u32 = 0x2800;
    let v6 = v & 0b00111111;
    // bit 0 stay at "dot 1", bit 1 becomes "dot 4", bit 2 becomes "dot 2",
    // bit 3 becomes "dot 5", bit 4 becomes "dot 3" and bit 5 stays at "dot 6"
    let dot6 =
        (v6 & 1) | (v6 & 2) << 2 | (v6 & 4) >> 1 | (v6 & 8) << 1 | (v6 & 16) >> 2 | (v6 & 32);
    // bit 6 is "dot 7", bit 7 is "dot 8", which are unchanged bit positions.
    let dot8 = dot6 | (v & 0b11000000);
    let braille = base + dot8 as u32;
    char::from_u32(braille).unwrap()
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

    /// Pretty-print a boolean array using block elements Unicode chars for compact representation
    pub fn pretty_print_bool_half(&self) {
        // The only difficulty here is to handle odd width/height
        // when setting the values for the border characters,
        // if we don't want to pay the cost of using checked_get()
        // for all cells.
        eprintln!("[{},{}] = ", self.width, self.height);
        let mut zero_slice = Vec::<bool>::new();
        for y in 0..(self.height + 1) / 2 {
            let top = y * 2;
            let bot = y * 2 + 1;
            let top_slice = self.get_row_slice(top);
            let bot_slice;
            if bot < self.height {
                bot_slice = self.get_row_slice(bot);
            } else {
                zero_slice.resize(self.width, false);
                bot_slice = &zero_slice;
            }

            eprint!("[");
            for x in 0..(self.width + 1) / 2 {
                let left = x * 2;
                let right = x * 2 + 1;
                let b_0 = top_slice[left] as u8;
                let b_1;
                let b_2 = bot_slice[left] as u8;
                let b_3;
                if right < self.width {
                    b_1 = top_slice[right] as u8;
                    b_3 = bot_slice[right] as u8;
                } else {
                    b_1 = 0;
                    b_3 = 0;
                }
                let index = b_0 | (b_1 << 1) | (b_2 << 2) | (b_3 << 3);
                eprint!("{}", HALF_BLOCKS[index as usize]);
            }
            eprintln!("]");
        }
    }

    /// Pretty-print a boolean array using braille Unicode chars for
    /// even more compact representation
    pub fn pretty_print_bool_micro(&self) {
        eprintln!("[{},{}] = ", self.width, self.height);
        let mut zero_slice = Vec::<bool>::new();
        zero_slice.resize(self.width, false);
        for y in 0..(self.height + 3) / 4 {
            let t0 = y * 4;
            let t1 = y * 4 + 1;
            let t2 = y * 4 + 2;
            let t3 = y * 4 + 3;
            let slice0 = self.get_row_slice(t0);
            let slice1;
            let slice2;
            let slice3;
            if t1 < self.height {
                slice1 = self.get_row_slice(t1);
            } else {
                slice1 = &zero_slice;
            }
            if t2 < self.height {
                slice2 = self.get_row_slice(t2);
            } else {
                slice2 = &zero_slice;
            }
            if t3 < self.height {
                slice3 = self.get_row_slice(t3);
            } else {
                slice3 = &zero_slice;
            }

            eprint!("[");
            for x in 0..(self.width + 1) / 2 {
                let left = x * 2;
                let right = x * 2 + 1;
                let b_0 = slice0[left] as u8;
                let b_1;
                let b_2 = slice1[left] as u8;
                let b_3;
                let b_4 = slice2[left] as u8;
                let b_5;
                let b_6 = slice3[left] as u8;
                let b_7;
                if right < self.width {
                    b_1 = slice0[right] as u8;
                    b_3 = slice1[right] as u8;
                    b_5 = slice2[right] as u8;
                    b_7 = slice3[right] as u8;
                } else {
                    b_1 = 0;
                    b_3 = 0;
                    b_5 = 0;
                    b_7 = 0;
                }
                let index = b_0
                    | (b_1 << 1)
                    | (b_2 << 2)
                    | (b_3 << 3)
                    | (b_4 << 4)
                    | (b_5 << 5)
                    | (b_6 << 6)
                    | (b_7 << 7);
                eprint!("{}", u8_to_braille(index));
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

    #[test]
    fn grid_braille_pattern() {
        assert_ne!(u8_to_braille(0), ' '); // we do NOT expect a 0x20 space
        assert_eq!(u8_to_braille(0), '\u{2800}'); // but the dots-0 braille character

        assert_eq!(u8_to_braille(255), '⣿');

        // single bits to dots-1 -- dot-8
        assert_eq!(u8_to_braille(0b00000001), '⠁');
        assert_eq!(u8_to_braille(0b00000010), '⠈');
        assert_eq!(u8_to_braille(0b00000100), '⠂');
        assert_eq!(u8_to_braille(0b00001000), '⠐');
        assert_eq!(u8_to_braille(0b00010000), '⠄');
        assert_eq!(u8_to_braille(0b00100000), '⠠');
        assert_eq!(u8_to_braille(0b01000000), '⡀');
        assert_eq!(u8_to_braille(0b10000000), '⢀');

        assert_eq!(u8_to_braille(0b10110111), '⢯');

        assert_eq!(u8_to_braille(0b01100110), '⡪');

        assert_eq!(u8_to_braille(0b10011001), '⢕');

        assert_eq!(u8_to_braille(0b01010101), '⡇');
        assert_eq!(u8_to_braille(0b10101010), '⢸');
    }
}
