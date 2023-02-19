fn main() {
    let mut digit = String::from("");
    let mut i = 1;
    while digit.len() <= 1000000 {
        digit += &i.to_string();
        i += 1;
    }
    let digits: Vec<char> = digit.chars().collect();
    let mut result = 1;
    let indexes = [1, 10, 100, 1000, 10_000, 100_000, 1_000_000];
    for index in indexes {
        result *= digits[index - 1] as i32 - 48
    }
    println!("{:?}", result);
}
