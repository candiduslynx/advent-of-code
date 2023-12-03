use std::collections::{HashMap, HashSet};

use lib::point::Point;

pub(crate) fn get_numbers(lines: Vec<String>) -> Vec<u64> {
    let symbols = get_symbols(&lines);
    let mut res: Vec<u64> = Vec::new();

    for i in 0..lines.len() {
        let x = i as i32;
        let mut num: (Option<u64>, bool) = (None, false); // 1 - val, 2 - valid
        let line = lines[i].as_bytes();
        for j in 0..line.len() {
            let c = line[j];

            if c.is_ascii_digit() {
                num.0 = Some(num.0.unwrap_or_default() * 10 + ((c - b'0') as u64));
                if !num.1 {
                    // calc once
                    num.1 = Point { x, y: j as i32 }.neighbors().iter().any(|p| symbols.contains(p));
                }

                continue;
            }
            if num.1 { // we set valid only if we hit a digit, so we can check just that
                // valid num, push to vec
                res.push(num.0.unwrap());
            }
            num = (None, false);
        }
        // num ends here
        if num.1 { // we set valid only if we hit a digit, so we can check just that
            // valid num, push to vec
            res.push(num.0.unwrap());
        }
    }

    return res;
}

pub(crate) fn get_gear_ratios(lines: Vec<String>) -> Vec<u64> {
    let gears = get_gears(&lines);
    let mut res: HashMap<Point, u64> = Default::default();

    for i in 0..lines.len() {
        let x = i as i32;
        let mut num: (Option<u64>, Option<Point>) = (None, None); // 1 - val, 2 - gear coord
        let line = lines[i].as_bytes();
        for j in 0..line.len() {
            let c = line[j];

            if c.is_ascii_digit() {
                num.0 = Some(num.0.unwrap_or_default() * 10 + ((c - b'0') as u64));
                if num.1.is_none() {
                    // calc once
                    let nn = Point { x, y: j as i32 }.neighbors();
                    let v = nn.iter().find(|&&p| gears.contains(&p));
                    if v.is_some() {
                        let q = *v.unwrap();
                        num.1 = Some(q.clone());
                    }
                }
                continue;
            }
            if num.1.is_some() { // we set gear info only for numbers, so we're OK to check only this
                let key = num.1.unwrap();
                let val = res.get(&key).unwrap_or(&1u64);
                res.insert(key, val * num.0.unwrap());
            }
            num = (None, None);
        }
        // num ends here
        if num.1.is_some() {
            let key = num.1.unwrap();
            let val = res.get(&key).unwrap_or(&1u64);
            res.insert(key, val * num.0.unwrap());
        }
    }
    res.values().map(|s|*s).collect()
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

fn get_gears(lines: &Vec<String>) -> HashSet<Point> {
    let mut res: HashSet<Point> = HashSet::new();

    for i in 0..lines.len() {
        let x = i as i32;
        let line = lines[i].as_bytes();
        for j in 0..line.len() {
            let c = line[j];

            if c != b'*' {
                continue;
            }

            // check that we have exactly 2 number neighbors

            // above (& below): check corners + middle.
            // if middle - only 1 neighbor
            // otherwise check corners
            let mut above = 0u8;
            if is_digit_pos(Point { x: x - 1, y: j as i32 }, lines) {
                above = 1;
            } else {
                if is_digit_pos(Point { x: x - 1, y: j as i32 - 1 }, lines) {
                    above += 1;
                }
                if is_digit_pos(Point { x: x - 1, y: j as i32 + 1 }, lines) {
                    above += 1;
                }
            }

            let mut below = 0u8;
            if below != 1 && is_digit_pos(Point { x: x + 1, y: j as i32 }, lines) {
                below = 1;
            } else {
                if is_digit_pos(Point { x: x + 1, y: j as i32 - 1 }, lines) {
                    below += 1;
                }
                if is_digit_pos(Point { x: x + 1, y: j as i32 + 1 }, lines) {
                    below += 1;
                }
            }

            // left & right neighbors can only be separate numbers
            let mut lr = 0u8;
            if is_digit_pos(Point { x, y: j as i32 - 1 }, lines) {
                lr += 1;
            }
            if is_digit_pos(Point { x, y: j as i32 + 1 }, lines) {
                lr += 1;
            }

            if above + below + lr == 2 {
                res.insert(Point { x, y: j as i32 });
            }
        }
    }
    res
}

fn is_digit_pos(p: Point, lines: &Vec<String>) -> bool {
    if !p.is_valid(lines.len() as i32, lines[0].as_bytes().len() as i32) {
        return false;
    }
    lines[p.x as usize].as_bytes()[p.y as usize].is_ascii_digit()
}
