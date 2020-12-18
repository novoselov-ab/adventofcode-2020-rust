use std::{
    collections::{HashMap, HashSet},
    io::prelude::*,
};
use std::{fs::File, io::BufReader};

type Vec4 = (i32, i32, i32, i32);

fn simulate(world: &HashSet<Vec4>, dirs: &Vec<Vec4>) {
    let mut current = world.clone();

    for _ in 0..6 {
        let mut next = HashSet::new();
        let mut countmap = HashMap::new();

        for p in current.iter() {
            for d in dirs.iter() {
                *countmap
                    .entry((p.0 + d.0, p.1 + d.1, p.2 + d.2, p.3 + d.3))
                    .or_insert(0) += 1;
            }
        }

        for (p, &cnt) in countmap.iter() {
            if current.contains(p) && cnt == 2 || cnt == 3 {
                next.insert(*p);
            }
        }

        current = next;
    }

    println!("{:?}", current.iter().count());
}

fn main() {
    let content = BufReader::new(File::open("input/17.txt").unwrap());

    let mut dirs3 = Vec::new();
    let mut dirs4 = Vec::new();
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if x != 0 || y != 0 || z != 0 {
                    dirs3.push((x, y, z, 0));
                }
                for w in -1..=1 {
                    if x != 0 || y != 0 || z != 0 || w != 0 {
                        dirs4.push((x, y, z, w));
                    }
                }
            }
        }
    }

    let mut world = HashSet::new();
    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                world.insert((x as i32, y as i32, 0, 0));
            }
        }
    }

    simulate(&world, &dirs3);
    simulate(&world, &dirs4);
}
