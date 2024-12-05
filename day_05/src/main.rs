/*
https://adventofcode.com/2024/day/5
--- Day 5: Print Queue ---
 */

use std::collections::HashSet;
use std::io;
use std::str::FromStr;

// List all the pairs of "N|M" as a tuple (N,M).
// each N and M can appear multiple times in the list with
// different pair values, all combinations must be kept (it's not an
// ordered set)
type Order = HashSet<(u8, u8)>;

// Check all possible ordered pairs of value in the list,
// and check if they appear in the order map.
fn update_is_valid(order: &Order, update: &Vec<u8>) -> bool {
    for before in 0..update.len() - 1 {
        for after in (before + 1)..update.len() {
            let pair = (update[before], update[after]);
            if !order.contains(&pair) {
                return false;
            }
        }
    }

    true
}

fn middle_page(update: &Vec<u8>) -> u8 {
    update[(update.len() - 1) / 2]
}

fn count_correct_order(order: &Order, update_pages: &Vec<Vec<u8>>) -> usize {
    update_pages.iter().fold(0_usize, |acc, p| {
        if update_is_valid(order, p) {
            acc + middle_page(p) as usize
        } else {
            //eprintln!("{:?} INvalid", p);
            acc
        }
    })
}

// change the order of list of page to conform to the order map.
fn rearrange(order: &Order, update: &Vec<u8>) -> Vec<u8> {
    let mut rearranged = update.clone();

    // Perform a kind of bubble-sort by swapping two values whenever they
    // are not in the correct order.
    // Seems to work without getting stuck in an infinite loop, can't
    // say why it's safe.
    'outer: loop {
        for before in 0..rearranged.len() - 1 {
            for after in (before + 1)..rearranged.len() {
                let pair = (rearranged[before], rearranged[after]);
                if !order.contains(&pair) {
                    // problematic pair. Swap it.
                    rearranged[before] = pair.1;
                    rearranged[after] = pair.0;
                    continue 'outer;
                }
            }
        }
        break; // no wrong pair found
    }

    //eprintln!("Modified {:?} into {:?}", update, rearranged);

    rearranged
}

fn count_rearranged_order(order: &Order, update_pages: &Vec<Vec<u8>>) -> usize {
    update_pages.iter().fold(0_usize, |acc, p| {
        if update_is_valid(order, p) {
            acc
        } else {
            let solved = rearrange(order, p);
            acc + middle_page(&solved) as usize
        }
    })
}

fn main() {
    // By default all pages must be printed before "page 999"
    let mut order = Order::new();
    let mut update_pages = Vec::<Vec<u8>>::new();

    let mut input = String::new();
    // 1- Read order map
    loop {
        match io::stdin().read_line(&mut input) {
            Err(_) => {
                panic!("input error, exit");
            }
            Ok(0) => {
                panic!("Eof detected during order list parsing");
            }
            Ok(_) => {
                // remove the \n
                let line = input.trim().to_string();
                if line.len() == 0 {
                    // empty line separator with next section
                    break;
                }
                let ord: Vec<u8> = line.split('|').map(|i| u8::from_str(i).unwrap()).collect();

                order.insert((ord[0], ord[1]));
                let check = (ord[0], ord[1]);
                if !order.contains(&check) {
                    panic!("Added {:?} but not founding it back", check);
                }
            }
        }
        input = String::from("");
    }

    //eprintln!("Order map = {:?}", order);

    // 2- Read the page lists
    input = String::from("");
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
                let pages: Vec<u8> = input
                    .trim()
                    .to_string()
                    .split(',')
                    .map(|i| u8::from_str(i).unwrap())
                    .collect();
                update_pages.push(pages);
            }
        }
        input = String::from("");
    }

    // part 1
    let count = count_correct_order(&order, &update_pages);
    println!("Part1 = {count}");

    // part 2
    let count = count_rearranged_order(&order, &update_pages);
    println!("Part2 = {count}");
}
