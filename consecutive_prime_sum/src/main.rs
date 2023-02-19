use rayon::prelude::*;
fn main() {
    let numbers: Vec<i32> = (1..1_000_000).collect();
    let primes_map: Vec<i32> = numbers
        .par_iter()
        .map(|x| {
            if is_prime(x) {
                return *x;
            }
            0
        })
        .collect();
    let primes: Vec<i32> = primes_map.iter().filter(|x| **x != 0).cloned().collect();
    let result = find_biggest_series(primes, primes_map);
    println!("{:?}", result);
}

fn is_prime(number: &i32) -> bool {
    if number == &1 {
        return false;
    }
    if number == &2 {
        return true;
    }
    for i in 2..=(*number as f32).sqrt().ceil() as i32 {
        if number % i == 0 {
            return false;
        }
    }
    true
}

fn find_biggest_series(primes: Vec<i32>, primes_map: Vec<i32>) -> i32 {
    let mut result = vec![];
    for i in 0..primes.len() {
        let mut temp = vec![];
        for prime in primes.iter().take(primes.iter().len()).skip(i) {
            temp.push(*prime);
            let sum: i32 = temp.iter().sum();
            if sum >= 1_000_000 {
                break;
            }
            if primes_map[sum as usize - 1] != 0 && temp.len() > result.len() {
                result = temp.clone();
            }
        }
    }
    result.iter().sum()
}
