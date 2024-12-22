/*
https://adventofcode.com/2024/day/21
--- Day 21: Keypad Conundrum ---
*/

/*

+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+



    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+

*/

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

/*
Algo is simple for 1 level of indirection: each key is assigned a coordinate.
Travel between keys is a sequence of N successive <v>^, N being the
manhattan distance between the coordinates of the first level keys.
There are in general only 2 possible path, one taking the X axis
first, and the other using the Y axis first. On straigt direction
there is only one, and if we avoid The Gap there is also only one.
They all have the same sequence length.

However, for 2nd indirection, and with the constraint of NOT hovering above
the keypad gap, its not clear yet if the 1st level path of <<<^ can
be typed with the same number of indirection as the path of ^<<<.
Visually, it seems that if the robot needs to push the A key at the
end of each sequence, ending a sequence with ^ or > is closer to A
than ending with < or v.

We SUPPOSE that to solve this problem, a non-intuitive path (that
is not going streight) is never needed to be more efficient at a
higher indirection level.
*/

/*
Coordinates of each key on each keypad.
The Gap is named '!' for the warning.
*/

fn get_numeric_keypad_map() -> HashMap<char, (usize, usize)> {
    let mut h = HashMap::<char, (usize, usize)>::new();
    h.insert('7', (0, 0));
    h.insert('8', (1, 0));
    h.insert('9', (2, 0));
    h.insert('4', (0, 1));
    h.insert('5', (1, 1));
    h.insert('6', (2, 1));
    h.insert('1', (0, 2));
    h.insert('2', (1, 2));
    h.insert('3', (2, 2));
    h.insert('!', (0, 3));
    h.insert('0', (1, 3));
    h.insert('A', (2, 3));

    h
}

fn get_directional_keypad_map() -> HashMap<char, (usize, usize)> {
    let mut h = HashMap::<char, (usize, usize)>::new();
    h.insert('!', (0, 0));
    h.insert('^', (1, 0));
    h.insert('A', (2, 0));
    h.insert('<', (0, 1));
    h.insert('v', (1, 1));
    h.insert('^', (2, 1));

    h
}

fn direction_sign_to_key_letter(dx: isize, dy: isize) -> char {
    match (dx, dy) {
        (1, 0) => '>',
        (-1, 0) => '<',
        (0, 1) => 'v',
        (0, -1) => '^',
        _ => panic!("invalid direction"),
    }
}

// Returns the direction keys to press (literals <>^vA) to press in order
// to press the remote button at "target" starting from "start".
// The GAP coordinate to avoid is passed in "forbidden".
// Returns a vector of all possible, allowed sequences "<<<^A"+"^<<<A"

fn coordinates_to_possible_directions(
    start: (usize, usize),
    target: (usize, usize),
    forbidden: (usize, usize),
) -> Vec<String> {
    let forbidden = (forbidden.0 as isize, forbidden.1 as isize);
    let start: (isize, isize) = (start.0 as isize, start.1 as isize);
    let target: (isize, isize) = (target.0 as isize, target.1 as isize);
    let directions: (isize, isize) = (target.0 - start.0, target.1 - start.1);

    let directions_sign: (isize, isize) = (directions.0.signum(), directions.1.signum());

    let mut horizontal = String::new();
    if directions.0 != 0 {
        let c = direction_sign_to_key_letter(directions_sign.0, 0);
        horizontal = std::iter::repeat(c)
            .take(directions.0.abs() as usize)
            .collect();
    }

    let mut vertical = String::new();
    if directions.1 != 0 {
        let c = direction_sign_to_key_letter(0, directions_sign.1);
        vertical = std::iter::repeat(c)
            .take(directions.1.abs() as usize)
            .collect();
    }

    let mut sequences = Vec::<String>::new();

    if horizontal.len() == 0 {
        //only one vertical sequence
        sequences.push(vertical);
    } else if vertical.len() == 0 {
        sequences.push(horizontal);
    } else {
        // One horizontal and one vertical,
        // both orders possible.
        // Exclude one if the "forbidden" is
        // at the corner of start-end

        // forbidden on same Y as start, and same X
        // as target ? Dont do horizontal first
        if !(forbidden.1 == start.1 && forbidden.0 == target.0) {
            let mut hv = horizontal.clone();
            hv.push_str(&vertical);
            sequences.push(hv);
        }

        // Dont do vertical first if it would hit forbidden()
        if !(forbidden.0 == start.0 && forbidden.1 == target.1) {
            let mut vh = vertical;
            vh.push_str(&horizontal);
            sequences.push(vh);
        }
    }

    // In any case, finalize with a "A"
    for s in &mut sequences {
        s.push_str("A");
    }

    sequences
}

