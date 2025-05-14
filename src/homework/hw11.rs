use rand::Rng;

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    (0..n).map(|_| rand::thread_rng().gen_range(10..=99)).collect()
}

pub fn min_adjacent_sum(data: &[i32]) -> Option<(i32, usize, usize)> {
    data.windows(2)
        .enumerate()
        .map(|(i, w)| (w[0]+w[1], i, i+1))
        .min_by_key(|&(s, _, _)| s)
}

pub fn print_min_adjacent(data: &[i32]) {
    if let Some((sum,i,j)) = min_adjacent_sum(data) {
        let idxs = (0..data.len()).map(|k| format!("{:>2}.",k)).collect::<Vec<_>>().join(" ");
        let vals = data.iter().map(|v| format!("{:>2}",v)).collect::<Vec<_>>().join(", ");
        println!("indexes: {}\ndata:    [{}]", idxs, vals);
        let off = "indexes: ".len()+i*4;
        println!("{:>width$}{}", "", r"\__ __/", width=off);
        println!("min adjacent sum={}+{}={} at indexes:{},{}", data[i], data[j], sum, i, j);
    }
}

fn main(){ print_min_adjacent(&gen_random_vector(20)); }
