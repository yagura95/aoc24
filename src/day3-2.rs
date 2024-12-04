use std::io::Read;
use regex::Regex;

use std::fs::File;

fn main() {
    let mut file = File::open("day3-1.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let re = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap(); 
    let re2 = Regex::new(r"\d+").unwrap();

    let hay = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let m: Vec<_> = re.find_iter(&text).filter_map(|mul| {
        return Some(mul.as_str());
    }).collect();

    let mut active = true;
    let mut total = 0;

    for item in m {
        match item {
            "don't()" => {
                println!("Deactivated");
                active = false;
            },
            "do()" => {
                println!("Activated");
                active = true;
            },
            _ => {
                if !active { continue };

                let m: Vec<_> = re2.find_iter(&item).filter_map(|mul| {
                    return Some(mul.as_str());
                }).collect();

                total += m[0].parse::<i64>().unwrap() * m[1].parse::<i64>().unwrap();
            },
        }
    }

    println!("Total: {}", total);
}
