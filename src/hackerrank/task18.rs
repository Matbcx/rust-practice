use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let nk: Vec<usize> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let k = nk[1];

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let bill: Vec<i32> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let b_charged: i32 = line.trim().parse().unwrap();

    let total: i32 = bill.iter().sum();
    let anna_share = (total - bill[k]) / 2;

    if b_charged == anna_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b_charged - anna_share);
    }
}
