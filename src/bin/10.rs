use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn main() {
    let mut data: Vec<i64> = BufReader::new(File::open("input/10.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    data.push(0);
    data.sort();
    data.push(data.last().unwrap() + 3);

    // part 1
    let s1 = data.windows(2).filter(|&x| x[1] - x[0] == 1).count();
    let s3 = data.windows(2).filter(|&x| x[1] - x[0] == 3).count();
    println!("part 1: {:?}", s1 * s3);

    // part 2
    let mut arr = vec![0i64; data.len()];
    arr[0] = 1;
    for i in 1..data.len() {
        let mut j = i;
        while j > 0 && data[j - 1] >= data[i] - 3 {
            arr[i] += arr[j - 1];
            j -= 1;
        }
    }
    println!("part 2: {:?}", arr.last().unwrap());
}
