use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read;
use std::io::BufRead;

use lib::point::Point;

pub(crate) fn solve(path: &str) -> u64 {
    get_gear_ratios(read(path).unwrap().lines().map(|s| s.unwrap()).collect())
        .iter()
        .sum()
}

fn get_gear_ratios(lines: Vec<String>) -> Vec<u64> {
    let gears = get_gears(&lines);
    let mut res: HashMap<Point, (u64, u64)> = HashMap::new();

    for i in 0..lines.len() {
        let x = i as isize;
        let line = lines[i].as_bytes();

        let mut num: Option<u64> = None;
        let mut num_gears: HashSet<Point> = HashSet::new();

        for j in 0..line.len() {
            let c = line[j];

            if c.is_ascii_digit() {
                num = Some(num.unwrap_or_default() * 10 + ((c - b'0') as u64));
                Point { x, y: j as isize }
                    .neighbors()
                    .iter()
                    .filter(|p| gears.contains(p))
                    .for_each(|p| {
                        num_gears.insert(*p);
                    });
                continue;
            }

            if num.is_some() {
                num_gears
                    .drain()
                    .for_each(|gear| add_gear_num(&mut res, num.unwrap(), &gear));
                num = None;
            }
        }

        // num ends here
        if num.is_some() {
            num_gears
                .drain()
                .for_each(|gear| add_gear_num(&mut res, num.unwrap(), &gear));
        }
    }

    res.values().map(|val| val.0 * val.1).collect()
}

fn add_gear_num(res: &mut HashMap<Point, (u64, u64)>, num: u64, p: &Point) {
    res.entry(*p).and_modify(|v| v.1 = num).or_insert((num, 0));
}

fn get_gears(lines: &Vec<String>) -> HashSet<Point> {
    let mut res: HashSet<Point> = HashSet::new();

    for i in 0..lines.len() {
        let x = i as isize;
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
            if is_digit_pos(
                Point {
                    x: x - 1,
                    y: j as isize,
                },
                lines,
            ) {
                above = 1;
            } else {
                if is_digit_pos(
                    Point {
                        x: x - 1,
                        y: j as isize - 1,
                    },
                    lines,
                ) {
                    above += 1;
                }
                if is_digit_pos(
                    Point {
                        x: x - 1,
                        y: j as isize + 1,
                    },
                    lines,
                ) {
                    above += 1;
                }
            }

            let mut below = 0u8;
            if below != 1
                && is_digit_pos(
                    Point {
                        x: x + 1,
                        y: j as isize,
                    },
                    lines,
                )
            {
                below = 1;
            } else {
                if is_digit_pos(
                    Point {
                        x: x + 1,
                        y: j as isize - 1,
                    },
                    lines,
                ) {
                    below += 1;
                }
                if is_digit_pos(
                    Point {
                        x: x + 1,
                        y: j as isize + 1,
                    },
                    lines,
                ) {
                    below += 1;
                }
            }

            // left & right neighbors can only be separate numbers
            let mut lr = 0u8;
            if is_digit_pos(
                Point {
                    x,
                    y: j as isize - 1,
                },
                lines,
            ) {
                lr += 1;
            }
            if is_digit_pos(
                Point {
                    x,
                    y: j as isize + 1,
                },
                lines,
            ) {
                lr += 1;
            }

            if above + below + lr == 2 {
                res.insert(Point { x, y: j as isize });
            }
        }
    }
    res
}

fn is_digit_pos(p: Point, lines: &Vec<String>) -> bool {
    if !p.is_valid(lines.len(), lines[0].as_bytes().len()) {
        return false;
    }
    lines[p.x as usize].as_bytes()[p.y as usize].is_ascii_digit()
}
