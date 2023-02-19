fn main() {
    let mut incorrectly_cancelable = vec![];
    for a in 10..99 {
        for b in a + 1..99 {
            if a % 10 == 0 || b % 10 == 0 {
                continue;
            }
            if let Some(canceled) = incorrectly_cancel(a, b) {
                if can_incorrectly_cancel([a as f32, b as f32], canceled) {
                    incorrectly_cancelable.push([a, b])
                }
            }
        }
    }
    let mut product_of_cancealable_fractions = multiply_fractions(incorrectly_cancelable);
    let solution = reduce(&mut product_of_cancealable_fractions);
    println!("{:?}", solution);
}

fn incorrectly_cancel(a: i32, b: i32) -> Option<[f32; 2]> {
    let a: Vec<i32> = a.to_string().chars().map(|x| x as i32 - 48).collect();
    let b: Vec<i32> = b.to_string().chars().map(|x| x as i32 - 48).collect();
    let mut a_clone = a.clone();
    let mut b_clone = b.clone();
    for (a_index, a_digit) in a.iter().enumerate() {
        for (b_index, b_digit) in b.iter().enumerate() {
            if a_digit == b_digit {
                if a_clone.len() != a_index && b_clone.len() != b_index {
                    a_clone.remove(a_index);
                    b_clone.remove(b_index);
                };
            }
        }
    }
    if a_clone.len() == 2 {
        return None;
    }
    Some([a_clone[0] as f32, b_clone[0] as f32])
}

fn can_incorrectly_cancel(a: [f32; 2], b: [f32; 2]) -> bool {
    a[0] / a[1] == b[0] / b[1]
}

fn reduce(fraction: &mut [i32; 2]) -> [i32; 2] {
    for i in (2..=fraction[0]).rev() {
        if fraction[0] % i == 0 && fraction[1] % i == 0 {
            fraction[0] /= i;
            fraction[1] /= i;
        }
    }
    *fraction
}

fn multiply_fractions(fractions: Vec<[i32; 2]>) -> [i32; 2] {
    let mut result = [1, 1];
    for fraction in fractions {
        result[0] *= fraction[0];
        result[1] *= fraction[1];
    }
    result
}
