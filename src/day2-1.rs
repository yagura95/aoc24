use std::io::{
    BufReader,
    BufRead
};
use std::fs::File;

fn main() {
    let file = File::open("day2-1.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut total = 0;

    for line in lines {
        let l = line.unwrap();
        let l = l.split_whitespace();
        let mut v: Vec<i32> = vec!();

        for i in l {
            v.push(i.parse::<i32>().unwrap());
        }

        // 0 for negative (3,2,1), 1 for positive(1,2,3)
        let mut seq = 0;
        let mut safe = 1;


        for i in 0..v.len() - 1 {
            if i == 0 {
                if v[i] < v[i+1] {
                    seq = 1;
                }             
            } 

            if (v[i] - v[i+1]).abs() > 3 {
                safe = 0;
            } else if (v[i] - v[i+1]).abs() < 1 {
                safe = 0;
            } else if v[i] < v[i+1] && seq == 0 {
                safe = 0;
            } else if v[i] > v[i+1] && seq == 1 {
                safe = 0;
            }
        }

        if safe == 1 {
            total += 1;
        }
    }

    println!("{}", total);
}
