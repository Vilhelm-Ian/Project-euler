fn main() {
    let mut abbundant_numbers = vec![];
    let mut numbers: Vec<i32> = (1..=28123).collect();
    for i in 2..=28123 {
        let divisors = get_divisors(i);
        if is_abundant(divisors, i) {
            abbundant_numbers.push(i)
        }
    }
    for i in 0..abbundant_numbers.len() {
        for z in i..abbundant_numbers.len() {
            let sum = abbundant_numbers[i] + abbundant_numbers[z];
            if sum > 28123 {
                break;
            }
            numbers[sum as usize - 1] = 0;
        }
    }
    println!("{:?}", numbers.iter().sum::<i32>());
}

fn get_divisors(num: i32) -> Vec<i32> {
    let mut result = vec![1];
    for i in 2..=(num as f32).sqrt().floor() as i32 {
        if num % i == 0 {
            result.push(i);
            if i != num / i {
                result.push(num / i)
            }
        }
    }
    result
}

fn is_abundant(divisors: Vec<i32>, num: i32) -> bool {
    divisors.iter().sum::<i32>() > num
}
