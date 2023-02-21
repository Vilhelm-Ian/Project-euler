use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let numbers: Vec<i32> = (1..=1_000).collect();
    let before = Instant::now();
    let mut powers = numbers
        .par_iter()
        .fold(Vec::new, |mut acc: Vec<Vec<i32>>, i| {
            let mut num = generate_vec_from_number(*i);
            for _ in 1..*i {
                num = multiply_vec_par(&num, *i);
            }
            acc.push(num);
            acc
        })
        .reduce(Vec::new, |mut acc, ele| {
            acc.extend(ele);
            acc
        });

    println!("Elapsed time: {:.2?}", before.elapsed());
    let len = powers.len();
    let solution = add_vectors(&mut powers);
    println!("the solution is {:?}", solution);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn add_vectors(vectors: &mut Vec<Vec<i32>>) -> Vec<i32> {
    normalize_vectors(vectors);
    let mut result = vec![0; vectors[0].len()];
    for vector in vectors {
        for (index, digit) in vector.iter().enumerate() {
            result[index] += digit;
        }
    }
    convert_to_single_digit(&mut result);
    result
}

fn normalize_vectors(vectors: &mut Vec<Vec<i32>>) {
    let mut biggest = 0;
    for vector in vectors.iter_mut() {
        if vector.len() > biggest {
            biggest = vector.len()
        }
    }
    for vector in vectors {
        while vector.len() < biggest {
            vector.insert(0, 0)
        }
    }
}

fn multiply_vec(digits: &Vec<i32>, multiplyer: i32) -> Vec<i32> {
    let multiplyer: Vec<i32> = generate_vec_from_number(multiplyer)
        .into_iter()
        .rev()
        .collect();
    let mut vectors = vec![];
    for (index, multiplyer_digit) in multiplyer.iter().enumerate() {
        let mut temp = vec![];
        for digit in digits {
            temp.push(digit * multiplyer_digit)
        }
        temp.extend(vec![0; index]);
        vectors.push(temp)
    }
    add_vectors(&mut vectors)
}

fn multiply_vec_par(digits: &Vec<i32>, multiplyer: i32) -> Vec<i32> {
    let multiplyer: Vec<i32> = generate_vec_from_number(multiplyer)
        .into_iter()
        .rev()
        .collect();
    let mut vectors = multiplyer
        .par_iter()
        .enumerate()
        .fold(
            Vec::new,
            |mut vectors: Vec<Vec<i32>>, (index, multiplyer_digit)| {
                let mut temp = vec![];
                for digit in digits {
                    temp.push(digit * multiplyer_digit)
                }
                temp.extend(vec![0; index]);
                vectors.push(temp);
                vectors
            },
        )
        .reduce(Vec::new, |mut acc, ele| {
            acc.extend(ele);
            acc
        });
    add_vectors(&mut vectors)
}

fn generate_vec_from_number(number: i32) -> Vec<i32> {
    number
        .to_string()
        .chars()
        .map(|digit| digit as i32 - 48)
        .collect()
}

fn convert_to_single_digit(number: &mut Vec<i32>) {
    for mut index in (0..number.len()).rev() {
        if number[index] > 9 {
            if index == 0 {
                number.insert(0, 0);
                index = 1;
            }
            let temp = number[index];
            number[index] %= 10;
            number[index - 1] += temp / 10;
        }
    }
}
