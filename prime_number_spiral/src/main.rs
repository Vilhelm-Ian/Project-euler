fn main() {
    let mut diagonal = 1;
    let mut adder = 2;
    let mut primes = 0.0;
    let mut diagonal_numbers = 1.0;
    let mut result = 0;
    while primes == 0.0 || primes / diagonal_numbers > 0.1 {
        result += 1;
        for _ in 0..4 {
            diagonal += adder;
            diagonal_numbers += 1.0;
            if is_prime(diagonal) {
                primes += 1.0;
            }
        }
        adder += 2
    }
    println!("{:?}", result * 2 + 1);
}

fn is_prime(number: i32) -> bool {
    if number == 2 {
        return true;
    }
    if number <= 1 {
        return false;
    }
    for i in 2..=(number as f32).sqrt().ceil() as i32 {
        if number % i == 0 {
            return false;
        }
    }
    true
}
