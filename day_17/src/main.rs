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

    /// Run the program. If the output is different
    /// than the program code, stops and return false.
    /// If it halts and output == program, return true.
    fn run_until_halt_or_non_quine(&mut self) -> bool {
        self.run_until_halt_or_non_quine_or_outlen(self.program.len() + 1)
    }

    /// Run the program. If the output is different
    /// than the program code, stops and return false.
    /// If it halts, or the output length reached the
    /// specified size, and output == program, return true.
    fn run_until_halt_or_non_quine_or_outlen(&mut self, maxlen: usize) -> bool {
        // Compare only new "out" elements (no need
        // to compare the full array every time)
        let mut checked_len = 0;
        loop {
            let halted = !self.execute_one_step();
            let out_len = self.output.len();
            if out_len > checked_len {
                if out_len > self.program.len() {
                    // output longer than program
                    //println!("output too long");
                    //self.print_output();
                    return false;
                }
                if self.output[out_len - 1] != self.program[out_len - 1] {
                    // latest element differs
                    //println!("output differs at end");
                    //self.print_output();
                    return false;
                }
                checked_len = out_len;
            }
            if out_len == maxlen {
                return true;
            }
            if halted {
                //self.print_output();
                return self.program.len() == self.output.len();
            }
        }
    }

    /// Reboots the machine with a specific starting register value.
    fn reset_with_register(&mut self, a: usize) {
        self.instruction_ptr = 0;
        self.register_a = a;
        self.register_b = 0;
        self.register_c = 0;
        self.output.truncate(0);
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
            // Div A by power of 2 (= bit shift), multiple register dest
            Adv(d) | Bdv(d) | Cdv(d) => {
                let res = self.register_a >> self.get_combo_value(d);
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
        self.decode_instruction_at(self.instruction_ptr)
    }

    /// Return the Instruction encoded at any valid program offset
    fn decode_instruction_at(&self, p: usize) -> Instruction {
        let i = self.program[p];
        let o = self.program[p + 1];

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

    fn get_combo_representation(o: ComboOperand) -> String {
        match o.0 {
            0..=3 => o.0.to_string(),
            4 => String::from("A"),
            5 => String::from("B"),
            6 => String::from("C"),
            _ => String::from("???"),
        }
    }

    /// For debug: output readable assembly
    fn pretty_print_assembly(&self) {
        for k in (0..self.program.len()).step_by(2) {
            eprint!("{:02}: ", k);
            let ins = self.decode_instruction_at(k);
            match ins {
                Adv(d) => eprintln!("ADV {}", Self::get_combo_representation(d)),
                Bdv(d) => eprintln!("BDV {}", Self::get_combo_representation(d)),
                Cdv(d) => eprintln!("CDV {}", Self::get_combo_representation(d)),
                Bxl(x) => eprintln!("BXL {}", x.0),
                Bst(v) => eprintln!("BST {}", Self::get_combo_representation(v)),
                Jnz(p) => eprintln!("JNZ {}", p.0),
                // Xor C into B
                Bxc => eprintln!("BXC"),
                // out
                Out(o) => eprintln!("OUT {} % 8", Self::get_combo_representation(o)),
            }
        }
    }

    fn pretty_print_pseudocode(&self) {
        for k in (0..self.program.len()).step_by(2) {
            eprint!("{:02}: ", k);
            let ins = self.decode_instruction_at(k);
            match ins {
                Adv(d) => eprintln!("A = A >> {}", Self::get_combo_representation(d)),
                Bdv(d) => eprintln!("B = A >> {}", Self::get_combo_representation(d)),
                Cdv(d) => eprintln!("C = A >> {}", Self::get_combo_representation(d)),
                Bxl(x) => eprintln!("B = B xor {}", x.0),
                Bst(v) => eprintln!("B = {} % 8", Self::get_combo_representation(v)),
                Jnz(p) => eprintln!("If A != 0 JMP {}", p.0),
                Bxc => eprintln!("B = B xor C"),
                // out
                Out(o) => eprintln!("OUT {} % 8", Self::get_combo_representation(o)),
            }
        }
    }
}

fn brute_force(machine: &mut Machine) {
    let lower_a = 1 << (machine.program.len() - 1) * 3;
    let higher_a = 2 << (machine.program.len() - 1) * 3;
    println!("Range : {lower_a} .. {higher_a}");
    for k in lower_a..higher_a {
        if k % 100000 == 0 {
            eprintln!("testing {k}...");
        }
        machine.reset_with_register(k);
        if machine.run_until_halt_or_non_quine() {
            machine.print_output();
            println!("Part2 : Register A value for Quine = {k}");
            break;
        }
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

    println!("============");
    machine.pretty_print_assembly();
    println!("============");
    machine.pretty_print_pseudocode();
    println!("============");

    machine.run_until_halt();
    println!("Part1:");
    machine.print_output();

    // Brute_forcing didn't find after 2151200000 iterations.
    // reading the assembly output, my input is a loop
    // making some operations and dividing A by 2^3 (8), seems
    // to be a shifting of a long int 3 bits by 3 bits on each loop.
    // Program halts by a final JNZ whenever A is 0 at this point.
    // For a program of length 16, and output of the same size,
    // we need exactly 16 loops, so A must contains non-zero bits
    // in its bits located at 16*3[0,+1,+2] (or 15*3 ?)and none above.

    // This leads to a range of 35184372088832..
    //                          70368744177664,
    // not really tractable either.
    // Actual result            107416732707226 so there was one magnitude error...

    
    if false {
        brute_force(&mut machine);
    }

    // Obviously A itself contains some encoded version of the program
    // decoded octal by octal. Xor and dynamic shifts don't make it
    // easy to reverse-engineer; ideally we want to construct A by running the
    // program in reverse starting from the expected output.
    // However, the xor are limited, by the literal operand, to 0..7,
    // and code dissassembly show that indirect shifts (by ComboOperand)
    // use registers that are themselves assigned by BST (3 bits max) and xored
    // with other literals. So total range of "cascading" bits from A to the final
    // output value does not grow above something like 8 bits.

    /*
    ============
    00: BST A
    02: BXL 5
    04: CDV B
    06: BXL 6
    08: BXC
    10: OUT B % 8
    12: ADV 3
    14: JNZ 0
    ============
    00: B = A % 8
    02: B = B xor 5
    04: C = A >> B
    06: B = B xor 6
    08: B = B xor C
    10: OUT B % 8
    12: A = A >> 3
    14: If A != 0 JMP 0
    ============
     */

    // Therefore, each generated output is only depending on a limited range of lower bits
    // from A at each step (A being shifted each time).
    // We can chunk the search output number by output number instead of all at once:
    // Once the first valid out number (equals to the first program number) has found all its possible
    // A register values generating it, from "slightly brute-forcing" only on a range of 0..256,
    // the second out number will be generated from a slightly
    // modified A, shifted by 3 bits and a few xors.

    let mut valid_a = Vec::<usize>::new();

    //valid_a.push(0); // Just to avoid special_casing the first digit
    valid_a.push(42567035290); // This seed is the decimal version of the truncated octal value common to all results
                               // ( 0x42567035290) found by initial algorithm but which were "too high", generating all 14 first digits.
                               // This leads to the correct result 107416732707226 (oct 0x3033075014424632) instead
                               // of the one found by first algo,  107416748386714 (oct 0x3033075110264632)

    // Still searching why the standard starting point overshoots

    // loop search takes 7s from starting at 0, and 20s starting from the "magic" seed.
    
    let mut range_factor = 1;
    let bits = 8;

    for digit in 1..=machine.program.len() {
        let mut next_valid_a = Vec::<usize>::new();

        for prev in &valid_a {
            // "256" was too low (no matching digit after 7), 512 is goodenough.
            for k in 0..512 {
                let a = *prev + k * range_factor;
                machine.reset_with_register(a);
                let quine = if digit == machine.program.len() {
                    // For final loop we require exact match, not a prefix
                    // that continues for longer.
                    machine.run_until_halt_or_non_quine()
                } else {
                    machine.run_until_halt_or_non_quine_or_outlen(digit)
                };
                if quine {
                    machine.print_output();
                    println!("Partial found : First {digit} matching characters found for A = {a}");
                    next_valid_a.push(a);
                }
            }
        }

        if next_valid_a.len() == 0 {
            panic!(
                "No candidate A found to match the first {digit} output ! Must expand search range"
            );
        }

        println!("Previous lowest A was {}", valid_a[0]);

        // Collect our different candidates for next digit.
        // Since we have overlaps (k covers more than just the new bits factor),
        // need to deduplicate first. Also it will sort for final result.
        next_valid_a.sort();
        next_valid_a.dedup();
        valid_a = next_valid_a;

        range_factor *= bits;
    }

    println!("All valid_A = {:?}", valid_a);
    valid_a.truncate(10);

    println!("Part 2: Valid A for complete quine ?");
    for a in &valid_a {
        machine.reset_with_register(*a);
        if machine.run_until_halt_or_non_quine() {
            machine.print_output();
            println!("Found register A = {} == oct {:o}", *a, *a);
        } else {
            machine.print_output();
            println!("register A = {} is invalid !! (bug)", *a);
        }
    }

    /* debug */
    let h = 42567035290;
    machine.reset_with_register(h);
    if machine.run_until_halt_or_non_quine() {
        machine.print_output();
        println!("Found hardcoded register A = {h}",);
    } else {
        println!("hardcoded {h} is not quine:",);
        machine.print_output();
    }

    println!("Part 2 : First valid A is {}", valid_a[0]);
}
