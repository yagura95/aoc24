use std::io::Read;
use regex::Regex;

use std::fs::File;

fn main() {
    let mut file = File::open("day3-1.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap(); 
    let re2 = Regex::new(r"\d+").unwrap();

    let m: Vec<_> = re.find_iter(&text).filter_map(|mul| {
        return Some(mul.as_str());
    }).map(|dig| {
        let n: Vec<_> = re2.find_iter(dig).filter_map(|dig| {
            return Some(dig.as_str());
        }).collect();

        return n[0].parse::<i64>().unwrap() * n[1].parse::<i64>().unwrap();
    }).collect();

    let r: i64 = m.iter().sum();

    println!("{}", r);

}
