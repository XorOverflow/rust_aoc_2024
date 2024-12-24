/*
https://adventofcode.com/2024/day/24
--- Day 24: Crossed Wires ---

 */

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum LogicalOp {
    And,
    Or,
    Xor,
}

impl LogicalOp {
    fn compute(&self, in1: u8, in2: u8) -> u8 {
        match self {
            LogicalOp::And => in1 & in2,
            LogicalOp::Or => in1 | in2,
            LogicalOp::Xor => in1 ^ in2,
        }
    }
}

#[derive(Clone, Debug)]
struct Gate {
    in1: String,
    in2: String,
    out: String,
    op: LogicalOp,
}

// Update the signals/gates by 1 tick (sometimes a bit more
// if signals cascade on next gate in the loop order).
// Returns the list of still undefined gates (speedup test)
fn propagate_signal_once(wires: &mut HashMap<String, u8>, gates: &Vec<Gate>) -> Vec<Gate> {
    let mut remain = Vec::<Gate>::new();
    for g in gates {
        match wires.get(&g.in1) {
            None => remain.push(g.clone()),
            Some(i1) => {
                match wires.get(&g.in2) {
                    None => remain.push(g.clone()),
                    Some(i2) => {
                        let out = g.op.compute(*i1, *i2);
                        match wires.insert(g.out.clone(), out) {
                            Some(_) => panic!("Wire was already computed"),
                            None => (), // ok
                        }
                    }
                }
            }
        }
    }

    remain
}

// Update the signals/gates until no signal remains undefined.
fn propagate_signal(wires: &mut HashMap<String, u8>, gates: &Vec<Gate>) {
    let mut working_gates = gates.clone();

    while working_gates.len() != 0 {
        working_gates = propagate_signal_once(wires, &working_gates);
    }
}

fn parse_z_wires(wires: &HashMap<String, u8>) -> usize {
    // We don't really know or care in advance how many zxx wires
    // were defined.
    // Get them all until we don't find any.
    let mut res = 0;
    for bit in 0.. {
        let z = format!("z{:02}", bit);
        if let Some(val) = wires.get(&z) {
            res = res | (*val as usize) << bit;
        } else {
            break;
        }
    }

    res
}

fn main() {
    let mut wires = HashMap::<String, u8>::new();
    let mut gates = Vec::<Gate>::new();

    let mut lines = io::stdin().lock().lines();

    while let Some(Ok(line)) = lines.next() {
        if line.len() == 0 {
            break;
        }

        let (name, val) = line.split_once(": ").unwrap();
        let name = name.to_string();
        let val = u8::from_str(val).unwrap();
        wires.insert(name, val);
    }

    while let Some(Ok(line)) = lines.next() {
        let (gate, out) = line.split_once(" -> ").unwrap();
        let out = out.to_string();
        let gate: Vec<&str> = gate.split(" ").collect();
        if gate.len() != 3 {
            panic!("Gate format error");
        }
        let in1 = gate[0].to_string();
        let in2 = gate[2].to_string();
        let op = match gate[1] {
            "XOR" => LogicalOp::Xor,
            "AND" => LogicalOp::And,
            "OR" => LogicalOp::Or,
            _ => panic!("Gate logic op unknown"),
        };

        gates.push(Gate { in1, in2, out, op });
    }

    /*
    eprintln!("Parsed initial wires: {:?}", wires);
    eprintln!("Parsed gates: {:?}", gates);
     */

    let mut working_wires = wires.clone();

    propagate_signal(&mut working_wires, &gates);
    //eprintln!("Final wires values: {:?}", working_wires);
    let final_z = parse_z_wires(&working_wires);
    println!("Part 1 = {final_z}");
}
