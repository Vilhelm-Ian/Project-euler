fn main() {
    let mut result = 0;
    let palindromes_to_million = generate_palindromes_to_power(7);
    for palindrome in palindromes_to_million {
        let binary = format!("{palindrome:b}");
        if is_palindrome(binary) {
            result += palindrome
        }
    }
    println!("{:?}", result);
}

fn generate_palindromes_to_power(power: i32) -> Vec<i32> {
    let mut result = vec![];
    for i in 2..power {
        if i % 2 == 0 {
            let even_palindromes: Vec<i32> = generate_even_palindromes(i)
                .iter()
                .map(|x| x.parse().unwrap())
                .collect();
            result.extend(even_palindromes);
        } else {
            result.extend(generate_odd_palindromes(i));
        }
    }
    result
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

fn generate_odd_palindromes(power: i32) -> Vec<i32> {
    let even_palindromes = generate_even_palindromes(power - 1);
    let mut even_palindromes_converted: Vec<Vec<i32>> = even_palindromes
        .iter()
        .map(|x| number_to_vec(x.parse().unwrap()))
        .collect();
    even_palindromes_converted.remove(0);
    let mut result = vec![];
    for i in 0..10 {
        for palindrome in even_palindromes_converted.iter() {
            let mut palindrome_clone = palindrome.clone();
            palindrome_clone.insert(palindrome_clone.len() / 2, i);
            result.push(vec_to_number(&palindrome_clone));
        }
    }
    result
}

fn generate_even_palindromes(power: i32) -> Vec<String> {
    let mut result = vec![];
    for i in 0..10 {
        if power == 2 {
            result.push(format!("{}{}", i, i))
        } else {
            if i == 0 {
                continue;
            }
            let pallindromes = generate_even_palindromes(power - 2);
            for pallindrome in pallindromes {
                let pallindrome = format!("{}{}{}", i, pallindrome, i);
                result.push(pallindrome)
            }
        }
    }
    result
}

fn number_to_vec(number: i32) -> Vec<i32> {
    number.to_string().chars().map(|x| x as i32 - 48).collect()
}

fn vec_to_number(vector: &Vec<i32>) -> i32 {
    vector.iter().fold(0, |acc, elem| acc * 10 + elem)
}
