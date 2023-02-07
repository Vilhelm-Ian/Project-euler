use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

fn main() {
    let map = HashMap::from([
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
        (100, "hundred"),
    ]);
    let map1 = Arc::new(map);
    let mut result = 0;
    let mut handles = vec![];
    for i in 0..10 {
        let map2 = map1.clone();
        let handle = thread::spawn(move || {
            let mut result = 0;
            for number in (100 * i) + 1..=100 * (i + 1) {
                result += get_name(number, &map2).unwrap().len();
            }
            result
        });
        handles.push(handle)
    }
    for handle in handles {
        result += handle.join().unwrap();
    }
    println!("{:?}", result);
}

fn get_name(number: i32, map: &HashMap<i32, &str>) -> Option<String> {
    if number == 1000 {
        return Some(String::from("onethousand"));
    }
    if number > 99 {
        let result = format!("{}{}", map.get(&(number / 100)).unwrap(), "hundred");
        return match get_name(number % 100, map) {
            Some(name) => Some(format!("{}and{}", result, name)),
            None => Some(result),
        };
    }
    if number < 100 && number > 0 {
        if let Some(name) = map.get(&number) {
            return Some(String::from(*name));
        }
        let two_digit = map.get(&(number - number % 10)).unwrap();
        let single_digit = map.get(&(number % 10)).unwrap();
        return Some(format!("{}{}", two_digit, single_digit));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_hundred_and_forty_two() {
        let map: HashMap<i32, &str> = HashMap::from([
            (1, "one"),
            (2, "two"),
            (3, "three"),
            (4, "four"),
            (5, "five"),
            (6, "six"),
            (7, "seven"),
            (8, "eight"),
            (9, "nine"),
            (10, "ten"),
            (11, "eleven"),
            (12, "twelve"),
            (13, "thirteen"),
            (14, "fourteen"),
            (15, "fifteen"),
            (16, "sixteen"),
            (17, "seventeen"),
            (18, "eighteen"),
            (19, "ninetteen"),
            (20, "twenty"),
            (30, "thirty"),
            (40, "forty"),
            (50, "fifty"),
            (60, "sixty"),
            (70, "seventy"),
            (80, "eighty"),
            (90, "ninety"),
            (100, "hundred"),
        ]);
        let result = get_name(342, &map).unwrap();
        let expected = 23;
        assert_eq!(result.len(), expected);
    }
    #[test]
    fn hundred_and_fifteen() {
        let map: HashMap<i32, &str> = HashMap::from([
            (1, "one"),
            (2, "two"),
            (3, "three"),
            (4, "four"),
            (5, "five"),
            (6, "six"),
            (7, "seven"),
            (8, "eight"),
            (9, "nine"),
            (10, "ten"),
            (11, "eleven"),
            (12, "twelve"),
            (13, "thirteen"),
            (14, "fourteen"),
            (15, "fifteen"),
            (16, "sixteen"),
            (17, "seventeen"),
            (18, "eighteen"),
            (19, "ninetteen"),
            (20, "twenty"),
            (30, "thirty"),
            (40, "forty"),
            (50, "fifty"),
            (60, "sixty"),
            (70, "seventy"),
            (80, "eighty"),
            (90, "ninety"),
            (100, "hundred"),
        ]);
        let result = get_name(115, &map).unwrap();
        let expected = 20;
        assert_eq!(result.len(), expected);
    }
}
