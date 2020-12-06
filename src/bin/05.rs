use std::{cmp::max, io::prelude::*};
use std::{fs::File, io::BufReader};

fn main() {
    let file = File::open("input/5.txt").expect("Input file not found.");
    let reader = BufReader::new(file);

    let mut max_id = 0;
    let mut taken = vec![false; 1024];
    for line in reader.lines() {
        let s = line
            .unwrap()
            .replace("F", "0")
            .replace("B", "1")
            .replace("R", "1")
            .replace("L", "0");
        let id = isize::from_str_radix(&s, 2).unwrap();
        taken[id as usize] = true;
        max_id = max(max_id, id);
    }
    println!("max id: {:?}", max_id);

    for x in 1..taken.len() - 1 {
        if !taken[x] && taken[x - 1] && taken[x + 1] {
            println!("my place: {:?}", x);
            break;
        }
    }
}
