use std::collections::HashSet;

#[derive(Debug)]
struct Logarithm {
    base: i32,
    argument: i32,
    log: i32,
}

impl Logarithm {
    fn new(base: i32, argument: i32, log: i32) -> Self {
        Self {
            base,
            argument,
            log,
        }
    }
}

fn main() {
    let mut unique_powers = HashSet::new();
    let b = 100;
    let a_limit = 100;
    let logs = find_logs(a_limit);
    let not_logs = find_non_logs(a_limit, &logs);
    for not_log in not_logs {
        for i in 2..=b {
            unique_powers.insert([not_log, i]);
        }
    }
    for log in logs {
        for i in 2..=b {
            unique_powers.insert([log.base, log.log * i]);
        }
    }

    println!("{:?}", unique_powers.len());
}

fn find_non_logs(limit: i32, logs: &[Logarithm]) -> Vec<i32> {
    let mut result = vec![];
    for number in 2..=limit {
        let mut is_not_log = true;
        for log in logs.iter() {
            if number == log.argument {
                is_not_log = false;
                break;
            }
        }
        if is_not_log {
            result.push(number)
        }
    }
    result
}

fn find_logs(limit: i32) -> Vec<Logarithm> {
    let mut result = vec![];
    for i in 2..=limit {
        for number in 2..i {
            let log = (i as f32).log(number as f32);
            if log.fract() == 0.0 {
                result.push(Logarithm::new(number, i, log as i32));
                break;
            }
        }
    }
    result
}
