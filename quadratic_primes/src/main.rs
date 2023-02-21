fn main() {
    let mut biggest = vec![0, 0, 0];
    for a in -999..1000 {
        for b in -1000..=1000 {
            let number_primes = quadratic_formula(a, b);
            if number_primes > biggest[0] {
                biggest = vec![number_primes, a, b];
            }
        }
    }
    let product = biggest[1] * biggest[2];
    println!("{:?}", biggest);
}

fn quadratic_formula(a: i32, b: i32) -> i32 {
    let mut n = 0;
    loop {
        let result = (n * n) + (a * n) + b;
        if !is_prime(&result) {
            break;
        }
        n += 1;
    }
    n
}

fn is_prime(number: &i32) -> bool {
    if number == &2 {
        return true;
    }
    if number == &1 {
        return false;
    }
    for i in 2..=(*number as f32).sqrt().ceil() as i32 {
        if number % i == 0 {
            return false;
        }
    }
    true
}
