use std::io::{
    Read,
    stdout,
    Write
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

fn main() {
    let mut file = File::open("./test.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let mut grid: Vec<Vec<&str>> = text.split("\n").collect::<Vec<&str>>().iter().map(|l| l.split("").filter(|l| !l.is_empty()).collect()).collect();

    grid.pop();

    let grid_len: (usize, usize) = (grid[0].len() - 1, grid.len() - 1);

    let mut guard = Guard::new(&grid);

    fn draw(guard: &Guard, grid: &mut Vec<Vec<&str>>) {
        let mut stdout = stdout();
        stdout.flush().unwrap();

        let g = match guard.direction {
            Direction::TOP   => "^",
            Direction::RIGHT => ">", 
            Direction::DOWN  => "v", 
            Direction::LEFT  => "<",
        };

        grid[guard.coords.1][guard.coords.0] = g;

        grid.into_iter().for_each(|l| println!("{:?}", l.join("")));

        let _ = stdout.execute(MoveUp(10));
    };

    while guard.check_next_step(&grid, &grid_len) {
        grid[guard.coords.1][guard.coords.0] = "X";
        guard.walk();
        draw(&guard, &mut grid.clone());
        sleep(Duration::from_millis(500));
    };

    let g = grid.into_iter().flatten().filter(|s| s.to_string() == "X").collect::<Vec<_>>();

    let mut stdout = stdout();
    let _ = stdout.execute(MoveDown(10));

    // 5312
    println!("TOTAL: {:?}", g.len() + 1);
}
