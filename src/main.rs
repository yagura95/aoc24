use std::io::Read;
use std::fs::File;

struct Region {
    flower: String,
    area: u64,
    perimeter: u64
}

fn main() {
    let mut file = File::open("./test.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    
    let garden: Vec<Vec<&str>> = text.split_whitespace().filter(|s| !s.is_empty()).map(|t| t.split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    let garden_size = (garden.len() - 1, garden[0].len() - 1);

    //println!("{garden:?}");
    let mut regions: Vec<Region> = vec!();

    fn check_corner(region: &(usize, usize), garden_size: &(usize, usize)) -> bool {
        // top-left
        if region.0 == 0 && region.1 == 0 {
            return true;
        } else if region.0 == 0 && region.1 == garden_size.1 {
            return true
        } else if region.0 == garden_size.0 && region.1 == 0 {
            return true;
        } else if region.0 == garden_size.0 && region.1 == garden_size.1 {
            return true
        }

        return false 
    }

    fn check_top(region: (usize, usize)) {
        // Check if there is a equal region on top
    }

    fn check_bottom() {}
    fn check_left() {}
    fn check_right() {}

    // 
    fn check_region() {
        // If in a corner, adds 2 to perimeter    
        // 
    }

    for i in garden.iter().enumerate() {
        for j in garden[i.0].iter().enumerate() {
            // Already checked
            if garden[i.0][j.0] == "0" {
                continue
            }

            // Change checked regions to "0"
        }
    }
}
