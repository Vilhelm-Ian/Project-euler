pub fn generate_first_fibonachi_number_with_n_digits(
    first: u128,
    second: u128,
    digits: u128,
) -> u128 {
    let result = first + second;
    println!("{},{}", result, first);
    if result.to_string().len() == digits as usize {
        return result;
    }
    generate_first_fibonachi_number_with_n_digits(second, result, digits)
}

fn main() {
    let result = generate_first_fibonachi_number_with_n_digits(1, 1, 1000);
    println!("${}", result);
}
