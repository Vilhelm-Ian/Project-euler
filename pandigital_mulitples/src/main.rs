fn main() {
    let test = vec_to_num(vec![1, 2, 3]);
    println!("{:?}", test);
    let mut results = vec![];
    for i in 1..10_000 {
        let result = generate_pan_digital_num(i);
        if let Some(pandigit) = result {
            results.push(pandigit)
        }
    }
    println!("{:?}", results);
    let solution = results.iter().map(|x| vec_to_num(x.clone())).max();
    println!("result {:?}", solution);
}

fn is_no_repeating_digits(digits: &Vec<i32>) -> bool {
    let mut digits_clone = digits.clone();
    digits_clone.sort();
    digits_clone.dedup();
    digits.len() == digits_clone.len()
}

fn num_to_vec(num: i32) -> Vec<i32> {
    num.to_string().chars().map(|x| x as i32 - 48).collect()
}

fn generate_pan_digital_num(num: i32) -> Option<Vec<i32>> {
    let mut result = vec![];
    let mut i = 1;
    while result.len() < 9 {
        result.extend(num_to_vec(num * i));
        if !is_no_repeating_digits(&result) || result.contains(&0) {
            return None;
        }
        i += 1;
    }
    if result == vec![9, 8, 7, 6, 5, 4, 3, 2, 1] {
        println!("{:?}", num);
    }
    Some(result)
}

fn vec_to_num(digits: Vec<i32>) -> i32 {
    let mut result = 0;
    let len = digits.len();
    for (index, digit) in digits.iter().enumerate() {
        result += digit * 10_i32.pow(len as u32 - index as u32 - 1);
    }
    result
}
