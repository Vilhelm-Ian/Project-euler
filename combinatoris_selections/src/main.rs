fn main() {
    let mut result = 0;
    for n in 1..=100 {
        for r in 1..=n {
            let number_of_combinations = how_many_combinations(n, r);
            if number_of_combinations > 1_000_000 {
                result += 1;
            }
        }
    }
    println!("{:?}", result);
}

fn how_many_combinations(n: u128, r: u128) -> u128 {
    let mut n_factorial: Vec<u128> = (1..=n).rev().collect();
    let mut n_minus_r_factorial: Vec<u128> = (1..=n - r).rev().collect();
    n_factorial.resize((n - r) as usize, 0);
    cancel_out(&mut n_factorial, &mut n_minus_r_factorial);
    cancel_out(&mut n_minus_r_factorial, &mut n_factorial);
    let product1: u128 = n_factorial.iter().product();
    let product2: u128 = n_minus_r_factorial.iter().product();
    product1 / product2
}
fn cancel_out(vec1: &mut Vec<u128>, vec2: &mut Vec<u128>) {
    let mut indexes = vec![];
    for (index, number2) in vec2.into_iter().enumerate() {
        for number1 in vec1.into_iter() {
            if *number1 % *number2 == 0 {
                *number1 /= *number2;
                indexes.push(index);
                break;
            }
        }
    }
    for index in indexes {
        vec2[index] = 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancel() {
        let mut vec1 = vec![5, 4];
        let mut vec2 = vec![3, 2, 1];
        cancel_out(&mut vec1, &mut vec2);
        assert_eq!(vec1, vec![5, 2]);
        assert_eq!(vec2, vec![3, 1, 1]);
    }
}
