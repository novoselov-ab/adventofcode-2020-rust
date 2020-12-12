use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn find_sharp(
    arr: &Vec<Vec<char>>,
    pos: (i32, i32),
    dir: (i32, i32),
    size: (i32, i32),
    max_distance: i32,
) -> i32 {
    let mut pos = pos;
    let mut max_distance = max_distance;
    loop {
        max_distance -= 1;
        pos = (pos.0 + dir.0, pos.1 + dir.1);
        if pos.0 < 0 || pos.0 >= size.0 || pos.1 < 0 || pos.1 >= size.1 {
            return 0;
        }

        let pos = (pos.0 as usize, pos.1 as usize);
        if arr[pos.0][pos.1] == '#' {
            return 1;
        } else if arr[pos.0][pos.1] == 'L' {
            return 0;
        }

        if max_distance <= 0 {
            return 0;
        }
    }
}

fn simulate(arr: &Vec<Vec<char>>, limit: bool) -> i32 {
    let mut arrs = (arr.clone(), arr.clone());

    let (h, w) = (arrs.0[0].len(), arrs.0.len());
    let mut changed = true;
    let mut total = 0;
    let max_distance = if limit { 1 } else { (w + h) as i32 };
    let rule_magic = if limit { 4 } else { 5 };
    while changed {
        changed = false;
        total = 0;
        for x in 0..w {
            for y in 0..h {
                let mut occ = 0;
                if arrs.0[x][y] != '.' {
                    for a in -1i32..=1i32 {
                        for b in -1i32..=1i32 {
                            if (a, b) == (0, 0) {
                                continue;
                            }
                            occ += find_sharp(
                                &arrs.0,
                                (x as i32, y as i32),
                                (a, b),
                                (w as i32, h as i32),
                                max_distance,
                            )
                        }
                    }
                }

                if arrs.0[x][y] == 'L' && occ == 0 {
                    arrs.1[x][y] = '#';
                    changed = true;
                } else if arrs.0[x][y] == '#' && occ >= rule_magic {
                    arrs.1[x][y] = 'L';
                    changed = true;
                } else {
                    arrs.1[x][y] = arrs.0[x][y];
                }

                if arrs.1[x][y] == '#' {
                    total += 1;
                }
            }
        }

        std::mem::swap(&mut arrs.0, &mut arrs.1);
    }

    total
}

fn main() {
    let arr: Vec<Vec<char>> = BufReader::new(File::open("input/11.txt").unwrap())
        .lines()
        .map(|l| (l.unwrap()).chars().collect())
        .collect();

    println!("{:?}", simulate(&arr, true));
    println!("{:?}", simulate(&arr, false));
}
