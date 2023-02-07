mod input;

fn main() {
    let mut cloned_variable = input::VARIABLE.clone();
    cloned_variable.sort();
    let mut result = 0;
    for (iter, word) in cloned_variable.iter().enumerate() {
        let mut name_score = 0;
        for character in word.chars() {
            name_score += character as u32 - 64;
        }
        result += name_score * (iter as u32 + 1);
    }
    println!("Hello, world! {:?}", result);
}
