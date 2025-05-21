use std::collections::HashMap;
use std::io;

fn main() {
    let mut _n = String::new();
    io::stdin().read_line(&mut _n).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let socks: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut color_count = HashMap::new();

    for &sock in &socks {
        *color_count.entry(sock).or_insert(0) += 1;
    }

    let mut pairs = 0;
    for &count in color_count.values() {
        pairs += count / 2;
    }

    println!("{}", pairs);
}
