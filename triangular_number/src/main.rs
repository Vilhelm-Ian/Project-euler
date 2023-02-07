use std::thread;

fn main() {
    let result = find_first_triangular_number_with_over_n_divisors(500);
    println!("the solution is {:?},", result)
}

fn find_first_triangular_number_with_over_n_divisors(n: u128) -> Result<u128, String> {
    let mut primes = vec![];
    let mut i = smallest_number_with_n_divisors(n, &mut primes);
    let mut last_addend = 0;
    let mut is_triangular_number_found = false;
    let mut i = 0;
    loop {
        if is_triangular_number_found || is_triangular(i) {
            is_triangular_number_found = true;
            if get_factors(i, &mut primes).len() > n as usize {
                return Ok(i);
            }
            if last_addend == 0 {
                last_addend = get_last_addend_for_triangular_number(i) as u128;
            }
            last_addend += 1;
            i += last_addend;
            continue;
        }
        i += 1;
    }
    Err(String::from("result not found"))
}

fn is_triangular(number: u128) -> bool {
    let result = get_last_addend_for_triangular_number(number);
    result.fract() == 0.0
}

fn get_last_addend_for_triangular_number(number: u128) -> f32 {
    let c = number as f32 * -2.0;
    (-1.0 + (1.0 - 4.0 * c).sqrt()) / 2.0 //quadratic formula
}

fn get_factors(number: u128, memoziation: &mut Vec<u128>) -> Vec<u128> {
    //   let handler1 = thread::spawn(move || {
    //       let mut result = vec![];
    //       for i in 1..number / 4 {
    //           if number % i == 0 {
    //               result.push(i)
    //           }
    //       }
    //       return result;
    //   });
    //   let handler2 = thread::spawn(move || {
    //       let mut result = vec![];
    //       for i in (number / 4)..number / 2 + 1 {
    //           if number % i == 0 {
    //               result.push(i)
    //           }
    //       }
    //       return result;
    //   });
    //   let mut result = handler1.join().unwrap();
    //   let mut vec2 = handler2.join().unwrap();
    //   result.append(&mut vec2);
    let mut result = vec![];
    for i in 1..(number as f32).sqrt() as u128 + 1 {
        if number % i == 0 {
            let z = number / i;
            result.push(i);
            if i != z {
                result.push(z);
            }
        }
    }
    //let mut result = find_all_prime_factors(number, memoziation);
    //let primes = result.clone();
    //for prime in primes {
    //    let mut number_copy = number;
    //    while number_copy > 1 {
    //        number_copy /= prime;
    //        if !result.contains(&number_copy) && number_copy != 0 {
    //            result.push(number_copy);
    //        }
    //    }
    //}
    result
}

fn smallest_number_with_n_divisors(n: u128, memoziation: &mut Vec<u128>) -> u128 {
    let prime_factors: Vec<u128> = find_all_prime_factors(n, memoziation)
        .into_iter()
        .rev()
        .collect();
    let mut prime_divisors = vec![];
    for prime in &prime_factors {
        let mut n_copy = n;
        while n_copy % prime == 0 {
            n_copy /= prime;
            prime_divisors.push(prime)
        }
    }
    let primes: Vec<u128> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let mut result = 1;
    for i in 0..prime_divisors.len() {
        result *= primes[i].pow(*prime_divisors[i] as u32 - 1)
    }
    result
}

fn is_prime(n: u128, memoziation: &mut Vec<u128>) -> bool {
    if memoziation.contains(&n) {
        return true;
    }
    let first_primes: Vec<u128> = vec![1, 2, 3, 5];
    if first_primes.contains(&n) {
        return true;
    }
    for i in 2..(n as f64).sqrt().ceil() as u128 + 1 {
        if n % i == 0 {
            return false;
        }
    }
    memoziation.push(n);
    true
}

fn find_all_prime_factors(n: u128, memoziation: &mut Vec<u128>) -> Vec<u128> {
    let mut result = vec![];
    if is_prime(n, memoziation) {
        result.push(n)
    }
    for i in 2..n / 2 + 1 {
        if n % i == 0 && is_prime(i, memoziation) {
            result.push(i)
        }
        if !result.is_empty() && i > n / result[0] {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::find_all_prime_factors;
    use super::find_first_triangular_number_with_over_n_divisors;
    use super::get_factors;
    use super::is_prime;
    use super::is_triangular;
    use super::smallest_number_with_n_divisors;

    #[test]
    fn factors() {
        let mut primes = vec![];
        let mut factors_of_six = get_factors(6, &mut primes);
        factors_of_six.sort();
        let mut factors_of_twelve = get_factors(12, &mut primes);
        factors_of_twelve.sort();
        assert_eq!(vec![1, 2, 3, 6], factors_of_six);
        assert_eq!(vec![1, 2, 3, 4, 6, 12], factors_of_twelve);
    }

    #[test]
    fn triangular() {
        assert_eq!(true, is_triangular(3));
        assert_eq!(true, is_triangular(28));
        assert_eq!(false, is_triangular(27));
        assert_eq!(true, is_triangular(15));
        assert_eq!(false, is_triangular(22));
        assert_eq!(true, is_triangular(496));
    }

    #[test]
    fn find_first_triangular_number() {
        assert_eq!(
            6,
            find_first_triangular_number_with_over_n_divisors(3).unwrap()
        );
        assert_eq!(
            28,
            find_first_triangular_number_with_over_n_divisors(5).unwrap()
        );
        assert_eq!(
            73920,
            find_first_triangular_number_with_over_n_divisors(100).unwrap()
        );
    }

    #[test]
    fn primes() {
        let mut primes = vec![];
        assert_eq!(true, is_prime(1, &mut primes));
        assert_eq!(true, is_prime(2, &mut primes));
        assert_eq!(true, is_prime(5, &mut primes));
        assert_eq!(false, is_prime(4, &mut primes));
        assert_eq!(false, is_prime(6, &mut primes));
        assert_eq!(true, is_prime(7, &mut primes));
        assert_eq!(true, is_prime(23, &mut primes));
    }
    #[test]
    fn prime_factors() {
        let mut primes = vec![];
        assert_eq!(vec![2, 3], find_all_prime_factors(6, &mut primes));
        assert_eq!(vec![2], find_all_prime_factors(8, &mut primes));
        assert_eq!(vec![2, 5], find_all_prime_factors(10, &mut primes));
    }
    #[test]
    fn smallest_number_with_n_diviso() {
        let mut primes = vec![];
        assert_eq!(16, smallest_number_with_n_divisors(5, &mut primes));
        assert_eq!(12, smallest_number_with_n_divisors(6, &mut primes));
        assert_eq!(5184, smallest_number_with_n_divisors(35, &mut primes));
        assert_eq!(1296, smallest_number_with_n_divisors(25, &mut primes));
        assert_eq!(
            3779136000000,
            smallest_number_with_n_divisors(1001, &mut primes)
        );
    }
}
