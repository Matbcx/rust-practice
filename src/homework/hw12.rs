use rand::{thread_rng, Rng};

pub fn count_permutation(values: &Vec<u32>) -> isize {
    let len = values.len();
    let sum: u32 = values.iter().sum();

    if sum % len as u32 != 0 {
        return -1;
    }

    let average = sum / len as u32;
    let mut steps = 0;
    let mut diff = 0i32;

    for &val in values {
        diff += val as i32 - average as i32;
        steps += diff.abs() as usize;
    }

    steps as isize
}

pub fn count_permutation_result(values: &Vec<u32>) -> Result<usize, &'static str> {
    let len = values.len();
    let sum: u32 = values.iter().sum();

    if sum % len as u32 != 0 {
        return Err("Рівномірний розподіл неможливий");
    }

    let average = sum / len as u32;
    let mut steps = 0;
    let mut diff = 0i32;

    for &val in values {
        diff += val as i32 - average as i32;
        steps += diff.abs() as usize;
    }

    Ok(steps)
}

pub fn generate_shipments(size: usize) -> Vec<u32> {
    let mut rng = thread_rng();
    let avg = rng.gen_range(1..=20);
    let mut shipments = vec![avg; size];

    for _ in 0..size {
        let i = rng.gen_range(0..size);
        let j = rng.gen_range(0..size);
        if i != j && shipments[i] > 0 {
            let change = rng.gen_range(0..=shipments[i]);
            shipments[i] -= change;
            shipments[j] += change;
        }
    }

    shipments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let data = vec![1, 1, 1, 1, 6];
        assert_eq!(count_permutation(&data), 4);
    }

    #[test]
    fn test_case_2() {
        let data = vec![8, 2, 2, 4, 4];
        assert_eq!(count_permutation(&data), 4);
    }

    #[test]
    fn test_case_3() {
        let data = vec![9, 3, 7, 2, 9];
        assert_eq!(count_permutation(&data), 7);
    }

    #[test]
    fn test_invalid_case() {
        let data = vec![1, 2, 3];
        assert_eq!(count_permutation(&data), -1);
    }

    #[test]
    fn test_generated_data() {
        let data = generate_shipments(10);
        assert_ne!(count_permutation(&data), -1);
    }
}
