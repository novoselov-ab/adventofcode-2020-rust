use std::{cmp::max, collections::HashMap};

fn solve(arr: &[usize], num: usize) {
    let mut mem: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut x = 0;
    for i in 0..num {
        if i < arr.len() {
            x = arr[i];
        } else {
            let t = mem.entry(x).or_default();
            x = if t.1 == 0 || t.0 == 0 { 0 } else { t.0 - t.1 }
        }

        let t = mem.entry(x).or_insert((0, 0));
        *t = (i + 1, max(t.1, t.0));
    }
    println!("{}th: {}", num, x);
}

fn main() {
    solve(&[19, 20, 14, 0, 9, 1], 2020);
    solve(&[19, 20, 14, 0, 9, 1], 30000000);
}
