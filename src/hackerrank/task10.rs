use std::io;

fn main() {
    let mut n_input = String::new();
    io::stdin().read_line(&mut n_input).unwrap();
    let n: usize = n_input.trim().parse().unwrap();

    for _ in 0..n {
        let mut grade_input = String::new();
        io::stdin().read_line(&mut grade_input).unwrap();
        let grade: i32 = grade_input.trim().parse().unwrap();

        let final_grade = if grade < 38 {
            grade
        } else {
            let next_multiple_of_5 = ((grade + 4) / 5) * 5;
            if next_multiple_of_5 - grade < 3 {
                next_multiple_of_5
            } else {
                grade
            }
        };

        println!("{}", final_grade);
    }
}
