//! Command-line arguments parsing for basic debugging.
//! (No need for full-blow crate like clap)

use std::env;

const DEBUG_FLAG: &str = "-d";
const VERBOSE_FLAG: &str = "-v";

pub fn is_debug() -> bool {
    has_arg(DEBUG_FLAG)
}

pub fn is_verbose() -> bool {
    has_arg(VERBOSE_FLAG)
}

pub fn has_arg(s: &str) -> bool {
    env::args().any(|a| a == s)
}