// Return a VEC (complete chained sequence of all keys in the code)
// of multiple VEC (all valid paths joining successive keys)
// of String (the list of keys to press on the keypad)
fn keypad_code_to_directions(
    code: &String,
    keymap: &HashMap<char, (usize, usize)>,
) -> Vec<Vec<String>> {
    let mut start = *keymap.get(&'A').unwrap();
    let forbidden = *keymap.get(&'!').unwrap();
    let mut ret = Vec::<Vec<String>>::new();
    for c in code.chars() {
        let end = *keymap.get(&c).unwrap();
        let keys = coordinates_to_possible_directions(start, end, forbidden);
        ret.push(keys.clone());
        //eprintln!(" -> {} : paths = {:?}", *c, keys);
        start = end;
    }

    ret
}

// Create all possible combinations of concatenations of s[n] with s[n+1]
fn flatten_possibilites_of_sequences(sequences: &[Vec<String>]) -> Vec<String> {
    if sequences.len() == 1 {
        return sequences[0].clone();
    }

    let start = &sequences[0];
    let tail = &sequences[1..];
    let flattened_tail = flatten_possibilites_of_sequences(tail);
    let mut result = Vec::<String>::new();
    for s in start {
        for t in &flattened_tail {
            let mut s_ext = s.clone();
            s_ext.push_str(t);
            result.push(s_ext);
        }
    }

    result
}

// Input is the output of keypad_code_to_directions.
// this input is a list of possible sequences to type a key; each possible
// sequence is combined to other sequences to form multiple total sequences
// (all possible ways to type the complete code like "123A").
// each possible way returns a new keypad_code_to_directions() array.
fn sequences_to_directions(
    sequences: &Vec<Vec<String>>,
    keymap: &HashMap<char, (usize, usize)>,
) -> Vec<Vec<String>> {
    let flattened = flatten_possibilites_of_sequences(sequences);

    let mut ret = Vec::<Vec<String>>::new();
    ret.push(flattened);

    ret
}

fn extract_numeric(c: &str) -> usize {
    //    let s: String = c.iter().collect();
    let s = c.strip_suffix('A').unwrap();
    let s = s.trim_start_matches('0');

    if s.len() == 0 {
        0
    } else {
        usize::from_str(s).unwrap()
    }
}

fn main() {
    let mut codes = Vec::<String>::new();

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        let codeline = line;
        codes.push(codeline);
    }

    let nmap = get_numeric_keypad_map();
    let dmap = get_directional_keypad_map();

    for code in codes {
        let num = extract_numeric(&code);
        let robot1_door = keypad_code_to_directions(&code, &nmap);
        eprintln!("{num} => {:?}", robot1_door);
        let robot2_radiation = sequences_to_directions(&robot1_door, &dmap);
        eprintln!(" => {:?}", robot2_radiation);
        //let robot3_freezer = keypad_code_to_directions(&robot2_radiation, &dmap);
        //let human4 = keypad_code_to_directions(&robot3_freezer, &dmap);
    }
}

#[test]
fn check_basic_path_with_forbidden_gap() {
    let n1 = get_numeric_keypad_map();

    let test_case: String = "1006A".to_string();
    // This requires that the arbitrary order HV+VH is kept and not
    // reversed.
    let expected_paths = [
        vec!["^<<A"],
        vec![">vA"],
        vec!["A"],
        vec![">^^A", "^^>A"],
        vec!["vvA"],
    ];

    let found_paths = keypad_code_to_directions(&test_case, &n1);
    assert_eq!(found_paths, expected_paths);
}
