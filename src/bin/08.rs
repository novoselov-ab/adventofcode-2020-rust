use std::io::prelude::*;
use std::str::FromStr;
use std::{fs::File, io::BufReader};

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    NOP,
    ACC,
    JMP,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Operation, Self::Err> {
        match s {
            "nop" => Ok(Operation::NOP),
            "acc" => Ok(Operation::ACC),
            "jmp" => Ok(Operation::JMP),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
struct Instruction {
    op: Operation,
    v: i32,
}

fn main() {
    let file = File::open("input/8.txt").expect("Input file not found.");
    let reader = BufReader::new(file);

    let mut code = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let (op, v) = line.split_at(line.find(" ").unwrap());

        code.push(Instruction {
            op: Operation::from_str(op).unwrap(),
            v: v.trim().parse::<i32>().unwrap(),
        })
    }

    let mut tried = vec![false; code.len()];

    // That answers part 2, don't want to refactor to answer both parts.
    for _ in 0..tried.len() {
        let mut pc = 0;
        let mut acc = 0;
        let mut mutated = false;
        let mut visited = vec![false; code.len()];

        while !visited[pc] {
            visited[pc] = true;
            let mut ins = code[pc].clone();

            if !mutated && ins.op != Operation::ACC && !tried[pc] {
                ins.op = if ins.op == Operation::JMP {
                    Operation::NOP
                } else {
                    Operation::NOP
                };
                tried[pc] = true;
                mutated = true;
            }

            pc += 1;

            match ins.op {
                Operation::NOP => {}
                Operation::ACC => acc += ins.v,
                Operation::JMP => pc = ((pc as i32 - 1) + ins.v) as usize,
            }

            if pc == code.len() {
                println!("{:?} ", acc);
                return;
            }
        }
    }
}
