fn draw_tree(triangles: usize) {
    let max_lines = triangles + 1;
    let max_width = 2 * (triangles + 2) + 1;

    (1..=triangles).for_each(|t| {
        (1..=t).for_each(|i| {
            let stars = 2 * i - 1;
            let padding = (max_width - stars) / 2;
            println!("{}{}", " ".repeat(padding), "*".repeat(stars));
        });
    });

    (1..=triangles + 1).for_each(|i| {
        let stars = 2 * i - 1;
        let padding = (max_width - stars) / 2;
        println!("{}{}", " ".repeat(padding), "*".repeat(stars));
    });
}

fn main() {
    let triangles = 5;
    draw_tree(triangles);
}
