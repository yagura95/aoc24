use std::io::{
    BufReader,
    BufRead
};
use std::collections::HashMap;
use std::fs::File;

fn main() {
    let file = File::open("day1-2.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut total = 0;
    let mut v1: Vec<u64> = vec!();
    let mut v2: HashMap<u64, u64> = HashMap::new();

    for line in lines {
        let l = line.unwrap();
        let l = l.split_whitespace();
        let mut v: Vec<u64> = vec!();

        for i in l {
            v.push(i.parse::<u64>().unwrap());
        }
        
        v1.push(v[0]);
        *v2.entry(v[1] as u64).or_insert(0) += 1;
    }

    for i in 0..v1.len() {
        total = total + v1[i] * (*v2.entry(v1[i]).or_insert(0)) ;
    }

    println!("{}", total);
}
