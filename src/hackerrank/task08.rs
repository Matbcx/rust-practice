use std::io;

fn main() {

    let mut _n = String::new();
    io::stdin().read_line(&mut _n).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let candles: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    if let Some(&max_height) = candles.iter().max() {
        let count = candles.iter().filter(|&&x| x == max_height).count();
        println!("{}", count);
    }
}
