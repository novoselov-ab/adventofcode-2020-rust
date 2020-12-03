use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn calc_slope(map: &Vec<String>, r: usize, d: usize) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    while y < map.len() {
        if map[y].chars().nth(x).unwrap() == '#' {
            trees += 1;
        }
        x = (x + r) % map[y].len();
        y += d;
    }
    println!("trees on slope {}-{}: {}", r, d, trees);
    trees
}

fn main() {
    let file = File::open("input/3.txt").expect("Input file not found.");
    let reader = BufReader::new(file);

    let input: Vec<String> = match reader.lines().collect() {
        Err(err) => panic!("Unknown error reading input: {}", err),
        Ok(result) => result,
    };

    println!(
        "answer1: {}",
        calc_slope(&input, 1, 1)
            * calc_slope(&input, 3, 1)
            * calc_slope(&input, 5, 1)
            * calc_slope(&input, 7, 1)
            * calc_slope(&input, 1, 2)
    );
}
