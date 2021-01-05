
fn main() {
    let (a , b) = (11239946, 10464955);

    let mut r = 1u64;
    let mut loops = 0;
    while r != a {
        r = (r * 7) % 20201227;
        loops += 1;
    }

    let mut q = 1u64;
    for _ in 0..loops {
        q = (q * b) % 20201227;
    }

    println!("ans: {}", q);
}
