use rayon::prelude::*;
use std::collections::HashSet;

fn main() {
    let mut result: Vec<i32> = (3..1_000_000)
        .into_par_iter()
        .filter(|i| i % 2 != 0)
        .filter(|i| is_circular_prime(*i))
        .collect();
    result.push(2);
    println!("Hello, world! {:?}", result.len());
    println!("Hello, world! {:?}", result);
}

fn generate_rotations(number: i32) -> Vec<i32> {
    let vector = number_to_vecor(number);
    let mut result = vec![];
    result.push(number);
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

fn is_circular_prime(number: i32) -> bool {
    let permutations = generate_rotations(number);
    for permutation in permutations {
        if !is_prime(permutation) {
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
        let result = is_circular_prime(input);
        assert_eq!(expected, result);
    }
}
