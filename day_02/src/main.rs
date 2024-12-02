/*
https://adventofcode.com/2024/day/2
--- Day 2: Red-Nosed Reports ---
 */

use std::io;
use std::str::FromStr;

fn is_report_safe(report: &[i32]) -> bool {
    let delta: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();
    //eprintln!("delta of {:?} = {:?}", report, delta);

    // Safe == delta are all in [-3,-1] or all in [1,3].
    // Test and perform early exit in a for loop.
    // Map or filter would require scanning all and
    // then counting the number of bad difference which is more
    // annoying.
    let mut direction: i32 = 0;
    for d in &delta {
        let d = *d;
        if !(1..=3).contains(&d.abs()) {
            // bad range anyway
            return false;
        }
        if direction == 0 {
            // Memorize initial levels direction.
            direction = d;
        } else if d * direction < 0 {
            // mixed increase and decrease
            return false;
        }
    }

    // All checks passed.
    true
}

fn is_report_safe_with_dampener(report: &Vec<i32>) -> bool {
    // Dampening works by removing a single level,
    // not a single delta between levels.
    // No simple formula a priori to detect "the" wrong
    // level from the wrong delta. It could even
    // be due to a wrong initial direction.
    // When in doubt, brute-force.

    if is_report_safe(report) {
        true
    } else {
        // retry by checking all possible 1-element removal.
        for k in 0..report.len() {
            let mut test_report = report.clone();
            test_report.remove(k);
            if is_report_safe(&test_report) {
                //eprintln!("Made {:?} safe by removing element {k}", report);
                return true;
            }
        }
        false
    }
}

fn main() {
    // List of reports, which are lists of levels
    let mut reports = Vec::<Vec<i32>>::new();

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
                let input_clean = input.trim();
                let levels: Vec<i32> = input_clean
                    .split(' ')
                    .map(|i| i32::from_str(i).unwrap())
                    .collect();
                reports.push(levels);
            }
        }
        // must clear for next loop
        input = String::from("");
    }

    // Part 1
    let safe_count: u32 = reports.iter().map(|r| is_report_safe(r) as u32).sum();
    println!("Safe reports = {safe_count}");

    // Part 2
    let safe_count: u32 = reports
        .iter()
        .map(|r| is_report_safe_with_dampener(r) as u32)
        .sum();
    println!("Safe reports with dampener = {safe_count}");
}
