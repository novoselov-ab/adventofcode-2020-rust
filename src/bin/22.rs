use std::collections::{HashSet, VecDeque};
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn play_1(players: &mut [VecDeque<usize>; 2]) -> usize {
    let mut winner = 0;
    while !players[0].is_empty() && !players[1].is_empty() {
        let a = players[0].pop_front().unwrap();
        let b = players[1].pop_front().unwrap();
        winner = if a > b { 0 } else { 1 };
        players[winner].push_back(std::cmp::max(a, b));
        players[winner].push_back(std::cmp::min(a, b));
    }

    winner
}

fn play_2(players: &mut [VecDeque<usize>; 2]) -> usize {
    let mut states: HashSet<[VecDeque<usize>; 2]> = HashSet::new();

    let mut winner = 0;
    while !players[0].is_empty() && !players[1].is_empty() {
        if states.contains(players) {
            return 0;
        }
        states.insert(players.clone());

        let a = players[0].pop_front().unwrap();
        let b = players[1].pop_front().unwrap();

        if a <= players[0].len() && b <= players[1].len() {
            let mut players_copy = players.clone();
            players_copy[0].drain(a..);
            players_copy[1].drain(b..);
            winner = play_2(&mut players_copy);
        } else {
            winner = if a > b { 0 } else { 1 };
        }
        players[winner].push_back(if winner == 0 { a } else { b });
        players[winner].push_back(if winner == 0 { b } else { a });
    }

    winner
}

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

    for part in 0..2 {
        let mut players = players.clone();

        let winner = if part == 0 {
            play_1(&mut players)
        } else {
            play_2(&mut players)
        };
        let ans: usize = players[winner]
            .iter()
            .rev()
            .enumerate()
            .map(|(i, x)| x * (i + 1))
            .sum();
        println!("{:?}", players);
        println!("part{}: {}", part, ans);
    }
}
