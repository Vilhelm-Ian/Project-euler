fn main() {
    let mut result = 0;
    for i in 1..1_000_000 {
        if is_double_base(i) {
            result += i;
        }
    }
    println!("{:?}", result);
}

fn is_palindrome<T: std::fmt::Display>(number: T) -> bool {
    let string = number.to_string();
    let chars: Vec<char> = string.chars().collect();
    for index in 0..chars.len() / 2 {
        if chars[index] != chars[chars.len() - index - 1] {
            return false;
        }
    }
    true
}

fn is_double_base(number: i32) -> bool {
    let binary = format!("{number:b}");
    is_palindrome(number) && is_palindrome(binary)
}
