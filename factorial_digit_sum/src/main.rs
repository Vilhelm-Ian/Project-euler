fn main() {
    println!("Hello, world!");
    let result = calculate_factorial(100);
    println!("{:?}", result);
}

fn calculate_factorial(n: u128) -> u128 {
    let mut result = 1;
    for z in 1..4 + 1 {
        for i in (n / 4 * (z - 1)) + 1..(n / 4 * z) + 1 {
            println!("{}, {}", i, z);
            result *= i
        }
    }
    result
}
