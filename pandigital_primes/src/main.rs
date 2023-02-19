fn main() {
    let result = find_largest_pandigital_prime_n(9);
    println!("{}", result.unwrap());
}

fn find_largest_pandigital_prime_n(n: i32) -> Option<i32> {
    if n == 0 {
        return None;
    }
    let mut digits: Vec<i32> = (1..=n).collect();
    let mut result = vec![digits.clone()];
    let k = digits.len();
    heaps_algorithm(k as i32, &mut digits, &mut result);
    result.sort();
    for index in (0..result.len()).rev() {
        let number = number_to_vec(result[index].clone());
        if is_prime(number) {
            return Some(number);
        }
    }
    find_largest_pandigital_prime_n(n - 1)
}

fn is_prime(number: i32) -> bool {
    for i in 2..=(number as f32).sqrt().ceil() as i32 {
        if number % i == 0 {
            return false;
        }
    }
    true
}

fn heaps_algorithm(k: i32, digits: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if k == 1 {
        return;
    }
    heaps_algorithm(k - 1, digits, result);
    for i in 0..k - 1 {
        let i = i as usize;
        if k % 2 == 0 {
            let k = k as usize;
            (digits[i], digits[k - 1]) = (digits[k - 1], digits[i]);
        } else {
            let k = k as usize;
            (digits[0], digits[k - 1]) = (digits[k - 1], digits[0]);
        }
        result.push(digits.clone());
        heaps_algorithm(k - 1, digits, result);
    }
}

fn number_to_vec(digits: Vec<i32>) -> i32 {
    digits
        .iter()
        .map(|x| (*x).to_string())
        .fold(String::new(), |acc, ele| format!("{}{}", acc, ele))
        .parse()
        .unwrap()
}
