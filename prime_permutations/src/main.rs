fn main() {
    for i in 1000..10_0000 {
        if is_prime(i) {
            let b = i + 3330;
            let c = b + 3330;

            if is_permutation(i, b) && is_permutation(i, c) && is_prime(b) && is_prime(c) {
                println!("{:?}{:?}{:?}", i, b, c);
            }
        }
    }
}

fn is_prime(number: i32) -> bool {
    for i in 2..=(number as f32).sqrt().ceil() as i32 {
        if number % i == 0 {
            return false;
        }
    }
    true
}

fn is_permutation(a: i32, b: i32) -> bool {
    let mut a = convert_number_to_digits(a);
    let mut b = convert_number_to_digits(b);
    a.sort();
    b.sort();
    a == b
}

fn convert_number_to_digits(number: i32) -> Vec<i32> {
    number.to_string().chars().map(|x| x as i32 - 48).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation() {
        let a = 1487;
        let b = 4817;

        let result = is_permutation(a, b);

        assert!(result);
    }
}
