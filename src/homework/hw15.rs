use itertools::Itertools;

fn main() {
    let digits = (1..=8).collect::<Vec<_>>();
    let mut count = 0;

    for perm in digits.iter().permutations(8).unique() {
        let m = *perm[0];
        let u = *perm[1];
        let x = *perm[2];
        let a = *perm[3];
        let s = *perm[4];
        let l = *perm[5];
        let o = *perm[6];
        let n = *perm[7];

        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;

        if muxa * a == slon {
            count += 1;
            println!("  {}{}{}{}", m, u, x, a);
            println!("x     {}", a);
            println!("-------");
            println!("  {}{}{}{}", s, l, o, n);
            println!();
        }
    }

    println!("Кількість розв'язків: {}", count);
}
