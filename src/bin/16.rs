use std::{collections::HashMap, collections::HashSet, io::prelude::*};
use std::{fs::File, io::BufReader};

fn main() {
    let content = BufReader::new(File::open("input/16.txt").unwrap());

    let mut iter = content.lines();

    let mut rules: HashMap<i32, HashSet<String>> = HashMap::new();
    let mut fields: HashMap<String, i32> = HashMap::new();
    let mut field_num = 0;
    for line in &mut iter {
        let line = line.unwrap();
        if line == "" {
            break;
        }
        let (name, conds) = line.split_at(line.find(":").unwrap());
        let name = name.to_string();
        for cond in conds.strip_prefix(":").unwrap().trim().split("or") {
            let cond = cond.trim();
            let (from, to) = cond.split_at(cond.find("-").unwrap());
            let from = from.parse::<i32>().unwrap();
            let to = to[1..].parse::<i32>().unwrap();
            for i in from..=to {
                rules.entry(i).or_default().insert(name.clone());
            }
        }
        fields.insert(name, field_num);
        field_num += 1;
    }

    iter.next();
    let my_ticket: Vec<i32> = iter
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    iter.next();
    iter.next();
    let mut tickets: Vec<Vec<i32>> = Vec::new();
    for line in &mut iter {
        tickets.push(
            line.unwrap()
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
        );
    }

    let part1: i32 = tickets
        .iter()
        .map(|t| t.iter().filter(|x| rules.get(&x).is_none()).sum::<i32>())
        .sum();
    println!("part1: {}", part1);

    let tickets: Vec<Vec<i32>> = tickets
        .into_iter()
        .filter(|t| t.iter().all(|x| !rules.get(&x).is_none()))
        .collect();

    let mut variants: Vec<(i32, HashSet<String>)> = Vec::new();
    for i in 0..my_ticket.len() {
        let mut s: HashSet<String> = fields.keys().cloned().collect();
        for ticket in &tickets {
            s = s
                .intersection(rules.get(&ticket[i]).unwrap())
                .cloned()
                .collect();
        }
        variants.push((i as i32, s));
    }

    variants.sort_by_key(|t| t.1.len());
    solve(&mut variants, 0);

    let ans: i64 = variants
        .iter()
        .filter(|(_, v)| v.iter().next().unwrap().starts_with("departure"))
        .map(|(i, _)| my_ticket[*i as usize] as i64)
        .product();
    println!("part2: {:?}", ans);
}

fn solve(mut variants: &mut Vec<(i32, HashSet<String>)>, start: usize) -> bool {
    for i in start..variants.len() {
        if variants[i].1.len() == 0 {
            return false;
        }
        for j in 0..variants[i].1.len() {
            let candidate = variants[i].1.iter().nth(j).unwrap().clone();
            let mut removed = Vec::new();
            for k in i + 1..variants.len() {
                if variants[k].1.remove(&candidate) {
                    removed.push(k);
                }
            }

            if solve(&mut variants, i + 1) {
                return true;
            }

            for k in removed {
                variants[k].1.insert(candidate.clone());
            }
        }
    }

    return true;
}
