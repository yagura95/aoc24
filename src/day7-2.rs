use std::io::{
    Read
};
use std::fs::File;

fn main() {
    let mut file = File::open("./day7-1.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let mut total = 0;

    fn concat(i: u64, j: u64) -> u64 {
        let mut a = i.to_string();
        let b = j.to_string();
       
        a.push_str(b.as_str());
        a.parse::<u64>().unwrap()
    }

    fn check_solution(numbers: Vec<u64>, result: u64) -> bool {
        let mut solutions: Vec<u64> = vec!();
        
        solutions.push(numbers[0] + numbers[1]);
        solutions.push(numbers[0] * numbers[1]);
        solutions.push(concat(numbers[0], numbers[1]));

        for i in 2..numbers.len() {
            let mut v: Vec<u64> = vec!();
            for _j in 0..solutions.len() {
                let n = solutions.remove(0);
                v.push(numbers[i] + n);
                v.push(numbers[i] * n);
                v.push(concat(n, numbers[i]));
            }
            solutions = v;
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
