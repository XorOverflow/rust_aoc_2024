/*
https://adventofcode.com/2024/day/4
--- Day 4: Ceres Search ---
 */

use std::io;

// Input grid stored with a margin on right and bottom
// to avoid the need for constant bound checking.
type Grid = Vec<Vec<u8>>;

fn count_xmas(grid: &Grid) -> usize {
    // get the real size without padding.
    let height = grid.len() - 4;
    let width = grid[0].len() - 4;

    let searched = vec![vec!['X', 'M', 'A', 'S'],
                        vec!['S', 'A', 'M', 'X']];

    // Look for the searched string (to simulate
    // bacward and forward search) on right, down, and
    // diagonal down-right, on all coordinates.

    // Used to display the result at the end
    let debug = false;
    let mut debug_grid = grid.clone();
    
    let mut found:usize = 0;
    for x in 0..width {
        for y in 0..height {
            // hackish way to count the number of found
            // by removing the number of non-found, this
            // simplify the handling of for-break.
            let mut potential_found = searched.len() * 3;
            for s in &searched {
                // right search
                for k in 0..s.len() {
                    if s[k] as u8 != grid[y][x+k] {
                        potential_found -= 1;
                        break;
                    }
                }
                // down search
                for k in 0..s.len() { 
                    if s[k] as u8 != grid[y+k][x] {
                        potential_found -= 1;
                        break;
                    }
                }
                // diagonal search down-right
                for k in 0..s.len() {
                    if s[k] as u8 != grid[y+k][x+k] {
                        potential_found -= 1;
                        break;
                    }
                }
                // diagonal search up-right (only if low enough )
                if y >= 3 {
                    potential_found += 1;
                    for k in 0..s.len() {
                        if s[k] as u8 != grid[y-k][x+k] {
                            potential_found -= 1;
                            break;
                        }
                    }
                }
            }

            if debug && potential_found != 0 {
                eprintln!("Found {potential_found} at {x},{y}");
                debug_grid[y][x] = format!("{potential_found}").as_bytes()[0];
            }
            found += potential_found;
        }
    }

    if debug {
        eprintln!("debug found:");
        for k in debug_grid {
            eprintln!("{}", String::from_utf8(k).expect("error"));
        }
    }
    
    return found;
}

fn main() {

    let mut grid = Grid::new();
    
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Err(_) => {
                panic!("input error, exit");
            }
            Ok(0) => {
                eprintln!("Eof detected");
                break;
            }
            Ok(_) => {
                // remove the \n
                // expect input is pure ascii and each u8 is
                // one ascii char
                // Add a padding on the right of the same size as "xmas"
                // to avoid bound-checking during search
                let line_ascii = input.trim().to_string();
                let line_ascii = line_ascii + "....";
                let line_ascii = line_ascii.as_bytes();
                grid.push(line_ascii.into());
            }
        }
        // must clear for next loop
        input = String::from("");
    }
    // Add 4 more empty lines on the bottom
    let binding = ".".repeat(grid[0].len());
    let padding_line = binding.as_bytes();
    for _ in 0..4 {
        grid.push(padding_line.to_vec());
    }

    // part 1
    let count = count_xmas(&grid);
    println!("Part1 = {count}");
}
