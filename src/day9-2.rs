use std::io::Read;
use std::fs::File;

#[derive(Clone, Debug)]
struct MemorySegment {
    index: u64,
    count: u64,
    mem_type: u64,
}

fn main() {
    let mut file = File::open("./day9-1.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    // (index, count, type)
    let mut memory: Vec<MemorySegment> = vec!();

    let mut index: u64 = 0;

    for c in text.chars().enumerate() {
        if !c.1.is_alphanumeric() {
            continue;
        }
        
        let count = c.1.to_string().parse::<u64>().unwrap();

        if c.0 % 2 == 0 {
            memory.push(MemorySegment { index, count, mem_type: 0 });
            index += 1;
        } else {
            memory.push(MemorySegment { index, count, mem_type: 1 });
        }    
    }
    
    fn search(index: usize, memory: &mut Vec<MemorySegment>, count: u64) -> Option<usize>{
        if count < 1 {
            return None
        }

        for i in (index..memory.len()).rev() {
            if memory[i].mem_type == 1 {
                continue
            }

            if memory[i].count <= count {
                return Some(i);
            }
        }

        None
    }

    let mut m: Vec<MemorySegment> = vec!();

    for i in 0..memory.len() {
        if memory[i].mem_type == 0 {
            m.push(memory[i].clone());
            continue;
        }
        
        let mut count = memory[i].count;

        while let Some(index) = search(i, &mut memory, count) {
            m.push(memory[index].clone());

            count -= memory[index].count;
            memory[index].mem_type = 1;
        }

        if count != 0 {
            m.push(MemorySegment { index: 0, count, mem_type: 1 });
        }
    }

    let mut v: Vec<u64> = vec!();


    for i in 0..m.len() {
        for _j in 0..m[i].count {
            v.push(m[i].index);
        }
    }

    let mut total = 0;

    for i in 0..v.len() {
        total += v[i] * i as u64;
    }

    println!("Total: {:?}", total);
    
}
