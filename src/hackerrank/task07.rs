use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let nums: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let total_sum: u64 = nums.iter().sum();

    let min_val = nums.iter().min().unwrap();
    let max_val = nums.iter().max().unwrap();

    let min_sum = total_sum - max_val;
    let max_sum = total_sum - min_val;

    println!("{} {}", min_sum, max_sum);
}
