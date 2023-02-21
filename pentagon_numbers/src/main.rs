const LIMIT: i32 = 10_000_000;

fn main() {
    let mut numbers = vec![0; LIMIT as usize];
    let mut n = 0;
    while n < LIMIT {
        let pentagon_number = generate_pentagon_number(n);
        if pentagon_number > LIMIT {
            break;
        }
        if pentagon_number < LIMIT {
            numbers[pentagon_number as usize] = pentagon_number;
        }
        n += 1;
    }
    let pentagon_numbers: Vec<i32> = numbers.iter().filter(|x| *x != &0).cloned().collect();
    for i in 0..pentagon_numbers.len() {
        for z in i + 1..pentagon_numbers.len() {
            if let Some(difference) =
                is_sum_differenc_pentagon(pentagon_numbers[i], pentagon_numbers[z], &numbers)
            {
                println!("{:?}", difference);
            }
        }
    }
    println!("Hello, world!");
}

fn generate_pentagon_number(n: i32) -> i32 {
    n * (3 * n - 1) / 2
}

fn is_sum_differenc_pentagon(a: i32, b: i32, pentagon_numbers: &Vec<i32>) -> Option<i32> {
    let sum = a + b;
    let difference = b - a;
    if sum < LIMIT {
        if pentagon_numbers[sum as usize] != 0 && pentagon_numbers[difference as usize] != 0 {
            return Some(difference);
        }
    }
    None
}
