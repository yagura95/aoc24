use std::io::{
    BufReader,
    BufRead
};
use std::fs::File;

fn main() {
    let file = File::open("day1-1.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut total = 0;
    let mut v1: Vec<i64> = vec!();
    let mut v2: Vec<i64> = vec!();

    for line in lines {
        let l = line.unwrap();
        let l = l.split_whitespace();
        let mut v: Vec<i64> = vec!();

        for i in l {
            v.push(i.parse::<i64>().unwrap());
        }
        
        v1.push(v[0]);
        v2.push(v[1]);
    }

    v1.sort();
    v2.sort();

    for i in 0..v1.len() {
        total = total + (v1[i] - v2[i]).abs();
    }

    println!("{}", total);
}
