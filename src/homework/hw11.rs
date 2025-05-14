use rand::Rng;

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

pub fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    assert!(data.len() >= 2, "Потрібно принаймні два елементи");
  
    let mut min_sum = data[0] + data[1];
    let mut min_i = 0;

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_i = i;
        }
    }

    (min_sum, min_i, min_i + 1)
}

pub fn print_min_adjacent(data: &[i32]) {
    let n = data.len();
    let (min_sum, i1, i2) = min_adjacent_sum(data);

    print!("indexes:");
    for i in 0..n {
        print!(" {:>2}.", i);
    }
    println!();

    print!("data:   [");
    for (j, &v) in data.iter().enumerate() {
        if j > 0 { print!(", "); }
        print!("{:>2}", v);
    }
    println!("]");

    let label_width = "indexes:".len();
    let cell_width = 4; // " 0. " або "10. "
    let indent = label_width + i1 * cell_width + 1;
    print!("{:indent$}", "", indent = indent);
    println!("\\__ __/");

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[i1], data[i2], min_sum, i1, i2
    );
}

fn main() {
  
    let data = gen_random_vector(20);
    print_min_adjacent(&data);
}
