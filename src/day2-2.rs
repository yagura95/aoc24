use std::io::{
    BufReader,
    BufRead
};

use std::fs::File;

fn level_correct(v1: i64, v2: i64, seq: u8) -> bool {
    if (v1 - v2).abs() > 3 {
        return false;
    } else if (v1 - v2).abs() < 1 {
        return false;
    } else if v1 < v2 && seq == 0 {
        return false;
    } else if v1 > v2 && seq == 1 {
        return false;
    }

    return true;
}

fn check_if_safe(v: &mut Vec<i64>, recursion_level: u8) -> i64 {
    if recursion_level > 1 {
        return 0;
    };

    // 0 for negative (3,2,1), 1 for positive(1,2,3)
    let mut seq = 0;

    for i in 0..v.len() - 1 {
        if i == 0 {
            if v[i] < v[i+1] {
                seq = 1;
            }             
        } 

        if !level_correct(v[i], v[i+1], seq) {
            let n1 = &mut v.clone();
            let n2 = &mut v.clone();
            let n3 = &mut v.clone();

            n1.remove(i);
            n2.remove(i+1);
            n3.remove(0);

            // Check if correct if wrong level removed
            if check_if_safe(n1, recursion_level+1) != 0 {
                return 1;
            } 

            if check_if_safe(n2, recursion_level+1) != 0 {
                return 1;
            } 

            if check_if_safe(n3, recursion_level+1) != 0 {
                return 1;
            } 

            return 0;
        }
    }

    return 1;
}

fn main() {
    let file = File::open("day2-2.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut total = 0;

    for line in lines {
        let l = line.unwrap();
        let l = l.split_whitespace();
        let mut v: Vec<i64> = vec!();

        for i in l {
            v.push(i.parse::<i64>().unwrap());
        }


        let n = check_if_safe(&mut v, 0);
        total += n;
    }

    println!("{}", total);
}
