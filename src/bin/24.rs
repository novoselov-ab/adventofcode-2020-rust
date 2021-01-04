use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

#[derive(Default, Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Vec2 { x, y }
    }
}

const ALL_DIRS: [Vec2; 6] = [
    Vec2 { x: 0, y: -1 },
    Vec2 { x: -1, y: -1 },
    Vec2 { x: 0, y: 1 },
    Vec2 { x: -1, y: 1 },
    Vec2 { x: 1, y: 0 },
    Vec2 { x: -1, y: 0 },
];

fn count_near(start: &HashMap<Vec2, bool>, pos: &Vec2) -> usize {
    let mut cnt = 0;
    for d in ALL_DIRS.iter() {
        if *start.get(&calc_pos(pos, d)).unwrap_or(&false) {
            cnt += 1;
        }
    }
    cnt
}

fn calc_pos(pos: &Vec2, d: &Vec2) -> Vec2 {
    let shift = if d.y == 0 { 0 } else { (pos.y % 2 + 1) % 2 };
    Vec2::new(pos.x + d.x + shift, pos.y + d.y)
}

fn simulate(world: HashMap<Vec2, bool>) -> HashMap<Vec2, bool> {
    let mut new_world = HashMap::new();

    for (pos, &value) in world.iter() {
        if value {
            let near = count_near(&world, pos);
            if near == 1 || near == 2 {
                new_world.insert(pos.clone(), true);
            }

            for d in ALL_DIRS.iter() {
                let p = calc_pos(pos, d);
                if !*world.get(&p).unwrap_or(&false) {
                    if count_near(&world, &p) == 2 {
                        new_world.insert(p, true);
                    }
                }
            }
        }
    }

    new_world
}

fn main() {
    let mut input = Vec::new();
    for line in BufReader::new(File::open("input/24.txt").unwrap()).lines() {
        let line = line.unwrap();
        let mut dirs = Vec::new();
        let mut iter = line.chars();
        loop {
            match iter.next() {
                Some(c) => {
                    let d = match c {
                        's' => match iter.next().unwrap() {
                            'e' => Vec2::new(0, -1),
                            'w' => Vec2::new(-1, -1),
                            _ => panic!("unexpected input"),
                        },
                        'n' => match iter.next().unwrap() {
                            'e' => Vec2::new(0, 1),
                            'w' => Vec2::new(-1, 1),
                            _ => panic!("unexpected input"),
                        },
                        'e' => Vec2::new(1, 0),
                        'w' => Vec2::new(-1, 0),
                        _ => panic!("unexpected input"),
                    };
                    dirs.push(d);
                }
                None => break,
            }
        }
        input.push(dirs);
    }

    // 0 1 2 3 4 5
    //  0 1 2 3 4 5
    // 0 1 2 3 4 5
    //  0 1 2 3 4 5

    // ^
    // |
    // Y, N
    // |
    // ---X, E--->

    let mut world: HashMap<Vec2, bool> = HashMap::new();

    for dirs in &input {
        let mut pos = Vec2 { x: 0, y: 0 };
        for d in dirs {
            pos = calc_pos(&pos, d);
        }
        let e = world.entry(pos).or_insert(false);
        *e = !*e;
    }

    println!("part 1 : {:?}", world.values().filter(|&x| *x).count());

    for _ in 0..100 {
        world = simulate(world);
    }
    println!("part 2 : {:?}", world.values().filter(|&x| *x).count());
}
