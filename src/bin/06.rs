use std::{io::prelude::*};
use std::{fs::File, io::BufReader};

fn main() {
    let file = File::open("input/6.txt").expect("Input file not found.");
    let reader = BufReader::new(file);

    let mut sum = (0u32, u32::MAX);
    let mut ans = (0, 0);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            ans = (ans.0 + sum.0.count_ones(), ans.1 + sum.1.count_ones());
            sum = (0, u32::MAX);
            continue;
        }

        let v = line
            .chars()
            .into_iter()
            .fold(0, |acc, x| acc | 1 << (x as u32 - 'a' as u32));
        sum = (sum.0 | v, sum.1 & v);
    }
    ans = (ans.0 + sum.0.count_ones(), ans.1 + sum.1.count_ones());

    println!("answers: {:?}", ans);
}
