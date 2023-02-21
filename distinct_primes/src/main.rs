const LIMIT: i32 = 4;

fn main() {
    let numbers: Vec<i32> = (1..1_000_000).collect();
    let solution = find_n_distinct_primes(LIMIT, numbers);
    println!("{:?}", solution);
}

fn find_n_distinct_primes(n: i32, numbers: Vec<i32>) -> Option<i32> {
    let mut i = 3;
    while i < numbers.len() as i32 - n {
        let mut factors: Vec<i32> = vec![];
        for z in 0..n {
            let _factors = get_factors(i + z);
            //println!("{:?} {:?}", _factors, i + z);
            if _factors.len() != n as usize {
                i += z;
                break;
            }
            factors.extend(_factors);
        }
        if factors.len() == (n * n) as usize {
            return Some(numbers[i as usize - 1]);
        }
        i += 1;
    }
    None
}

fn get_factors(mut number: i32) -> Vec<i32> {
    let number_copy = number;
    let mut solution: Vec<i32> = vec![];
    if number % 2 == 0 {
        solution.push(2);
        number /= 2;
    }
    let mut i = 3;
    while number != 1 && i < number_copy / 2 {
        //println!("{:?} {:?} {:?}", number % i, number, i);
        if number % i == 0 && is_prime(i) {
            number /= i;
            if !solution.contains(&i) {
                solution.push(i)
            };
            if solution.len() > LIMIT as usize {
                break;
            }
        }

        i += 2;
    }
    solution
}

fn is_prime(number: i32) -> bool {
    if number == 2 {
        return true;
    }
    for i in 2..=(number as f32).sqrt().ceil() as i32 {
        if number % i == 0 {
            return false;
        }
    }
    true
}
