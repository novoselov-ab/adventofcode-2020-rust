use std::collections::VecDeque;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn main() {
    let mut players: [VecDeque<usize>; 2] = Default::default();
    let mut index = 0;
    for line in BufReader::new(File::open("input/22.txt").unwrap())
        .lines()
        .skip(1)
    {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("Player") {
            index += 1;
            continue;
        }
        players[index].push_back(line.parse::<usize>().unwrap());
    }

    println!("{:?}", players);

    let mut winner = 0;
    while !players[0].is_empty() && !players[1].is_empty() {
        let a = players[0].pop_front().unwrap();
        let b = players[1].pop_front().unwrap();
        winner = if a > b { 0 } else { 1 };
        players[winner].push_back(std::cmp::max(a, b));
        players[winner].push_back(std::cmp::min(a, b));
    }

    let ans1: usize = players[winner]
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| x * (i + 1))
        .sum();

    println!("{:?}", players);
    println!("{:?}", ans1);
}
