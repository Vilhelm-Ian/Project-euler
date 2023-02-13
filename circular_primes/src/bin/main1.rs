// use rayon::prelude::*;
use std::collections::HashSet;
use std::thread;

fn main() {
    let mut result = vec![];
    let mut handles = vec![];
    for i in 1..=6 {
        handles.push(thread::spawn(move || {
            let mut set = HashSet::new();
            let mut primes = vec![];
            for z in 1 * 10_i32.pow(i - 1)..1 * 10_i32.pow(i) {
                if z % 2 == 0 || set.get(&z).is_some() {
                    continue;
                }
                let rotations = generate_rotations(z);
                for rotation in &rotations {
                    set.insert(*rotation);
                }
                if is_circular_prime(&rotations) {
                    primes.extend(rotations);
                }
            }
            return primes;
        }));
    }
    for handle in handles {
        let primes = handle.join().unwrap();
        result.extend(primes)
    }
    println!("{:?}", result.len());
    println!("{:?}", result);
}

fn generate_rotations(number: i32) -> Vec<i32> {
    let vector = number_to_vecor(number);
    let mut result = vec![];
    //result.push(number);
    let mut vector_clone = vector.clone();
    let last_element = vector_clone.pop().unwrap();
    vector_clone.insert(0, last_element);
    result.push(vec_to_number(&vector_clone));
    while vector_clone != vector {
        let last_element = vector_clone.pop().unwrap();
        vector_clone.insert(0, last_element);
        result.push(vec_to_number(&vector_clone));
    }
    result
}

fn number_to_vecor(number: i32) -> Vec<i32> {
    number.to_string().chars().map(|x| x as i32 - 48).collect()
}

fn vec_to_number(vector: &Vec<i32>) -> i32 {
    let mut result = 0;
    for (index, number) in vector.iter().enumerate() {
        result += number * i32::pow(10, vector.len() as u32 - 1 - index as u32)
    }
    result
}

fn is_prime(number: i32) -> bool {
    for i in 2..=(number as f32).sqrt().ceil() as i32 {
        if number % i == 0 {
            return false;
        }
    }
    true
}

fn is_circular_prime(permutations: &Vec<i32>) -> bool {
    for permutation in permutations {
        if !is_prime(*permutation) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime() {
        assert_eq!(true, is_prime(23));
        assert_eq!(true, is_prime(971));
        assert_eq!(true, is_prime(5));
    }

    #[test]
    fn vec_to_num() {
        let input = vec![1, 2, 3];
        let expected = 123;
        let result = vec_to_number(&input);
        assert_eq!(expected, result);
    }
    #[test]
    fn is_circular() {
        let input = 197;
        let expected = true;
        let permutations = generate_rotations(input);
        let result = is_circular_prime(&permutations);
        assert_eq!(expected, result);
    }
}
