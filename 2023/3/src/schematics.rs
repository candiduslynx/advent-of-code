use std::collections::HashSet;
use lib::point::Point;

#[derive(Debug)]
pub(crate) struct Schema {}

impl Schema {
    pub(crate) fn get_numbers(lines: Vec<String>) -> Vec<u32> {
        let symbols = Schema::get_symbols(&lines);

        let mut res: Vec<u32> = Vec::new();
        for i in 0..lines.len() {
            let x = i as i32;
            let mut num: (Option<u32>, bool) = (None, false); // 1 - val, 2 - valid
            let line = lines[i].as_bytes();
            for j in 0..line.len() {
                let c = line[j];

                if c.is_ascii_digit() {
                    num.0 = Some(num.0.unwrap_or_default() * 10 + ((c - b'0') as u32));
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

    fn get_symbols(lines: &Vec<String>) -> HashSet<Point> {
        let mut res:HashSet<Point> = HashSet::new();

        for i in 0..lines.len() {
            let x = i as i32;
            let line = lines[i].as_bytes();
            for j in 0..line.len() {
                let c = line[j];

                if c.is_ascii_digit() || c == b'.' {
                    continue;
                }

                res.insert(Point{x, y:j as i32});
            }
        }
        res
    }
}
