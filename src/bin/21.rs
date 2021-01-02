use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

#[derive(Debug)]
struct Entry {
    items: Vec<String>,
    props: Vec<String>,
}

#[derive(Debug, Default)]
struct Solution {
    item_to_prop: HashMap<String, String>,
    prop_to_item: HashMap<String, String>,
}

fn solve(
    entries: &Vec<Entry>,
    mut index: usize,
    mut prop_index: usize,
    solution: &mut Solution,
) -> bool {
    if prop_index >= entries[index].props.len() {
        index += 1;
        prop_index = 0;
        if index >= entries.len() {
            return true;
        }
    }

    let entry = &entries[index];
    let prop = &entry.props[prop_index];

    if solution.prop_to_item.contains_key(prop) {
        return solve(entries, index, prop_index + 1, solution);
    } else {
        for item in &entry.items {
            if solution.item_to_prop.contains_key(item) {
                continue;
            }

            if entries
                .iter()
                .skip(index + 1)
                .any(|e| !e.items.contains(item) && e.props.contains(prop))
            {
                continue;
            }

            solution.prop_to_item.insert(prop.clone(), item.clone());
            solution.item_to_prop.insert(item.clone(), prop.clone());

            if solve(entries, index, prop_index + 1, solution) {
                return true;
            }

            solution.prop_to_item.remove(prop);
            solution.item_to_prop.remove(item);
        }
    }

    false
}

fn main() {
    let mut entries = Vec::new();

    for line in BufReader::new(File::open("input/21.txt").unwrap()).lines() {
        let line = line.unwrap();
        let (a, b) = line.split_at(line.find("(contains").unwrap());
        let items: Vec<String> = a.trim().split(" ").map(|s| s.to_string()).collect();
        let props: Vec<String> = b
            .strip_prefix("(contains ")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split(",")
            .map(|s| s.trim().to_string())
            .collect();

        entries.push(Entry {
            items: items,
            props: props,
        });
    }

    let mut solution = Solution::default();
    solve(&entries, 0, 0, &mut solution);

    let ans1: usize = entries
        .iter()
        .map(|e| {
            e.items
                .iter()
                .filter(|i| !solution.item_to_prop.contains_key(*i))
                .count()
        })
        .sum();
    println!("part 1: {} ", ans1);

    let mut pairs: Vec<(String, String)> = solution.item_to_prop.into_iter().collect();
    pairs.sort_by(|a, b| a.1.cmp(&b.1));
    let ingredients: Vec<String> = pairs.into_iter().map(|p| p.0).collect();

    println!("part 2: {:?} ", ingredients.join(","));
}
