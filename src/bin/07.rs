use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

#[derive(Default, Debug)]
pub struct Node {
    pub links: Vec<usize>,
    pub reverse_links: Vec<(usize, usize)>,
    pub visited: bool,
}

#[derive(Default, Debug)]
pub struct Tree {
    pub nodes: Vec<Node>,
    pub node_by_name: HashMap<String, usize>,
}

impl Tree {
    fn add_link(&mut self, from: &str, to: &str, data: usize) {
        let from_index = self.get_node_index(from);
        let to_index = self.get_node_index(to);
        let from_node = &mut self.nodes[from_index];
        if !from_node.links.contains(&to_index) {
            from_node.links.push(to_index);
        }
        let to_node = &mut self.nodes[to_index];
        to_node.reverse_links.push((from_index, data));
    }

    fn get_node_index(&mut self, name: &str) -> usize {
        let i = self
            .node_by_name
            .entry(name.to_string())
            .or_insert(self.nodes.len());
        if *i == self.nodes.len() {
            self.nodes.push(Node::default());
        }
        return *i;
    }

    fn calc_reachable(&mut self, index: usize) -> usize {
        let node = &mut self.nodes[index];
        if node.visited {
            return 0;
        }
        node.visited = true;

        let mut num = 0;
        for link in self.nodes[index].links.clone().iter() {
            num += self.calc_reachable(*link);
        }

        return num + 1;
    }

    fn calc_reverse_data_sum(&mut self, index: usize) -> usize {
        let node = &mut self.nodes[index];
        let mut num = 0;
        for i in 0..node.reverse_links.len() {
            let link = self.nodes[index].reverse_links[i];
            num += link.1 * self.calc_reverse_data_sum(link.0);
            num += link.1;
        }

        return num;
    }

    fn reset_visited(&mut self) {
        for n in &mut self.nodes {
            n.visited = false;
        }
    }
}

fn main() {
    let file = File::open("input/7.txt").expect("Input file not found.");
    let reader = BufReader::new(file);

    let mut tree = Tree::default();

    for line in reader.lines() {
        let line: String = line
            .unwrap()
            .replace("bags", "")
            .replace("bag", "")
            .replace(".", "");

        let p = line.find("contain").unwrap();
        let (first, second) = line.split_at(p);
        for part in second.trim().strip_prefix("contain").unwrap().split(",") {
            let part = part.trim();
            let (num, name) = part.split_at(part.find(" ").unwrap());
            let num = num.trim();
            if num == "no" {
                continue;
            }

            tree.add_link(name.trim(), first.trim(), num.parse::<usize>().unwrap());
        }
    }

    let gold_index = tree.get_node_index("shiny gold");
    tree.reset_visited();
    println!("answer0: {:?}", tree.calc_reachable(gold_index) - 1);
    println!("answer1: {:?}", tree.calc_reverse_data_sum(gold_index));
}
