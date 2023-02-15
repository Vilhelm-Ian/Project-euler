fn main() {
    let mut result = 0;
    for i in 1..10_000 {
        if is_amicable(i) {
            result += i
        }
    }
    println!("{:?}", result);
}

fn get_sum_of_divisors(number: i32) -> i32 {
    let mut result = 1;
    for i in 2..(number as f32).sqrt().ceil() as i32 {
        if number % i == 0 {
            result += i + number / i
        }
    }
    result
}

fn is_amicable(number: i32) -> bool {
    let second_number = get_sum_of_divisors(number);
    second_number != number && get_sum_of_divisors(second_number) == number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisors() {
        let result = get_sum_of_divisors(220);
        let expected = 284;
        assert_eq!(result, expected);
    }
    #[test]
    fn amicable() {
        let result = is_amicable(220);
        assert!(result);
    }
}
