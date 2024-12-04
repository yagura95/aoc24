use std::io::Read;
use std::fs::File;

// Search for A's
// Check if up left is M or S, check if down right is S(if up M) or M(if top S)
// Check if down left is M or S, check if up right is S(if up M) or M(if top S)

fn check_diag(a: &str, b: &str) -> bool {
    if a == "M" && b == "S" || a == "S" && b == "M" {
        return true;
    }

    return false;
}

// DONE
fn check(v: &Vec<Vec<&str>>, line: usize, column: usize) -> bool {
    if column > (v[line].len() - 2) || column < 1 || line < 1 || line > (v.len() - 2) {
        return false;
    }

    if v[line][column] == "A" &&
       check_diag(v[line-1][column-1], v[line+1][column+1]) && 
       check_diag(v[line-1][column+1], v[line+1][column-1]) {
           return true;
    } 

    return false;
}

fn main() {
    let mut file = File::open("day4-1.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    let mut grid: Vec<Vec<&str>> = vec!();
    let mut total = 0;

    for line in text.lines() {
        let l: Vec<&str> = line.split("").collect();
        grid.push(l[1..l.len()-1].to_vec());
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if check(&grid, i, j) {
                total += 1;
            }
        }
    }

    println!("Total: {}", total);
}
