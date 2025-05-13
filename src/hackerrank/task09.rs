use std::io;

fn main() {
    let mut time_str = String::new();
    io::stdin().read_line(&mut time_str).unwrap();
    let time_str = time_str.trim();

    let hour = &time_str[..2];
    let rest = &time_str[2..8];
    let period = &time_str[8..];

    let mut hour_int: u32 = hour.parse().unwrap();

    if period == "AM" {
        if hour_int == 12 {
            hour_int = 0;
        }
    } else if period == "PM" {
        if hour_int != 12 {
            hour_int += 12;
        }
    }

    println!("{:02}{}", hour_int, rest);
}
