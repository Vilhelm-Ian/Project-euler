use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut primes: HashSet<i32> = HashSet::new();
    let mut number_divisors_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let numbers: Vec<i32> = (1..1_000_000).collect();
    let solution = find_n_distinct_primes(4, numbers, &mut primes, &mut number_divisors_map);
    println!("{:?}", solution);
}

fn find_n_distinct_primes(
    n: i32,
    numbers: Vec<i32>,
    primes: &mut HashSet<i32>,
    map: &mut HashMap<i32, Vec<i32>>,
) -> Option<i32> {
    for i in 3..numbers.len() as i32 - n {
        let mut factors: Vec<i32> = vec![];
        for z in 0..n {
            let _factors;
            let map_get = map.get(&(i + z));
            if map_get.is_some() {
                _factors = map_get.unwrap().clone();
            } else {
                _factors = get_factors(i + z, primes);
                map.insert(i + z, _factors.clone());
            };
            map.insert(i + z, _factors.clone());
            if _factors.len() != n as usize {
                break;
            }
            factors.extend(_factors);
        }
        if factors.len() == (n * n) as usize {
            return Some(numbers[i as usize - 1]);
        }
    }
    None
}

fn get_factors(mut number: i32, primes: &mut HashSet<i32>) -> Vec<i32> {
    let mut solution: Vec<i32> = vec![];
    for prime in primes.iter() {
        if *prime > number {
            break;
        }
        if number % prime == 0 {
            number /= prime;
            if !solution.contains(prime) {
                solution.push(*prime)
            }
        }
    }
    if number == 1 {
        return solution;
    }
    let new_primes = get_primes_of_factor(number, primes);
    for prime in new_primes {
        primes.insert(prime);
        if number % prime == 0 {
            number /= prime;
            if !solution.contains(&prime) {
                solution.push(prime)
            }
        }
    }
    solution
}

fn get_primes_of_factor(number: i32, primes: &mut HashSet<i32>) -> Vec<i32> {
    let mut solution = vec![];
    for i in 2..=number {
        let get_prime = primes.get(&i);
        if get_prime.is_some() || is_prime(i) {
            solution.push(i)
        }
    }
    solution
}

fn is_prime(number: i32) -> bool {
    if number == 2 {
        return true;
    }
    for i in 2..=(number as f32).sqrt().ceil() as i32 {
        if number % i == 0 {
            return false;
        }
    }
    true
}
