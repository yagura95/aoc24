use std::io::Read;
use std::fs::File;

const TURNS: i32 = 25;

fn main() {
    let mut file = File::open("./test.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    
    let stones: Vec<u64> = text.split_whitespace().filter(|s| !s.is_empty()).map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let mut total = 0;

    fn apply_rules(stones: Vec<u64>) -> Vec<u64> { 
        let mut v: Vec<u64> = vec!();

        for stone in stones {
            if stone == 0 {
                v.push(1);
                continue
            } 

            let s = stone.to_string();
            let l = s.len();
    
            if l % 2 == 0 {
               let first = s[0..l/2].to_owned();
               let second = s[l/2..].to_owned();
                
               v.push(first.parse::<u64>().unwrap());
               v.push(second.parse::<u64>().unwrap());
               continue
            }        

            v.push(stone.to_owned()*2024);
        }

        return v;
    }

    for stone in stones {
        let mut v: Vec<u64> = vec!(stone);
        for _i in 0..TURNS {
            v = apply_rules(v);
            println!("TURN {} - {:?}", _i, v.len());
        }

        total += v.len();
    }

    println!("Total: {}", total);
}
