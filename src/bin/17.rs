use std::{collections::HashMap, io::prelude::*};
use std::{fs::File, io::BufReader};

type Vec4 = (i32, i32, i32, i32);

fn simulate(world: &HashMap<Vec4, bool>, dirs: &Vec<Vec4>) {
    let wake_neighbors = |pos: &Vec4, w: &mut HashMap<Vec4, bool>| {
        for d in dirs.iter() {
            let vn = (pos.0 + d.0, pos.1 + d.1, pos.2 + d.2, pos.3 + d.3);
            w.entry(vn).or_insert(false);
        }
    };

    let mut new_worlds = (world.clone(), HashMap::new());
    let mut worlds = (&mut new_worlds.0, &mut new_worlds.1);

    for (pos, &active) in worlds.0.iter() {
        worlds.1.insert(*pos, active);
        wake_neighbors(pos, worlds.1);
    }
    std::mem::swap(&mut worlds.0, &mut worlds.1);
    worlds.1.clear();

    for _ in 0..6 {
        for (pos, &active) in worlds.0.iter() {
            let mut near = 0;
            for d in dirs.iter() {
                let vn = (pos.0 + d.0, pos.1 + d.1, pos.2 + d.2, pos.3 + d.3);
                if *worlds.0.get(&vn).unwrap_or(&false) {
                    near += 1;
                }
            }

            let activate = if active {
                near == 2 || near == 3
            } else {
                near == 3
            };

            worlds.1.insert(*pos, activate);

            if activate {
                wake_neighbors(pos, worlds.1);
            }
        }

        std::mem::swap(&mut worlds.0, &mut worlds.1);
        worlds.1.clear();
    }

    println!("{:?}", worlds.0.values().filter(|&x| *x).count());
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

    let mut world: HashMap<Vec4, bool> = HashMap::new();
    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            world.insert((x as i32, y as i32, 0, 0), c == '#');
        }
    }

    simulate(&world, &dirs3);
    simulate(&world, &dirs4);
}
