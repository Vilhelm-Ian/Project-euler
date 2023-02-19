fn main() {
    let mut digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut permutations: Vec<Vec<i32>> = vec![digits.clone()];
    let len = digits.len();
    heap_algoirthm(len as i32, &mut digits, &mut permutations);
    permutations.sort();
    println!("{:?}", permutations[1_000_000 - 1]);
}

fn heap_algoirthm(k: i32, digits: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) -> () {
    if k == 1 {
        return;
    }
    heap_algoirthm(k - 1, digits, result);
    for i in 0..k - 1 {
        if k % 2 == 0 {
            let i = i as usize;
            let k = (k - 1) as usize;
            (digits[i], digits[k]) = (digits[k], digits[i])
        } else {
            let k = (k - 1) as usize;
            (digits[0], digits[k]) = (digits[k], digits[0])
        }
        result.push(digits.clone());
        heap_algoirthm(k - 1, digits, result)
    }
}
