fn iter_links(links: &Vec<usize>, start: usize, count: usize) -> Vec<usize> {
    let mut res = Vec::new();
    let mut p = start;
    for _ in 0..count {
        p = links[p];
        res.push(p);
    }

    res
}

fn craby_crab(links: &mut Vec<usize>, iter_count: usize, start: usize) {
    let max_num = links.len() - 1;
    let mut current = start;
    let mut pickup: [usize; 3] = Default::default();
    for _ in 0..iter_count {
        let mut p = current;
        for i in 0..3 {
            p = links[p];
            pickup[i] = p;
        }

        let mut dest = current;
        loop {
            dest -= 1;
            if dest == 0 {
                dest = max_num;
            }
            if !pickup.contains(&dest) {
                break;
            }
        }

        let prev = links[current];
        links[current] = links[p];
        let prev2 = links[dest];
        links[dest] = prev;
        links[p] = prev2;

        current = links[current];
    }
}

fn build_links(items: &Vec<usize>) -> Vec<usize> {
    let mut links = Vec::new();
    links.resize(items.len() + 1, 0);
    let mut p = items[0];
    for i in 1..items.len() {
        links[p] = items[i];
        p = links[p];
    }
    links[p] = items[0];

    links
}

fn main() {
    let mut items: Vec<usize> = "685974213"
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    let mut links = build_links(&items);
    craby_crab(&mut links, 100, items[0]);
    println!(
        "part 1: {}",
        iter_links(&links, 1, 8)
            .iter()
            .map(|x| x.to_string())
            .collect::<String>()
    );

    for i in 10..=1000000 {
        items.push(i);
    }
    let mut links = build_links(&items);
    craby_crab(&mut links, 10000000, items[0]);
    let r = iter_links(&links, 1, 2);
    println!("part 2: {}", r[0] * r[1]);
}
