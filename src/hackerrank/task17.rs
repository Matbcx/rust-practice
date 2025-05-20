use std::collections::HashMap;
use std::io;

fn main() {
    let mut _n = String::new();
    io::stdin().read_line(&mut _n).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let arr: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut freq = HashMap::new();

    for &id in &arr {
        *freq.entry(id).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut bird_id = i32::MAX;

    for (&id, &count) in &freq {
        if count > max_count || (count == max_count && id < bird_id) {
            max_count = count;
            bird_id = id;
        }
    }

    println!("{}", bird_id);
}
