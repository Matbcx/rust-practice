use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    lines.next();

    if let Some(Ok(line)) = lines.next() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let total = numbers.len() as f64;

        let (mut pos, mut neg, mut zero) = (0.0, 0.0, 0.0);

        for num in numbers {
            if num > 0 {
                pos += 1.0;
            } else if num < 0 {
                neg += 1.0;
            } else {
                zero += 1.0;
            }
        }

        println!("{:.6}", pos / total);
        println!("{:.6}", neg / total);
        println!("{:.6}", zero / total);
    }
}
