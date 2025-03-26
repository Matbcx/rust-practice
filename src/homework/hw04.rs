fn main() {
    const WIDTH: usize = 11;
    const HEIGHT: usize = 11;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let center = WIDTH / 2;
            let abs_x = (x as isize - center as isize).abs();
            let abs_y = (y as isize - center as isize).abs();

            let to_show = abs_x + abs_y <= center as isize; 

            print!("{}", if to_show { '*' } else { ' ' });
        }
        println!();
    }
}
