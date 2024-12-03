use itertools::Itertools;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::fs::read_to_string;

enum Instruction {
    Do,
    Dont,
    Mul(usize, usize),
}

fn read_file() -> String {
    read_to_string("inputs/03.txt").expect("input file not present")
}

fn find_instructions<'a>(input: &'a str) -> impl Iterator<Item = Instruction> + 'a {
    lazy_static! { static ref INSTR_REGEX: Regex = Regex::new(r"(?:mul\((?<a>\d{1,3}),(?<b>\d{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))").unwrap(); }
    INSTR_REGEX.captures_iter(input).map(parse_captures)
}

fn parse_captures(c: Captures) -> Instruction {
    if c.name("do").is_some() { Instruction::Do }
    else if c.name("dont").is_some() { Instruction::Dont }
    else { 
        Instruction::Mul(
            c.name("a").unwrap().as_str().parse().expect("not a number"), 
            c.name("b").unwrap().as_str().parse().expect("not a number")
        ) 
    }
}

fn evaluate(instructions: &[Instruction], conditionals: bool) -> usize {
    instructions.iter().fold((0, true), |mut acc, instr| {
        match instr {
            Instruction::Do => { acc.1 = true; },
            Instruction::Dont => { if conditionals { acc.1 = false; }},
            Instruction::Mul(a, b) => { if acc.1 { acc.0 += a * b; }},
        };
        acc
    }).0
}

fn main() {
    let instructions = find_instructions(&read_file()).collect_vec();
    println!("Part 1: {}", evaluate(&instructions, false));
    println!("Part 2: {}", evaluate(&instructions, true));
}
