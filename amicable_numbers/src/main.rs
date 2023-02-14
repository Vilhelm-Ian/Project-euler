fn main() {
    let mut result = 0;
    for i in 1..10_000 {
        if is_amicable(i) {
            result += i
        }
    }
    println!("{:?}", result);
}

fn get_divisors(number: i32) -> Vec<i32> {
    let mut result = vec![1];
    for i in 2..(number as f32).sqrt().ceil() as i32 {
        if number % i == 0 {
            result.push(i);
            result.push(number / i)
        }
    }
    result
}

fn is_amicable(number: i32) -> bool {
    let second_number: i32 = get_divisors(number).iter().sum();
    if second_number == number {
        return false;
    }
    get_divisors(second_number).iter().sum::<i32>() == number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisors() {
        let result = get_divisors(220);
        println!("{:?}", result);
        let result: i32 = result.iter().sum();
        let expected = 284;
        assert_eq!(result, expected);
    }
    #[test]
    fn amicable() {
        let result = is_amicable(220);
        assert!(result);
    }
}
