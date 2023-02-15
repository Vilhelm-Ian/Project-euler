fn main() {
    let mut diagonal = 1;
    let mut adder = 2;
    let mut result = 1;
    for _ in 0..1001 / 2 {
        for _ in 0..4 {
            diagonal += adder;
            result += diagonal;
        }
        adder += 2
    }
    println!("Hello, world! {:?}", result);
}
