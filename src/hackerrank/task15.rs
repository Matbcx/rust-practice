use std::io;

fn main() {
    let mut _n = String::new();
    io::stdin().read_line(&mut _n).unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let s: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let dm: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (d, m) = (dm[0], dm[1]);

    let mut count = 0;

    for i in 0..=s.len().saturating_sub(m as usize) {
        let sum: i32 = s[i..i + m as usize].iter().sum();
        if sum == d {
            count += 1;
        }
    }

    println!("{}", count);
}
