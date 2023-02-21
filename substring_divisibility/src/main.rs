use rayon::prelude::*;

const PRIMES: [u128; 7] = [2, 3, 5, 7, 11, 13, 17];

fn main() {
    let mut start = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut permutations = vec![start.clone()];
    let k = start.len();
    heaps_algorithm(k as i32, &mut start, &mut permutations);
    let result: u128 = permutations
        .par_iter()
        .filter(|x| is_property(x))
        .map(|x| vec_to_num(x))
        .sum();
    println!("{:?}", result);
}

fn is_property(digits: &Vec<i32>) -> bool {
    let chunks = chunk_vec(digits);
    for (index, chunk) in chunks.iter().enumerate() {
        let number = vec_to_num(chunk);
        if number % PRIMES[index] != 0 {
            return false;
        }
    }
    true
}

fn heaps_algorithm(k: i32, array: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if k == 1 {
        return;
    }
    heaps_algorithm(k - 1, array, result);
    for i in 0..k - 1 {
        if k % 2 == 0 {
            let k = k as usize;
            let i = i as usize;
            (array[i], array[k - 1]) = (array[k - 1], array[i])
        } else {
            let k = k as usize;
            (array[0], array[k - 1]) = (array[k - 1], array[0])
        }
        result.push(array.clone());

        heaps_algorithm(k - 1, array, result);
    }
}

fn chunk_vec(vector: &Vec<i32>) -> Vec<[i32; 3]> {
    let mut result = vec![];
    for i in 1..vector.len() - 2 {
        result.push([vector[i], vector[i + 1], vector[i + 2]]);
    }
    result
}

fn vec_to_num(vector: &[i32]) -> u128 {
    let mut result = 0;
    for (index, number) in vector.iter().rev().enumerate() {
        result += *number as u128 * 10_u128.pow(index as u32);
    }
    result
}
