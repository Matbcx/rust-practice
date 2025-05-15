use std::io;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

fn gcd_list(nums: &[i32]) -> i32 {
    nums.iter().cloned().reduce(gcd).unwrap()
}

fn lcm_list(nums: &[i32]) -> i32 {
    nums.iter().cloned().reduce(lcm).unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let b: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    let lcm_a = lcm_list(&a);
    let gcd_b = gcd_list(&b);

    let mut count = 0;
    let mut multiple = lcm_a;
    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    println!("{}", count);
}
