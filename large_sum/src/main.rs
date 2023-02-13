mod input;

fn main() {
    let mut result = vec![0; 50];
    let numbers: Vec<Vec<i32>> = input::INPUT
        .split('\n')
        .map(|number| number.chars().map(|digit| digit as i32 - 48).collect())
        .collect();
    for number in numbers {
        for (index, digit) in number.iter().enumerate() {
            result[index] += digit;
        }
    }
    result.insert(0, 0);

    for index in (1..51).rev() {
        if result[index] > 10 {
            let temp = result[index];
            result[index] = result[index] % 10;
            result[index - 1] += temp / 10;
        }
    }

    println!("{:?}", &result[0..10]);
}

// fn are_all_singe_digit(digits: Vec<i32>) -> bool {}
