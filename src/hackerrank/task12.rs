use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let (x1, v1, x2, v2) = (nums[0], nums[1], nums[2], nums[3]);

    let result = if v1 == v2 {
        if x1 == x2 {
            "YES"
        } else {
            "NO"
        }
    } else {
        let dv = v1 - v2;
        let dx = x2 - x1;
        if dv != 0 && dx % dv == 0 && (dx / dv) >= 0 {
            "YES"
        } else {
            "NO"
        }
    };

    println!("{}", result);
}
