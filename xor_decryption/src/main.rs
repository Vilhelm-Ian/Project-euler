use std::collections::HashMap;
mod variable;

fn main() {
    let lower_case: Vec<u8> = (97..=122).collect();
    let mut passwords: Vec<[u8; 3]> = vec![];
    for a in lower_case.iter() {
        for b in lower_case.iter() {
            for c in lower_case.iter() {
                passwords.push([*a, *b, *c])
            }
        }
    }
    for solution in passwords {
        let message = decrypt(solution, variable::MESSAGE);
        let message = message
            .iter()
            .fold(String::from(""), |acc, ele| format!("{}{}", acc, ele));
        let most_common = find_most_common_word(&message);
        if most_common == String::from("the") {
            println!("{:?}", message);
            let solution: i32 = message.chars().map(|x| x as i32).sum();
            println!("{:?}", solution);
            break;
        }
    }
}

fn decrypt(password: [u8; 3], message: [u8; 1455]) -> Vec<char> {
    let mut i = 0;
    let mut solution = vec![];
    for byte in message {
        solution.push((byte ^ password[i % 3]) as char);
        i += 1;
    }
    solution
}

fn find_most_common_word(words: &String) -> String {
    let mut map = HashMap::new();

    for word in words.split(' ') {
        let entry = map.entry(word).or_insert(0);
        *entry += 1;
    }
    let mut keys_values = vec![];
    for (key, value) in map {
        keys_values.push((key, value))
    }
    keys_values.sort_by(|a, b| b.1.cmp(&a.1));
    keys_values[0].0.to_string()
}
