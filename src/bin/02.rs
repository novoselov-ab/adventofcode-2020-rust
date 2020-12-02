use std::io::prelude::*;
use std::str::FromStr;
use std::{fs::File, io::BufReader};

#[derive(Debug)]
struct PasswordInfo {
    x: usize,
    y: usize,
    c: char,
    str: String,
}

impl FromStr for PasswordInfo {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let xy: Vec<&str> = parts[0].split("-").collect();
        Ok(PasswordInfo {
            x: xy[0].parse::<usize>().unwrap(),
            y: xy[1].parse::<usize>().unwrap(),
            c: parts[1].chars().nth(0).unwrap(),
            str: parts[2].to_string(),
        })
    }
}

fn part1(data: &Vec<PasswordInfo>) -> i32 {
    let mut res = 0;
    for d in data {
        let r = d.str.chars().filter(|&n| n == d.c).count();
        if r >= d.x && r <= d.y {
            res += 1;
        }
    }
    res
}

fn part2(data: &Vec<PasswordInfo>) -> i32 {
    let mut res = 0;
    for d in data {
        let c1 = d.str.chars().nth(d.x - 1).unwrap();
        let c2 = d.str.chars().nth(d.y - 1).unwrap();
        if (d.c == c1 || d.c == c2) && (c1 != c2) {
            res += 1;
        }
    }
    res
}

fn main() {
    let file = File::open("input/2.txt").expect("Input file not found.");
    let reader = BufReader::new(file);

    let input: Vec<String> = match reader.lines().collect() {
        Err(err) => panic!("Unknown error reading input: {}", err),
        Ok(result) => result,
    };

    let data: Vec<_> = input
        .iter()
        .map(|x| x.parse::<PasswordInfo>().unwrap())
        .collect();

    println!("answer1: {}", part1(&data));
    println!("answer2: {}", part2(&data));
}
