use std::io::Read;
use std::fs::File;

fn main() {
    let mut file = File::open("./day9-1.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let mut memory: Vec<String> = vec!();

    let mut index = 0;

    for c in text.chars().enumerate() {
        if !c.1.is_alphanumeric() {
            continue;
        }
        
        let n = c.1.to_string().parse::<u64>().unwrap();
        for _j in 0..n {
            if c.0 % 2 == 0 {
                memory.push(index.to_string());
            } else {
                memory.push(".".to_string());
            }    
        }

        if c.0 % 2 == 0 {
            index += 1;
        } 
    }

    let mut first: usize = 0;
    let mut last: usize = memory.len() - 1;

    while first < last {
        if memory[first] == "." {
            while memory[last] == "." {
                last -= 1;
            }

            let tmp = String::from(&memory[first]);
            memory[first] = String::from(&memory[last]);
            memory[last] = tmp;
            last -= 1;
        } else {
            first += 1;
        }
    }


    let m: Vec<u32>  = memory.iter().filter(|s| *s != &".".to_string()).map(|s| s.parse::<u32>().unwrap()).collect();

    let mut total = 0;

    for n in m.iter().enumerate() {
        total += n.0 * *n.1 as usize;
    }

    println!("Total: {}", total);
}
