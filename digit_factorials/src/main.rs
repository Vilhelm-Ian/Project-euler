fn main() {
    let test = is_num_equal_to_sum_factorial(145);
    let result: i32 = (3..1_000_000)
        .filter(|x| is_num_equal_to_sum_factorial(*x))
        .sum();
    println!("{:?}", result);
}

fn is_num_equal_to_sum_factorial(num: i32) -> bool {
    num == factorials_of_digits(num)
}

fn factorials_of_digits(num: i32) -> i32 {
    num.to_string()
        .chars()
        .fold(0, |acc, ele| acc + factorial(ele as i32 - 48))
}

fn factorial(num: i32) -> i32 {
    (1..=num).product()
}
