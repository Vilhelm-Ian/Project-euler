use std::collections::HashMap;

fn main() {
    let mut map: HashMap<[i32; 10], Vec<u128>> = HashMap::new();
    for i in 345..10_000 {
        let mut nums = [0; 10];
        let num = i * i * i;
        let converte_num = number_to_vec(&num);
        for num in converte_num {
            nums[num as usize] += 1;
        }
        map.entry(nums).or_default().push(num);
    }
    let values: Vec<Vec<u128>> = map.into_values().collect();
    let solutions: Vec<Vec<u128>> = values.iter().filter(|x| x.len() == 5).cloned().collect();
    let solutions: Vec<&u128> = solutions.iter().map(|x| x.iter().min().unwrap()).collect();
    println!("{:?}", solutions.iter().min());
}

fn number_to_vec(number: &u128) -> Vec<u128> {
    number.to_string().chars().map(|x| x as u128 - 48).collect()
}
