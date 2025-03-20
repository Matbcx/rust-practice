use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let a: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let b: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = a.iter().zip(&b).fold((0, 0), |(alice, bob), (&x, &y)| {
        (alice + (x > y) as i32, bob + (x < y) as i32)
    });

    println!("{} {}", result.0, result.1);
}
