fn main() {
    let w: usize = 28;
    let h: usize = 10;

    for y in 0..h {
        for x in 0..w {
            let is_hor = y == 0 || y == h - 1;
            let is_ver = x == 0 || x == w - 1;

            let diag1_x = (y * (w - 1) + (h - 1) / 2) / (h - 1);
            let diag2_x = ((h - 1 - y) * (w - 1) + (h - 1) / 2) / (h - 1);

            let is_diag1 = x == diag1_x;
            let is_diag2 = x == diag2_x;

            let to_show = is_hor || is_ver || is_diag1 || is_diag2;
            let sym = if to_show { '*' } else { ' ' };
            print!("{}", sym);
        }
        println!();
    }
}
