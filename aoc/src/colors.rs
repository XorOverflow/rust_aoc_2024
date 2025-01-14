//! Terminal ANSI color codes.

pub const ANSI_RESET: &str = "\x1B[0m";

// black, red, green, yellow, blue, magenta, cyan, white
pub const BLACK: usize = 0;
pub const RED: usize = 1;
pub const GREEN: usize = 2;
pub const YELLOW: usize = 3;
pub const BLUE: usize = 4;
pub const MAGENTA: usize = 5;
pub const CYAN: usize = 6;
pub const WHITE: usize = 7;

#[rustfmt::skip]
pub const FG_COLORS: [&str; 8] = [
    "\x1B[30m",
    "\x1B[31m",
    "\x1B[32m",
    "\x1B[33m",
    "\x1B[34m",
    "\x1B[35m",
    "\x1B[36m",
    "\x1B[37m",
];

// Brighter version
#[rustfmt::skip]
pub const FG_BRIGHT_COLORS: [&str; 8] = [
    "\x1B[90m",
    "\x1B[91m",
    "\x1B[92m",
    "\x1B[93m",
    "\x1B[94m",
    "\x1B[95m",
    "\x1B[96m",
    "\x1B[97m",
];

// Same but for background
#[rustfmt::skip]
pub const BG_COLORS: [&str; 8] = [
    "\x1B[40m",
    "\x1B[41m",
    "\x1B[42m",
    "\x1B[43m",
    "\x1B[44m",
    "\x1B[45m",
    "\x1B[46m",
    "\x1B[47m",
];

#[rustfmt::skip]
pub const BG_BRIGHT_COLORS: [&str; 8] = [
    "\x1B[100m",
    "\x1B[101m",
    "\x1B[102m",
    "\x1B[103m",
    "\x1B[104m",
    "\x1B[105m",
    "\x1B[106m",
    "\x1B[107m",
];
