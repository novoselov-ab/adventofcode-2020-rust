use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn find_closing_brace(s: &str) -> usize {
    let mut opened = 0;
    for i in 0..s.len() {
        match s.chars().nth(i).unwrap() {
            ')' => opened -= 1,
            '(' => opened += 1,
            _ => {}
        }

        if opened == 0 {
            return i;
        }
    }
    panic!("wrong input");
}

fn consume(s: &str, pos: &mut usize, part1: bool) -> u64 {
    let c = s.chars().nth(0).unwrap();
    if c == '(' {
        let brace_pos = find_closing_brace(&s);
        *pos += brace_pos + 1;
        return calc(&s[1..brace_pos], part1);
    } else {
        *pos += 1;
        return c.to_digit(10).unwrap() as u64;
    }
}

fn calc(s: &str, part1: bool) -> u64 {
    let mut pos = 0;
    let mut a = consume(&s[pos..], &mut pos, part1);

    if s.len() == 1 {
        return a;
    }

    let mut stack = Vec::new();
    while pos + 1 < s.len() {
        let op = s.chars().nth(pos).unwrap();
        pos += 1;

        let b = consume(&s[pos..], &mut pos, part1);

        match op {
            '+' => a += b,
            '*' => {
                if part1 {
                    a *= b;
                } else {
                    stack.push(a);
                    a = b;
                }
            }
            _ => {}
        }
    }

    stack.iter().fold(a, |acc, v| acc * v)
}

fn main() {
    let lines: Vec<String> = BufReader::new(File::open("input/18.txt").unwrap())
        .lines()
        .map(|l| l.unwrap().replace(" ", ""))
        .collect();

    println!(
        "ans 1: {}",
        lines.iter().map(|l| calc(l, true)).sum::<u64>()
    );
    println!(
        "ans 2: {}",
        lines.iter().map(|l| calc(l, false)).sum::<u64>()
    );
}
