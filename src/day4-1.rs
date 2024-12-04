use std::io::Read;
use std::fs::File;

// DONE
fn check_right(v: &Vec<Vec<&str>>, line: usize, column: usize) -> bool {
    if column > (v[line].len() - 4) {
        return false;
    }

    if v[line][column]   == "X" && 
       v[line][column+1] == "M" && 
       v[line][column+2] == "A" && 
       v[line][column+3] == "S" {
        return true;
    }

    return false;
}

// DONE
fn check_left(v: &Vec<Vec<&str>>, line: usize, column: usize) -> bool {
    if column < 3 {
        return false;
    }

    if v[line][column]   == "X" && 
       v[line][column-1] == "M" && 
       v[line][column-2] == "A" && 
       v[line][column-3] == "S" {
        return true;
    }

    return false;
}

// DONE
fn check_up(v: &Vec<Vec<&str>>, line: usize, column: usize) -> bool {
    if line < 3 {
        return false;
    }

    if v[line][column]   == "X" && 
       v[line-1][column] == "M" && 
       v[line-2][column] == "A" && 
       v[line-3][column] == "S" {
        return true;
    }

    return false;
}

// DONE
fn check_down(v: &Vec<Vec<&str>>, line: usize, column: usize) -> bool {
    if line > (v.len() - 4) {
        return false;
    }

    if v[line][column]   == "X" && 
       v[line+1][column] == "M" && 
       v[line+2][column] == "A" && 
       v[line+3][column] == "S" {
        return true;
    }

    return false;
}

// DONE
fn check_diag_up_right(v: &Vec<Vec<&str>>, line: usize, column: usize) -> bool {
    if line < 3 || column > (v[line].len() - 4) {
        return false;
    }

    if v[line][column]   == "X" && 
       v[line-1][column+1] == "M" && 
       v[line-2][column+2] == "A" && 
       v[line-3][column+3] == "S" {
        return true;
    }

    return false;
}

// DONE
fn check_diag_up_left(v: &Vec<Vec<&str>>, line: usize, column: usize) -> bool {
    if line < 3 || column < 3 {
        return false;
    }

    if v[line][column]   == "X" && 
       v[line-1][column-1] == "M" && 
       v[line-2][column-2] == "A" && 
       v[line-3][column-3] == "S" {
        return true;
    }

    return false;
}

fn check_diag_down_right(v: &Vec<Vec<&str>>, line: usize, column: usize) -> bool {
    if line > (v.len() - 4) || column > (v[line].len() - 4) {
        return false;
    }

    if v[line][column]   == "X" && 
       v[line+1][column+1] == "M" && 
       v[line+2][column+2] == "A" && 
       v[line+3][column+3] == "S" {
        return true;
    }

    return false;
}

fn check_diag_down_left(v: &Vec<Vec<&str>>, line: usize, column: usize) -> bool {
    if line > (v.len() - 4) || column < 3 {
        return false;
    }

    if v[line][column]   == "X" && 
       v[line+1][column-1] == "M" && 
       v[line+2][column-2] == "A" && 
       v[line+3][column-3] == "S" {
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
            if check_right(&grid, i, j) {
                total += 1;
            }

            if check_left(&grid, i, j) {
                total += 1;
            }

            if check_up(&grid, i, j) {
                total += 1;
            }

            if check_down(&grid, i, j) {
                total += 1;
            }

            if check_diag_up_right(&grid, i, j) {
                total += 1;
            }

            if check_diag_up_left(&grid, i, j) {
                total += 1;
            }

            if check_diag_down_right(&grid, i, j) {
                total += 1;
            }

            if check_diag_down_left(&grid, i, j) {
                total += 1;
            }
        }
    }

    println!("Total: {}", total);
}
