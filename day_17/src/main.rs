/*
https://adventofcode.com/2024/day/17
--- Day 17: Chronospatial Computer ---
 */

use std::io;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Clone)]
struct Machine {
    instruction_ptr: usize,
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

#[derive(Copy, Clone)]
struct ComboOperand(u8);
#[derive(Copy, Clone)]
struct LiteralOperand(u8);

/// Each possible machine instruction with their embedded operand type.
enum Instruction {
    Adv(ComboOperand),
    Bxl(LiteralOperand),
    Bst(ComboOperand),
    Jnz(LiteralOperand),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

use Instruction::*;

impl Machine {
    /// Run the program from its current state/IP until it halts.
    fn run_until_halt(&mut self) {
        while self.execute_one_step() {}
    }

    /// Run the program for exactly one instruction
    /// (at current IP).
    /// Returns true if it executed,
    /// false if it is now halted.
    fn execute_one_step(&mut self) -> bool {
        if self.instruction_ptr >= self.program.len() {
            return false;
        }

        let ins = self.decode_current_instruction();
        self.instruction_ptr += 2;

        match ins {
            // Div A by power of 2, multiple register dest
            Adv(d) | Bdv(d) | Cdv(d) => {
                let den = 1 << self.get_combo_value(d);
                let res = self.register_a / den;
                match ins {
                    Adv(_) => self.register_a = res,
                    Bdv(_) => self.register_b = res,
                    Cdv(_) => self.register_c = res,
                    // rustc should know that we can match only on the first ones ?
                    _ => panic!("Impossible inner match"),
                }
            }
            // bitwise xor
            Bxl(x) => self.register_b ^= x.0 as usize,
            // modulo 8
            Bst(v) => self.register_b = self.get_combo_value(v) % 8,
            // cond jump if A != 0
            Jnz(p) => {
                if self.register_a != 0 {
                    self.instruction_ptr = p.0 as usize;
                }
            }
            // Xor C into B
            Bxc => self.register_b ^= self.register_c,
            // out
            Out(o) => self.output.push((self.get_combo_value(o) % 8) as u8),
        }

        true
    }

    /// Return the Instruction encoded at current IP
    fn decode_current_instruction(&self) -> Instruction {
        let i = self.program[self.instruction_ptr];
        let o = self.program[self.instruction_ptr + 1];

        match i {
            0 => Adv(ComboOperand(o)),
            1 => Bxl(LiteralOperand(o)),
            2 => Bst(ComboOperand(o)),
            3 => Jnz(LiteralOperand(o)),
            4 => Bxc,
            5 => Out(ComboOperand(o)),
            6 => Bdv(ComboOperand(o)),
            7 => Cdv(ComboOperand(o)),
            _ => panic!("Illegal instruction {i}"),
        }
    }

    /// Make the necessary indirection from a combo operand
    /// encoding into the real value/register value.
    fn get_combo_value(&self, o: ComboOperand) -> usize {
        match o.0 {
            0..=3 => o.0.into(),
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Illegal combo operand {}", o.0),
        }
    }

    /// Format the output vector with coma separator
    fn print_output(&self) {
        let s: String = self
            .output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",");
        println!("Program output:");
        println!("{s}");
    }
}

fn main() {
    let mut machine = Machine {
        instruction_ptr: 0,
        register_a: 0,
        register_b: 0,
        register_c: 0,
        program: Vec::<u8>::new(),
        output: Vec::<u8>::new(),
    };
    let mut lines = io::stdin().lock().lines();
    if let Some(Ok(line)) = lines.next() {
        let val = line.split_once(": ").unwrap().1;
        machine.register_a = usize::from_str(val).unwrap();
    }
    if let Some(Ok(line)) = lines.next() {
        let val = line.split_once(": ").unwrap().1;
        machine.register_b = usize::from_str(val).unwrap();
    }
    if let Some(Ok(line)) = lines.next() {
        let val = line.split_once(": ").unwrap().1;
        machine.register_c = usize::from_str(val).unwrap();
    }
    lines.next(); // empty
    if let Some(Ok(line)) = lines.next() {
        let program = line.split_once(": ").unwrap().1;
        machine.program = program
            .split(',')
            .map(|v| u8::from_str(v).unwrap())
            .collect();
    }

    machine.run_until_halt();
    println!("Part1:");
    machine.print_output();
}
