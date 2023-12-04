use lib::point::Point;
use std::collections::HashSet;
use std::fs::read;
use std::io::BufRead;


pub(crate) fn solve(path: &str) -> u64 {
    get_numbers(
        read(path).unwrap().lines().map(|s|s.unwrap()).collect()
    ).iter().sum()
}

fn get_numbers(lines: Vec<String>) -> Vec<u64> {
    let symbols = get_symbols(&lines);
    let mut res: Vec<u64> = Vec::new();

    for i in 0..lines.len() {
        let x = i as i32;
        let mut num: Option<u64> = None; 
        let mut valid = false;
        let line = lines[i].as_bytes();
        for j in 0..line.len() {
            let c = line[j];

            if c.is_ascii_digit() {
                num = Some(num.unwrap_or_default() * 10 + ((c - b'0') as u64));
                if !valid {
                    // calc once
                    valid = Point { x, y: j as i32 }.neighbors().iter().any(|p| symbols.contains(p));
                }
                continue;
            }

            if valid { // we set valid only if we hit a digit, so we can check just that
                // valid num, push to vec
                res.push(num.unwrap());
            }

            (num, valid) = (None, false);
        }

        // num ends here
        if valid { // we set valid only if we hit a digit, so we can check just that
            // valid num, push to vec
            res.push(num.unwrap());
        }
    }

    res
}

fn get_symbols(lines: &Vec<String>) -> HashSet<Point> {
    let mut res: HashSet<Point> = HashSet::new();

    for i in 0..lines.len() {
        let x = i as i32;
        let line = lines[i].as_bytes();
        for j in 0..line.len() {
            let c = line[j];

            if c.is_ascii_digit() || c == b'.' {
                continue;
            }

            res.insert(Point { x, y: j as i32 });
        }
    }

    res
}