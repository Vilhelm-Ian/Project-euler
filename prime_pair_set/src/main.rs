use std::collections::HashSet;

fn main() {
    let primes_set = find_primes_upto_range(1_000_000);
    let mut primes_vec: Vec<u128> = primes_set.clone().into_iter().collect();
    primes_vec.sort();
    let mut primes = None;
    let mut n = 0;
    while primes.is_none() {
        primes = brute_force(n, &primes_vec, &primes_set);
        n += 1;
    }

    println!("{:?}", primes);
}

fn find_primes_upto_range(number: u128) -> HashSet<u128> {
    let mut primes = HashSet::new();
    let mut not_primes = HashSet::new();
    for i in 2..=(number as f32).sqrt().ceil() as u128 {
        if is_prime(i) {
            primes.insert(i);
            for z in 2..=number / i {
                not_primes.insert(i * z);
            }
        }
    }
    for i in (number as f32).sqrt().ceil() as u128..number {
        if not_primes.get(&i).is_none() {
            primes.insert(i);
        }
    }
    primes
}

fn brute_force(num: usize, primes_vec: &[u128], primes_set: &HashSet<u128>) -> Option<Vec<u128>> {
    let mut primes = vec![primes_vec[num]];
    for prime in primes_vec.iter().skip(num) {
        //if prime > &100_000 {
        //    break;
        // }
        primes.push(*prime);
        if solution(&primes, primes_set).is_none() {
            primes.pop();
        }
        if primes.len() == 5 {
            return Some(primes);
        }
    }
    None
}

fn solution(primes: &Vec<u128>, primes_set: &HashSet<u128>) -> Option<u128> {
    for index_a in 0..primes.len() - 1 {
        for index_b in index_a + 1..primes.len() {
            if !is_concate_primes_prime(primes[index_a], primes[index_b], primes_set) {
                return None;
            }
        }
    }
    Some(primes.iter().sum())
}

fn is_concate_primes_prime(prime_a: u128, prime_b: u128, primes_set: &HashSet<u128>) -> bool {
    let first_var = format!("{}{}", prime_a, prime_b);
    let second_var = format!("{}{}", prime_b, prime_a);
    if primes_set.get(&convert_str_to_num(first_var)).is_none()
        || primes_set.get(&convert_str_to_num(second_var)).is_none()
    {
        return false;
    };
    true
}

fn is_prime(number: u128) -> bool {
    if number == 2 {
        return true;
    }
    if number <= 1 {
        return false;
    }
    for i in 2..=(number as f32).sqrt().ceil() as u128 {
        if number % i == 0 {
            return false;
        }
    }
    true
}

fn convert_str_to_num(num: String) -> u128 {
    let mut solution = 0;
    for (index, digit) in num.chars().enumerate() {
        solution += (digit as u128 - 48) * 10_u128.pow((num.len() - index - 1) as u32);
    }
    solution
}
