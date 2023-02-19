fn main() {
    let mut result = vec![];
    let mut i = 285;
    while result.len() != 3 {
        let triangle = generate_triangle(i);
        if is_pentagonal(triangle) && is_hexagonal(triangle) {
            result.push(triangle);
            println!("{:?}", triangle);
        }
        i += 1;
    }
    println!("{:?}", result);
}

fn generate_triangle(n: u128) -> u128 {
    n * (n + 1) / 2
}

fn is_pentagonal(c: u128) -> bool {
    let a = 3.0;
    let b = -1.0;
    let c = c as f64 * -2.0;
    let result = quadratic_formula(a, b, c);
    quadratic_formula(a, b, c).fract() == 0_f64
}

fn is_hexagonal(c: u128) -> bool {
    let a = 2.0;
    let b = -1.0;
    let c = c as f64 * -1.0;
    quadratic_formula(a, b, c).fract() == 0_f64
}

fn quadratic_formula(a: f64, b: f64, c: f64) -> f64 {
    (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a)
}
