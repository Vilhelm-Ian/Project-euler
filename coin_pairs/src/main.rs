fn main() {
    let coins = vec![1, 2, 5, 10, 20, 50, 100];
    let mut result = vec![];
    for (mut i, coin) in coins.iter().enumerate() {
        let mut temp = vec![];
        let len = coins.len();
        let mut z = i;
        while z < len - 1 {
            let len = temp.len();
            if temp.iter().sum::<i32>() + coins[z] <= 200 {
                temp.push(coins[z]);
            } else {
                z += 1;
                if z == 7 {
                    break;
                }
                while temp.iter().sum::<i32>() + coins[z] > 200 {
                    temp.remove(0);
                }
            }
            if temp.iter().sum::<i32>() == 200 {
                result.push(temp.clone());
            }
        }
    }
    println!("{:?}", result);
}
