use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    let numbers = vec![1, 2, 5, 10, 20, 50, 100];
    let mut combinations = generate_all_combinations(200, &mut set);
    let result: Vec<Vec<i32>> = combinations
        .iter()
        .filter(|x| {
            for digit in *x {
                if !numbers.contains(digit) {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();
    println!("{:?}", result.len());
}

fn generate_sums_for_n(num: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    for i in 1..=num / 2 {
        result.push(vec![i, num - i]);
    }
    result
}

fn generate_all_combinations(number: i32, set: &mut HashSet<Vec<i32>>) -> Vec<Vec<i32>> {
    let combinations = generate_sums_for_n(number);
    let mut results = combinations.clone();
    for combination in combinations.iter() {
        for index in 0..combination.len() {
            let mut combination_clone = combination.clone();
            let removed = combination_clone.remove(index);
            let new_combinations = generate_all_combinations(removed, set);
            for new_combination in new_combinations {
                let mut combined = combine_two_vectors(&combination_clone, &new_combination);
                combined.sort();
                if set.get(&combined).is_none() {
                    set.insert(combined.clone());
                    results.push(combined)
                }
            }
        }
    }
    results
}

fn combine_two_vectors(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    let mut result = a.clone();
    for element in b {
        result.push(*element)
    }
    result
}
