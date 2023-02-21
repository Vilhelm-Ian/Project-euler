fn main() {
    let mut result = 0;
    let mut max_length = 0;
    for i in 5..=1000 {
        let solutions = find_solutions_for_triangle_perimeter(i);
        if solutions.len() > max_length {
            max_length = solutions.len();
            result = i;
        }
    }
    println!("{:?}", result);
}

fn find_solutions_for_triangle_perimeter(perimeter: i32) -> Vec<[i32; 3]> {
    let mut solutions = vec![];
    for a in 1..perimeter {
        for b in a..perimeter {
            let c = pythagorean_theorem(a, b);
            if let Some(c) = c {
                let calculated_perimeter = a + b + c;
                if calculated_perimeter == perimeter {
                    solutions.push([a, b, c])
                }
                if calculated_perimeter > perimeter {
                    break;
                }
            };
        }
    }
    solutions
}

fn pythagorean_theorem(a: i32, b: i32) -> Option<i32> {
    let solution = ((a * a + b * b) as f32).sqrt();
    if solution.fract() == 0_f32 {
        return Some(solution as i32);
    }
    None
}
