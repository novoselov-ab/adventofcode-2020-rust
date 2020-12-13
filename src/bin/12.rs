use std::io::prelude::*;
use std::{fs::File, io::BufReader};

#[derive(Default, Debug)]

struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    fn add(&self, v: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }

    fn mul(&self, v: i32) -> Vec2 {
        Vec2 {
            x: self.x * v,
            y: self.y * v,
        }
    }

    fn rotate(&mut self, v: i32) -> Vec2 {
        let (sn, cs) = (v as f32).to_radians().sin_cos();
        let (sn, cs) = (sn as i32, cs as i32);

        Vec2 {
            x: cs * self.x - sn * self.y,
            y: sn * self.x + cs * self.y,
        }
    }
}

fn part1(data: &Vec<(char, i32)>) {
    let mut pos = Vec2 { x: 0, y: 0 };
    let mut dir = Vec2 { x: 1, y: 0 };
    for &(c, v) in data {
        match c {
            'N' => pos.y += v,
            'S' => pos.y -= v,
            'E' => pos.x += v,
            'W' => pos.x -= v,
            'L' => dir = dir.rotate(v),
            'R' => dir = dir.rotate(-v),
            'F' => pos = pos.add(&dir.mul(v)),
            _ => (),
        };
    }
    println!("{:?}", pos.x.abs() + pos.y.abs());
}

fn part2(data: &Vec<(char, i32)>) {
    let mut wp = Vec2 { x: 10, y: 1 };
    let mut pos = Vec2 { x: 0, y: 0 };
    for &(c, v) in data {
        match c {
            'N' => wp.y += v,
            'S' => wp.y -= v,
            'E' => wp.x += v,
            'W' => wp.x -= v,
            'L' => wp = wp.rotate(v),
            'R' => wp = wp.rotate(-v),
            'F' => pos = pos.add(&wp.mul(v)),
            _ => (),
        };
    }
    println!("{:?}", pos.x.abs() + pos.y.abs());
}

fn main() {
    // y, N
    // ^
    // |
    // + -- > x, E

    let data: Vec<(char, i32)> = BufReader::new(File::open("input/12.txt").unwrap())
        .lines()
        .map(|x| x.unwrap())
        .map(|x| (x.chars().nth(0).unwrap(), x[1..].parse::<i32>().unwrap()))
        .collect();

    part1(&data);
    part2(&data);
}
