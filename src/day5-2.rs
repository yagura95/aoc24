use std::io::Read;
use std::fs::File;
use std::collections::{ 
    HashMap,
    HashSet
};



// 87|25
// 87|37
// 25|37

fn parse_rule(rules: &mut HashMap<u64, HashSet<u64>>, rule: &str) {
    let a: Vec<_> = rule.split("|").map(|n| n.parse::<u64>().unwrap()).collect();
    rules.entry(a[1]).or_insert(HashSet::new()).insert(a[0]);
}

fn parse_update(update: &str) -> Vec<u64> {
    update.split(",").map(|n| n.parse::<u64>().unwrap()).collect()
}

fn check_update(update: &Vec<u64>, rules: &HashMap<u64, HashSet<u64>>) -> bool {
    for i in update.iter().enumerate() {
        for j in update[(i.0)..].iter().enumerate() {
            let a = rules.get(i.1);

            match a {
               Some(h) => {
                    if h.contains(j.1) {
                        return false 
                    }
               },
               None => { continue }
            }
        }
    }
     
    return true;
}

fn fix_update(update: &mut Vec<u64>, rules: &HashMap<u64, HashSet<u64>>) -> bool {
    for i in update.clone().iter().enumerate() {
        for j in update.clone()[(i.0)..].iter().enumerate() {
            let a = rules.get(i.1);

            match a {
               Some(h) => {
                    if h.contains(j.1) {
                        let tmp = update[i.0].clone();
                        update[i.0] = update[i.0+j.0].clone();
                        update[i.0+j.0] = tmp;

                        return fix_update(update, rules);
                    }
               },
               None => { continue }
            }
        }
    }
     
    return true;
}

fn get_middle_number(v: &Vec<u64>) -> u64 {
    v[(v.len() - 1) / 2]
}

fn main() {
    let mut file = File::open("day5-1.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let mut rules: HashMap<u64, HashSet<u64>> = HashMap::new();

    // switch from parsing rules to parsing updates
    let mut r = false;

    let mut total = 0;

    for line in text.lines() {
        if line.trim().len() < 1 {
            r = true;
            continue
        }

        if !r {
            parse_rule(&mut rules, line);
        } else {
            let mut u = parse_update(line);
            let correct = check_update(&u, &rules);

            if !correct {
                fix_update(&mut u, &rules);
                total += get_middle_number(&u);
            }
        }
    }

    println!("Total: {}", total);
}
