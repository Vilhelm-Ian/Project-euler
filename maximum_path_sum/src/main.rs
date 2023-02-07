mod input;

fn main() {
    let result = solve(input::VARIABLE_2);
    println!("the result is {:?}", result)
}

fn parse(s: &str) -> Vec<Vec<i32>> {
    s.split('\n')
        .map(|line| {
            line.split(' ')
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve(s: &str) -> i32 {
    let mut parsed = parse(s);
    let mut y = parsed.len() - 1;
    while y > 0 {
        for x in 0..parsed[y - 1].len() {
            parsed[y - 1][x] += if parsed[y][x] > parsed[y][x + 1] {
                parsed[y][x]
            } else {
                parsed[y][x + 1]
            }
        }
        y -= 1;
    }
    parsed[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exampler() {
        let input = "3
7 4
2 4 6
8 5 9 3";
        let result = solve(input);
        let expected = 23;
        assert_eq!(result, expected);
    }
}
