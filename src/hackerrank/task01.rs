use std::io;

fn simple_array_sum(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let _n: usize = input.trim().parse().expect("Invalid number");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    println!("{}", simple_array_sum(&numbers));
}
