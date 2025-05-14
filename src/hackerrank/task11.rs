use std::io;

fn read_line_to_vec() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let st = read_line_to_vec();
    let s = st[0];
    let t = st[1];

    let ab = read_line_to_vec();
    let a = ab[0];
    let b = ab[1];

    let _mn = read_line_to_vec();

    let apples = read_line_to_vec();
    let oranges = read_line_to_vec();

    let apple_count = apples.iter()
        .map(|&d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count();

    let orange_count = oranges.iter()
        .map(|&d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count();

    println!("{}", apple_count);
    println!("{}", orange_count);
}
