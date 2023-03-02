use std::collections::HashSet;

fn main() {
    find_solution()
}
fn find_solution() {
    for i in 1..=6 {
        for a in 10 * 10_i32.pow(i as u32)..17 * 10_i32.pow(i as u32) {
            let a_vec = num_to_vec(a);
            if a_vec.contains(&0) {
                continue;
            }
            for z in 2..=6 {
                let b = a * z;
                let b_vec = num_to_vec(b);
                if !is_permutation(&a_vec, &b_vec) {
                    break;
                } else if z == 6 {
                    println!("{:?}", a);
                    return;
                }
            }
        }
    }
}

fn num_to_vec(num: i32) -> Vec<i32> {
    num.to_string().chars().map(|x| x as i32 - 48).collect()
}

fn is_permutation(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    let mut a_set = HashSet::new();
    let mut b_set = HashSet::new();
    for index in 0..a.len() {
        if a[index] == b[index] {
            return false;
        }
        a_set.insert(a[index]);
        b_set.insert(b[index]);
    }
    let intersection: Vec<&i32> = a_set.intersection(&b_set).collect();
    intersection.len() == a.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perumtaion() {
        let result = is_permutation(&vec![1, 2, 5, 8, 7, 4], &vec![2, 5, 1, 7, 4, 8]);
        assert!(result);
    }
    #[test]
    fn not_perumtaion() {
        let result = is_permutation(&vec![1, 2, 5, 8, 7, 5], &vec![2, 5, 1, 7, 4, 8]);
        assert!(!result);
    }
}
