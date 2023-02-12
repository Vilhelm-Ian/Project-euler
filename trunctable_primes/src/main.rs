fn main() {
    let mut result = vec![];
    let mut i = 11;
    while result.len() != 11 {
        if is_truncable_prime(&i) {
            result.push(i);
        }
        i += 2;
    }
    let sum: u32 = result.iter().sum();
    println!("{:?}", sum);
}

fn is_truncable_prime(number: &u32) -> bool {
    let mut right = trunc_right(&number);
    let left = trunc_left(&number);
    right.extend(left);
    for number in right {
        if !is_prime(number) {
            return false;
        }
    }
    true
}

fn trunc_right(number: &u32) -> Vec<u32> {
    let str = number.to_string();
    let mut result = vec![];
    for i in 1..str.len() {
        result.push(number / 10_u32.pow(i as u32))
    }
    result
}

fn trunc_left(number: &u32) -> Vec<u32> {
    let str = number.to_string();
    let mut result = vec![];
    for (index, letter) in str.chars().rev().enumerate() {
        let mut number = letter.to_digit(10).unwrap();
        if result.len() != 0 {
            let length = result.len();
            number = number * 10_u32.pow(index as u32) + result[length - 1];
        }
        result.push(number)
    }
    result
}

fn is_prime(number: u32) -> bool {
    if number == 1 {
        return false;
    }
    if number == 2 {
        return true;
    }
    for i in 2..=(number as f32).sqrt().ceil() as u32 {
        if number % i == 0 {
            return false;
        }
    }
    true
}
