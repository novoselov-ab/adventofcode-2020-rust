use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn find_two(data: &[i64], sum_to: i64) -> Option<(usize, usize)> {
    let mut l = 0;
    let mut r = data.len() - 1;
    while l < r {
        let sum = data[l] + data[r];
        if sum == sum_to {
            return Some((l, r));
        } else if sum < sum_to {
            l += 1;
        } else {
            r -= 1;
        }
    }

    None
}

fn find_three(data: &Vec<i64>, sum_to: i64) -> Option<(usize, usize, usize)> {
    for i in 0..data.len() - 2 {
        if data[i] > sum_to {
            return None;
        }

        let rest = sum_to - data[i];

        if let Some((x, y)) = find_two(&data[i + 1..], rest) {
            return Some((x + i + 1, y + i + 1, i));
        }
    }

    None
}

fn main() {
    let file = File::open("input/1.txt").expect("Input file not found.");
    let reader = BufReader::new(file);

    let input: Vec<String> = match reader.lines().collect() {
        Err(err) => panic!("Unknown error reading input: {}", err),
        Ok(result) => result,
    };

    let mut data: Vec<i64> = input.iter().map(|x| x.parse::<i64>().unwrap()).collect();
    data.sort();

    let (x, y) = find_two(&data, 2020).unwrap();

    println!("answer 1: {}", data[x] * data[y]);

    let (x, y, z) = find_three(&data, 2020).unwrap();

    println!("answer 2: {}", data[x] * data[y] * data[z]);
}
