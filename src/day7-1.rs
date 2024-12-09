use std::io::{
    Read
};
use std::fs::File;

fn main() {
    let mut file = File::open("./day7-1.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let mut total = 0;

    fn check_solution(numbers: Vec<u64>, result: u64) -> bool {
        let mut solutions: Vec<u64> = vec!();
        
        solutions.push(numbers[0] + numbers[1]);
        solutions.push(numbers[0] * numbers[1]);

        for i in 2..numbers.len() {
            for _j in 0..solutions.len() {
                let n = solutions.remove(0);
                solutions.push(numbers[i] + n);
                solutions.push(numbers[i] * n);
            }
        }

        solutions.contains(&result)
    }

    for line in text.lines() {
        let params: Vec<&str> = line.split_whitespace().collect();

        let mut result = params[0].to_string();
        result.remove(result.len() - 1);

        let result = result.parse::<u64>().unwrap();
        let numbers: Vec<u64> = params[1..].into_iter().map(|s| s.parse::<u64>().unwrap()).collect();

        let found = check_solution(numbers, result);

        if found {
            total += result;
        }
    }

    println!("Result: {}", total);
}
