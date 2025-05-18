use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let nk: Vec<usize> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (n, k) = (nk[0], nk[1]);

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let ar: Vec<i32> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut count = 0;

    for i in 0..n {
        for j in i+1..n {
            if (ar[i] + ar[j]) % k as i32 == 0 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
