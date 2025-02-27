use std::io::Read;
use std::fs::File;

type MapPosition = (usize, usize);

#[derive(Debug)]
struct Path {
    position: MapPosition,
    height: i64,
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.position.0 == other.position.0 && self.position.1 == other.position.1 
    }
}

fn main() {
    let mut file = File::open("./day10-1.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    
    let mut map: Vec<Vec<i64>> = vec!();

    for line in text.lines() {
        let t = line.split("").filter(|s| !s.is_empty()).map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        println!("{:?}", line);
        map.push(t);
    }

    fn walk(starting: Path, map: &Vec<Vec<i64>>, trailheads: &mut Vec<Path>) -> u64 {
        if starting.height == 9 && !trailheads.contains(&starting) {
            trailheads.push(starting);
            return 1;
        }

        let (i, j) = starting.position;

        let mut total: u64 = 0;
    
        // top 
        if i > 0 && (map[i - 1][j] - map[i][j]) == 1  { 
            total += walk(Path { position: (i - 1, j), height: map[i - 1][j] }, map, trailheads);
        }

        // bottom 
        if (i < (map.len() - 1)) && (map[i + 1][j] - map[i][j]) == 1  { 
            total += walk(Path { position: (i + 1, j), height: map[i + 1][j] }, map, trailheads);
        }

        // left
        if j > 0 && (map[i][j - 1] - map[i][j]) == 1  { 
            total += walk(Path { position: (i, j - 1), height: map[i][j - 1] }, map, trailheads);
        }

        // right
        if (j < (map[0].len() - 1)) && (map[i][j + 1] - map[i][j]) == 1  { 
            total += walk(Path { position: (i, j + 1), height: map[i][j + 1] }, map, trailheads);
        }

        return total;
    }

    let mut total = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                let mut trailheads: Vec<Path> = vec!();
                walk(Path { position: (i, j), height: map[i][j] }, &map, &mut trailheads); 
                total += trailheads.len();
            }
        }
    }

    println!("Total: {}", total);

}
