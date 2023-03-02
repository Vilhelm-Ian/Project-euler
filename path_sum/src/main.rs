mod vairable;

struct Cordinates {
    x: usize,
    y: usize,
}

fn main() {
    let mut matrix = vec![
        vec![131, 673, 234, 103, 18],
        vec![201, 96, 342, 965, 150],
        vec![603, 803, 746, 422, 111],
        vec![805, 732, 524, 37, 331],
    ];
    solve(&mut matrix);
    for row in matrix {
        println!("{:?}", row);
    }
}

fn solve(matrix: &mut Vec<Vec<i32>>) {
    let original_matrix = matrix.clone();
    let mut cordinates = Cordinates { x: 0, y: 0 };
    let mut result = original_matrix[cordinates.y][cordinates.y];
    add_edges(matrix);
    let rows = matrix.len();
    let collums = matrix[0].len();
    while cordinates.x < collums - 1 && cordinates.y < rows - 1 {
        if matrix[cordinates.y + 1][cordinates.x] < matrix[cordinates.y][cordinates.x + 1] {
            cordinates.y += 1;
            sum_left(matrix, &cordinates);
        } else {
            cordinates.x += 1;
            sum_up(matrix, &cordinates);
        }
        println!("{:?}", matrix);
        println!("{:?}", result);
        result += original_matrix[cordinates.y][cordinates.x];
        ()
    }
    println!("{:?}", result);
}

fn add_edges(matrix: &mut Vec<Vec<i32>>) {
    let rows = matrix.len();
    let collums = matrix[0].len();
    let mut y = rows as i32 - 2;
    let mut x = collums as i32 - 2;
    while y != -1 || x != -1 {
        if x != -1 {
            matrix[rows - 1][x as usize] += matrix[rows - 1][x as usize + 1];
            x -= 1;
        }
        if y != -1 {
            matrix[y as usize][collums - 1] += matrix[y as usize + 1][collums - 1];
            y -= 1;
        }
    }
    let mut y = rows - 2;
    let mut x = collums - 2;
    while y != 0 || x != 0 {
        if x != 0 {
            matrix[0][x] += matrix[0][x + 1];
            x -= 1;
        }
        if y != 0 {
            matrix[y][0] += matrix[y + 1][0];
            y -= 1;
        }
    }
}

fn sum_up(matrix: &mut Vec<Vec<i32>>, cordinates: &Cordinates) {
    let len = matrix.len();
    for y in (cordinates.y + 2..len).rev() {
        matrix[y as usize - 1][cordinates.x] += matrix[y][cordinates.x];
    }
}

fn sum_left(matrix: &mut Vec<Vec<i32>>, cordinates: &Cordinates) {
    let len = matrix[0].len();
    for x in (cordinates.x + 2..len).rev() {
        matrix[cordinates.y][x - 1] += matrix[cordinates.y][x];
    }
}
