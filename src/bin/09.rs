use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn has_sum(data: &[i64], sum_to: i64) -> bool {
    let mut l = 0;
    let mut r = data.len() - 1;
    while l < r {
        let sum = data[l] + data[r];
        if sum == sum_to {
            return true;
        } else if sum < sum_to {
            l += 1;
        } else {
            r -= 1;
        }
    }

    false
}

fn crawl_find_sum(data: &[i64], sum_to: i64) -> (usize, usize) {
    let mut l = 0;
    let mut r = 0;
    let mut sum = 0;
    while l < data.len() {
        println!("{} - {} - {}", l, r, sum);
        if sum == sum_to {
            return (l, r);
        } else if sum < sum_to {
            sum += data[r];
            r += 1;
        } else if sum > sum_to {
            sum -= data[l];
            l += 1;
        }
    }
    (0, 0)
}

fn main() {
    let file = File::open("input/9.txt").expect("Input file not found.");
    let reader = BufReader::new(file);

    let data: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    let mut broken_number = 0;
    let mut window = Vec::new();
    const WINDOW_SIZE: usize = 25;
    for j in 0..data.len() {
        let x = data[j];

        if window.len() == WINDOW_SIZE {
            if !has_sum(&window, x) {
                broken_number = x;
                break;
            }
        }

        window.push(x);

        for i in (1..window.len()).rev() {
            if window[i] < window[i - 1] {
                window.swap(i, i - 1);
            } else {
                break;
            }
        }

        if window.len() > WINDOW_SIZE {
            window.remove(
                window
                    .iter()
                    .position(|&x| x == data[j - WINDOW_SIZE])
                    .unwrap(),
            );
        }
    }

    println!("answer 0: {}", broken_number);

    let r = crawl_find_sum(&data, broken_number);
    println!(
        "answer 1: {:?}",
        data[r.0..r.1].iter().min().unwrap() + data[r.0..r.1].iter().max().unwrap()
    );
}
