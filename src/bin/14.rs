use std::{collections::HashMap, io::prelude::*};
use std::{fs::File, io::BufReader};

fn main() {
    let mut base_mask: i64 = 0;
    let mut x_mask: i64 = 0;
    let mut mem0: HashMap<i64, i64> = HashMap::new();
    let mut mem1: HashMap<i64, i64> = HashMap::new();
    for line in BufReader::new(File::open("input/14.txt").unwrap()).lines() {
        let line = line.unwrap();
        let (op, v) = line.split_at(line.find("=").unwrap());
        let (op, v) = (op.trim(), v[1..].trim());

        if op == "mask" {
            base_mask = i64::from_str_radix(&v.replace("X", "0"), 2).unwrap();
            x_mask = i64::from_str_radix(&v.replace("1", "0").replace("X", "1"), 2).unwrap();
        } else {
            let addr = op[4..op.len() - 1].parse::<i64>().unwrap();
            let value = v.parse::<i64>().unwrap();

            // part 1
            mem0.insert(addr, base_mask | (x_mask & value));

            // part 2
            let bits = x_mask.count_ones();
            for i in 0..(1 << bits) {
                let mut next = 0;
                let mut final_addr = addr | base_mask;
                for k in 0..bits {
                    while x_mask & (1 << next) == 0 {
                        next += 1;
                    }
                    if i & (1 << k) != 0 {
                        final_addr |= 1 << next;
                    } else {
                        final_addr &= !(1 << next);
                    }
                    next += 1;
                }
                mem1.insert(final_addr, value);
            }
        }
    }

    println!("{:?}", mem0.iter().map(|(_, v)| v).sum::<i64>());
    println!("{:?}", mem1.iter().map(|(_, v)| v).sum::<i64>());
}
