const LIMIT: i32 = 200;

fn main() {
    let mut permutations = vec![];
    let coins = vec![1, 2, 5];
    for a in (0..=200) {
        for b in 0..=100 {
            if a + b * 2 > 200 {
                break;
            }
            for c in (0..=40) {
                if a + b * 2 + c * 5 > 200 {
                    break;
                }
                for d in (0..=20) {
                    if a + b * 2 + c * 5 + d * 10 > 200 {
                        break;
                    }
                    for e in (0..=10) {
                        if a + b * 2 + c * 5 + d * 10 + e * 20 > 200 {
                            break;
                        }
                        for f in (0..=4) {
                            if a + b * 2 + c * 5 + d * 10 + e * 20 + f * 50 > 200 {
                                break;
                            }
                            for g in (0..=2) {
                                if a + b * 2 + c * 5 + d * 10 + e * 20 + f * 50 + g * 100 > 200 {
                                    break;
                                }
                                for j in (0..=1) {
                                    if a + b * 2
                                        + c * 5
                                        + d * 10
                                        + e * 20
                                        + f * 50
                                        + g * 100
                                        + j * 200
                                        > 200
                                    {
                                        break;
                                    }
                                    permutations.push([a, b, c, d, e, f, g, j])
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let solution: Vec<[i32; 8]> = permutations
        .iter()
        .filter(|x| {
            x[0] * 1
                + x[1] * 2
                + x[2] * 5
                + x[3] * 10
                + x[4] * 20
                + x[5] * 50
                + x[6] * 100
                + x[7] * 200
                == 200
        })
        .cloned()
        .collect();
    for sl in solution.iter() {
        println!("{:?}", sl);
    }
    println!("{:?}", solution.len());
}
