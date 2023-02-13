fn main() {
    let mut result = vec![1];
    for i in 1..=100 {
        result = multiply_vec(&result, i)
    }
    let sum: usize = result.iter().sum();
    println!("{:?}", sum);
}

fn multiply_vec(digits: &Vec<usize>, multiplyer: usize) -> Vec<usize> {
    let multiplyer: Vec<usize> = generate_vec_from_number(multiplyer)
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

fn generate_vec_from_number(mut number: usize) -> Vec<usize> {
    number
        .to_string()
        .chars()
        .map(|digit| digit as usize - 48)
        .collect()
}

fn add_vectors(vectors: &mut Vec<Vec<usize>>) -> Vec<usize> {
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

fn normalize_vectors(vectors: &mut Vec<Vec<usize>>) {
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

fn convert_to_single_digit(number: &mut Vec<usize>) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize() {
        let mut vectors = vec![vec![1, 2], vec![2]];
        normalize_vectors(&mut vectors);
        let expected = vec![0, 2];
        assert_eq!(vectors[1], expected);
    }
    #[test]
    fn sum_vectors() {
        let mut vectors = vec![vec![1, 2], vec![2]];
        let result = add_vectors(&mut vectors);
        let expected = vec![1, 4];
        assert_eq!(result, expected);
    }
    #[test]
    fn single_digit() {
        let mut result = vec![9, 15];
        convert_to_single_digit(&mut result);
        let expected = vec![1, 0, 5];
        assert_eq!(result, expected);
    }
}
