use std::collections::VecDeque;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

#[derive(Debug, Clone)]
enum Node {
    Link(Vec<Vec<usize>>),
    Leaf(char),
}

fn search(nodes: &Vec<Node>, rules: &mut VecDeque<usize>, s: &str) -> bool {
    if s.is_empty() || rules.is_empty() {
        return s.is_empty() && rules.is_empty();
    }

    let id = rules.pop_back().unwrap();
    match &nodes[id] {
        Node::Leaf(c) => {
            if *c == s.chars().nth(0).unwrap() && search(nodes, rules, &s[1..]) {
                return true;
            }
        }
        Node::Link(links) => {
            for group in links {
                rules.extend(group.iter().rev());
                if search(nodes, rules, &s) {
                    return true;
                }
                rules.drain(rules.len() - group.len()..);
            }
        }
    };
    rules.push_back(id);

    return false;
}

fn main() {
    let mut nodes = Vec::new();

    let mut iter = BufReader::new(File::open("input/19.txt").unwrap()).lines();
    for line in &mut iter {
        let line = line.unwrap();

        if line == "" {
            break;
        }

        let (id, rest) = line.split_at(line.find(":").unwrap());
        let id = id.parse::<usize>().unwrap();
        let rest = rest[1..].trim();

        if id >= nodes.len() {
            nodes.resize(id + 1, Node::Leaf(char::default()));
        }

        if rest.starts_with("\"") {
            nodes[id] = Node::Leaf(rest.trim_matches('\"').chars().nth(0).unwrap());
        } else {
            nodes[id] = Node::Link(
                rest.split("|")
                    .map(|x| {
                        x.trim()
                            .split(" ")
                            .map(|i| i.parse::<usize>().unwrap())
                            .collect()
                    })
                    .collect(),
            );
        }
    }

    let lines: Vec<String> = iter.map(|l| l.unwrap()).collect();

    println!(
        "part 1: {}",
        lines
            .iter()
            .filter(|l| search(&nodes, &mut VecDeque::from(vec![0]), &l))
            .count()
    );

    nodes[8] = Node::Link(vec![vec![42], vec![42, 8]]);
    nodes[11] = Node::Link(vec![vec![42, 31], vec![42, 11, 31]]);
    println!(
        "part 2: {}",
        lines
            .iter()
            .filter(|l| search(&nodes, &mut VecDeque::from(vec![0]), &l))
            .count()
    );
}
