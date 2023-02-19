mod variable;

fn main() {
    let numbers: Vec<i32> = variable::INPUT
        .iter()
        .map(|x| x.chars().fold(0, |acc, ele| acc + (ele as i32 - 64)))
        .collect();
    let max = numbers.iter().max().unwrap();
    let mut triangles = vec![];
    let mut num = 0;
    let mut i = 1;
    while &num < max {
        num = generate_triangle(i);
        triangles.push(num);
        i += 1;
    }
    let result: Vec<&i32> = numbers.iter().filter(|x| triangles.contains(x)).collect();
    println!("{:?}", result.len());
}

fn generate_triangle(n: i32) -> i32 {
    (n * (n + 1)) / 2
}
