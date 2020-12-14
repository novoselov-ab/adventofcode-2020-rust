use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn main() {
    let data: Vec<String> = BufReader::new(File::open("input/13.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let t = data[0].parse::<i64>().unwrap();
    let values: Vec<&str> = data[1].split(",").collect();

    // part 1
    let mut bus_id = 0;
    let mut minutes = std::i64::MAX;
    for &v in &values {
        if v != "x" {
            let v = v.parse::<i64>().unwrap();
            let r = v - t % v;
            if r < minutes {
                minutes = r;
                bus_id = v as i64;
            }
        }
    }
    println!("{}", bus_id * minutes);

    // part 2
    let mut d = values
        .iter()
        .enumerate()
        .filter(|(_, &x)| x != "x")
        .map(|(i, &x)| (i as i64, x.parse::<i64>().unwrap()))
        .collect::<Vec<(i64, i64)>>();
    d.sort_by(|a, b| b.1.cmp(&a.1));

    let mut x = 0;
    let mut s = 1;
    let mut i = 0;
    while i < d.len() {
        if (x + d[i].0) % d[i].1 == 0 {
            s = s * d[i].1;
            i += 1;
            continue;
        }

        x += s;
    }
    println!("{}", x);
}
