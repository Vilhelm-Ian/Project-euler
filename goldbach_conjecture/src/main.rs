fn main() {
    let numbers: Vec<i32> = (1..=1_000_000)
        .map(|x| {
            if is_prime(x) {
                return x;
            };
            0
        })
        .collect();
    let primes: Vec<&i32> = numbers.iter().filter(|x| *x != &0).collect();
    for (index, number) in numbers.iter().enumerate() {
        //println!("{:?} {:?}", index, number);
        if number == &0 && index % 2 == 0 {
            if !find_solution_for_n(index as i32 + 1, &primes) {
                println!("{:?}", index + 1);
            }
        }
    }
    println!("Hello, world!");
}

fn find_solution_for_n(number: i32, primes: &Vec<&i32>) -> bool {
    for i in 1..(number as f32 / 2.0).sqrt().ceil() as i32 {
        for prime in primes.iter() {
            if *prime >= &number {
                break;
            }
            if formula(**prime, i) == number {
                //println!("number: {:?} ,{:?} {:?}", number, prime, i);
                return true;
            }
        }
    }
    false
}

fn formula(prime: i32, root: i32) -> i32 {
    prime + 2 * root * root
}

fn is_prime(number: i32) -> bool {
    if number == 2 {
        return true;
    }
    if number == 1 {
        return false;
    }
    for i in 2..=(number as f32).sqrt().ceil() as i32 {
        if number % i == 0 {
            return false;
        }
    }
    true
}
