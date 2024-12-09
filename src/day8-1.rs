use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("./test.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let mut antennas: HashMap<&str, Vec<(i64, i64)>> = HashMap::new();
    let mut grid: Vec<Vec<&str>>= vec!();

    for line in text.lines() {
        let a: Vec<&str> = line.split("")
                            .filter(|s| !s.is_empty())
                            .collect();
        grid.push(a);
    }

    let grid_len = (grid.len() as i64 , grid[0].len() as i64);

    for i in 0..grid_len.0 as usize {
        for j in 0..grid_len.1 as usize {
            if grid[i][j] == "." { continue }
            antennas.entry(grid[i][j]).or_insert(vec!()).push((i.try_into().unwrap(), j.try_into().unwrap()));
        }
    }

    let mut anti_nodes: Vec<(i64, i64)> = vec!();

    antennas.iter().for_each(|antenna| {
        antenna.1.iter().for_each(|tower| {
            antenna.1.iter().for_each(|t| {
                let col = tower.0 - t.0;
                let line = tower.1 - t.1;
                let c = &(col, line);
                let mut q = tower.0 + c.0;
                let mut w = tower.1 + c.1;

                if col != 0 && line != 0 {
                    let node = (q , w);
                    if !anti_nodes.contains(&node) {
                        anti_nodes.push(node);
                    }
                }
            })
        })
    });


    let total = anti_nodes.into_iter().filter(|node| node.0 > -1 && node.0 < grid_len.0 && node.1 > -1 && node.1 < grid_len.1).collect::<Vec<(i64, i64)>>();

    println!("Result: {:?}", total.len());
}
