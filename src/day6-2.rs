use std::io::{
    Read,
    Write,
    stdout,
};
use std::time::Duration;
use std::thread::sleep;
use std::fs::File;
use std::fmt::Debug;
use crossterm::cursor::{MoveUp, MoveDown};
use crossterm::ExecutableCommand;

#[derive(Debug)]
enum Direction {
    TOP,
    RIGHT,
    DOWN,
    LEFT
}

#[derive(Debug)]
struct Guard {
    coords: (usize, usize), // (x,y)
    direction: Direction,
}

impl Guard {
    fn new(grid: &Vec<Vec<&str>>) -> Guard {
        let l = grid.len();

        for i in 0..l-1 {
            for j in 0..l-1 {
                let dir = get_direction(grid[i][j]);
                
                if dir.is_none() {
                    continue
                }

                let guard = Guard {
                    coords: (j, i),
                    direction: dir.unwrap(),
                };

                println!("Guard: {guard:?}");

                return guard
            }    
        }

        panic!("No guard found")
    }

    fn walk(&mut self) {
        match self.direction {
            Direction::TOP   => self.coords = (self.coords.0, self.coords.1 - 1),
            Direction::RIGHT => self.coords = (self.coords.0 + 1, self.coords.1),
            Direction::DOWN  => self.coords = (self.coords.0, self.coords.1 + 1),
            Direction::LEFT  => self.coords = (self.coords.0 - 1, self.coords.1),
        } 


    }

    fn turn(&mut self) {
        return match self.direction {
            Direction::TOP   => self.direction = Direction::RIGHT,
            Direction::RIGHT => self.direction = Direction::DOWN,
            Direction::DOWN  => self.direction = Direction::LEFT,
            Direction::LEFT  => self.direction = Direction::TOP,
        }
    }

    fn check_next_step(&mut self, grid: &Vec<Vec<&str>>, grid_len: &(usize, usize)) -> bool {
        if self.check_if_exit(grid_len) {
            println!("EXIT");
            return false; 
        }

        let coords = match self.direction {
            Direction::TOP   => (self.coords.0, self.coords.1 - 1),
            Direction::RIGHT => (self.coords.0 + 1, self.coords.1),
            Direction::DOWN  => (self.coords.0, self.coords.1 + 1),
            Direction::LEFT  => (self.coords.0 - 1, self.coords.1),
            };

            if grid[coords.1][coords.0] == "#" {
                self.turn();
            }

            return true;
    }

    fn check_if_exit(&self, grid_len: &(usize, usize)) -> bool {
        return match self.direction {
            Direction::TOP   => if self.coords.1 == 0 { true } else { false },
            Direction::RIGHT => if self.coords.0 == grid_len.0 { true } else { false },
            Direction::DOWN  => if self.coords.1 == grid_len.1 { true } else { false },
            Direction::LEFT  => if self.coords.0 == 0 { true } else { false },
        };
    }

    fn check_path(&self, grid: &mut Vec<Vec<&str>>, traps: &mut Vec<(usize, usize)>) {
        match self.direction {
            Direction::TOP   => {
                let trap = (self.coords.0, self.coords.1 - 1);
                
                if grid[self.coords.1][(self.coords.0 + 1)..].contains(&">") {
                    if !traps.contains(&trap) {
                       traps.push(trap);
                    }
                }
                grid[self.coords.1][self.coords.0] = "^";
            },
            Direction::RIGHT => {
                for i in self.coords.1..grid.len() {
                    if grid[i][self.coords.0].contains(&"v") {
                        let trap = (self.coords.0 + 1, self.coords.1);
                        if !traps.contains(&trap) {
                           traps.push(trap);
                        }
                    }

                    if grid[i][self.coords.0].contains(&"#") {
                        return
                    }
                }
                
                if grid[self.coords.1 + 1][self.coords.0].contains("v") {
                    let trap = (self.coords.0 + 1, self.coords.1);
                    if !traps.contains(&trap) {
                       traps.push(trap);
                    }
                }
                grid[self.coords.1][self.coords.0] = ">";
            },
            Direction::DOWN => {
                if grid[self.coords.1][0..(self.coords.0 - 1)].contains(&"<") {
                    let trap = (self.coords.0, self.coords.1 + 1);
                    if !traps.contains(&trap) {
                       traps.push(trap);
                    }
                }
                grid[self.coords.1][self.coords.0] = "v";
            },

            Direction::LEFT => {
                for i in (0..grid.len() - 1).rev() {
                    if grid[i][self.coords.0].contains(&"^") {
                        let trap = (self.coords.0 - 1, self.coords.1);
                        if !traps.contains(&trap) {
                           traps.push(trap);
                        }
                    }

                    if grid[i][self.coords.0].contains(&"#") {
                        return
                    }
                }

                grid[self.coords.1][self.coords.0] = "<";
            },
        };
    }
}

fn get_direction(position: &str) -> Option<Direction>  { 
    return match position {
        "^" => Some(Direction::TOP),
        ">" => Some(Direction::RIGHT),
        "v" => Some(Direction::DOWN),
        "<" => Some(Direction::LEFT),
        _ => None
    }
}

fn draw(guard: &Guard, grid: &mut Vec<Vec<&str>>, traps_count: usize) {
    let mut stdout = stdout();
    stdout.flush().unwrap();

    let g = match guard.direction {
        Direction::TOP   => "^",
        Direction::RIGHT => ">", 
        Direction::DOWN  => "v", 
        Direction::LEFT  => "<",
    };

    grid[guard.coords.1][guard.coords.0] = g;

    println!("TRAPS: {:?}", traps_count);
    grid.into_iter().for_each(|l| println!("{:?}", l.join("")));

    let _ = stdout.execute(MoveUp(11));
}

fn main() {
    let mut file = File::open("./day6-1.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let mut grid: Vec<Vec<&str>> = text.split("\n").collect::<Vec<&str>>().iter().map(|l| l.split("").filter(|l| !l.is_empty()).collect()).collect();

    grid.pop();

    let grid_len: (usize, usize) = (grid[0].len() - 1, grid.len() - 1);

    let mut guard = Guard::new(&grid);

    let mut traps: Vec<(usize, usize)> = vec!();

    let mut running = true;

    while running {
        guard.check_path(&mut grid, &mut traps);
        running = guard.check_next_step(&grid, &grid_len);
        guard.walk();
        // draw(&guard, &mut grid.clone(), traps.len());
        // sleep(Duration::from_millis(500));
    };

    let mut stdout = stdout();
    let _ = stdout.execute(MoveDown(11));

    // 5312
    println!("TOTAL: {:?}", traps.len());
}
