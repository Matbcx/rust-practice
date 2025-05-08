use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        matrix.push(row);
    }

    let mut primary_diagonal = 0;
    let mut secondary_diagonal = 0;

    for i in 0..n {
        primary_diagonal += matrix[i][i];
        secondary_diagonal += matrix[i][n - 1 - i];
    }

    let result = (primary_diagonal - secondary_diagonal).abs();
    println!("{}", result);
}
