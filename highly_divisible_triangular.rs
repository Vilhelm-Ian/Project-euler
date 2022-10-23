use std::thread;
2
​
3
fn main() {
4
    let result = find_first_triangular_number_with_over_n_divisors(500);
5
    println!("the solution is {}", result.unwrap())
6
}
7
​
8
fn find_first_triangular_number_with_over_n_divisors(n: u128) -> Result<u128, String> {
9
    let mut primes = vec![];
10
    let mut i = smallest_number_with_n_divisors(n, &mut primes);
11
    let mut last_addend = 0;
12
    let mut is_triangular_number_found = false;
13
    loop {
14
        if is_triangular_number_found || is_triangular(i) {
15
            is_triangular_number_found = true;
16
            if get_factors(i, &mut primes).len() > n as usize {
17
                return Ok(i);
18
            }
19
            if last_addend == 0 {
20
                last_addend = get_last_addend_for_triangular_number(i) as u128;
21
            }
22
            last_addend += 1;
23
            i += last_addend;
24
            continue;
25
        }
26
        i += 1;
27
    }
28
    Err(String::from("result not found"))
29
}
30
​
31
fn is_triangular(number: u128) -> bool {
32
    let result = get_last_addend_for_triangular_number(number);
33
    result.fract() == 0.0
34
}
35
​
36
fn get_last_addend_for_triangular_number(number: u128) -> f32 {
37
    let c = number as f32 * -2.0;
38
    (-1.0 + (1.0 - 4.0 * c).sqrt()) / 2.0 //quadratic formula
39
}
40
​
41
fn get_factors(number: u128, memoziation: &mut Vec<u128>) -> Vec<u128> {
42
    let handler1 = thread::spawn(move || {
43
        let mut result = vec![];
44
        for i in 1..number / 4 {
45
            if number % i == 0 {
