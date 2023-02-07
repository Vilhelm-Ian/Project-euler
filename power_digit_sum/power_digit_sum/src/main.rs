use num::bigint::{BigInt, ToBigInt};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let product: Arc<Mutex<BigInt>> = Arc::new(Mutex::new(ToBigInt::to_bigint(&2).unwrap()));
    let mut handles = Vec::new();
    for _ in 1..1000 {
        let product = Arc::clone(&product);
        let handle = thread::spawn(move || {
            let mut num = product.lock().unwrap();
            *num *= 2;
        });
        handles.push(handle)
    }

    for handel in handles {
        handel.join().unwrap()
    }

    let product_string = format!("{}", *product.lock().unwrap());
    let splited_product = product_string.split("");

    let mut result = 0;

    for number in splited_product {
        let number = match number.parse::<i32>() {
            Ok(number) => number,
            Err(_) => 0,
        };
        result += number
    }

    println!("result {}", format!("{}", *product.lock().unwrap()));
    println!("result {}", result);
}
