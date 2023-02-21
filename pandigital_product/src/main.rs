use std::collections::HashSet;

fn main() {
    let mut numbers = HashSet::new();
    for i in 10..99 {
        for z in 100..999 {
            let c = i * z;
            if c > 10_000 {
                break;
            }
            if is_pandigal_n(9, vec![i, z, c]) {
                //    println!("{:?} {:?} {:?}", i, z, c);
                numbers.insert(c);
            }
        }
    }
    for i in 1..=9 {
        for z in 1000..9999 {
            let c = i * z;
            if is_pandigal_n(9, vec![i, z, c]) {
                //   println!("{:?} {:?} {:?}", i, z, c);
                numbers.insert(c);
            }
        }
    }
    let mut solution = 0;
    for number in numbers {
        solution += number;
    }
    println!("{:?}", solution);
}

fn is_pandigal_n(n: i32, numbers: Vec<i32>) -> bool {
    let numbers_digits: Vec<i32> = numbers.iter().fold(vec![], |mut acc, ele| {
        let number_vec = num_to_vec(*ele);
        acc.extend(number_vec);
        acc
    });
    !numbers_digits.contains(&0)
        && numbers_digits.len() == n as usize
        && are_digits_different(numbers)
}

fn are_digits_different(numbers: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    let numbers_vec: Vec<i32> = numbers.iter().fold(vec![], |mut acc, ele| {
        let number_vec = num_to_vec(*ele);
        acc.extend(number_vec);
        acc
    });
    for number in numbers_vec.iter() {
        set.insert(number);
    }
    set.len() == numbers_vec.len()
}

fn num_to_vec(number: i32) -> Vec<i32> {
    number.to_string().chars().map(|x| x as i32 - 48).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_different() {
        let input = vec![123, 456, 789];
        let result = are_digits_different(input);

        assert!(result);
    }
    #[test]
    fn digits_not_different() {
        let input = vec![123, 426, 789];
        let result = are_digits_different(input);

        assert!(!result);
    }
    #[test]
    fn not_pandigal() {
        let input = vec![123, 406, 789];
        let result = is_pandigal_n(9, input);

        assert!(!result);
    }
}
