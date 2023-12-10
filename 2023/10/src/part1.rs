use std::fs::read;
use std::io::BufRead;

use lib::point::Point;

pub(crate) fn solve(path: &str) -> usize {
    let lines: Vec<Vec<char>> = read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    let start: (usize, usize) = lines
        .iter()
        .enumerate()
        .find_map(|(x, row)| {
            let y = row
                .iter()
                .enumerate()
                .find_map(|(y, c)| if 'S'.eq(c) { Some(y) } else { None });
            if y.is_none() {
                None
            } else {
                Some((x, y.unwrap()))
            }
        })
        .unwrap();

    ['|', '-', 'F', '7', 'J', 'L']
        .into_iter()
        .find_map(|c| loop_len(&lines, &start, c))
        .unwrap() / 2
}

fn loop_len(lines: &Vec<Vec<char>>, start: &(usize, usize), treat: char) -> Option<usize> {
    let start = Point {
        x: start.0 as isize,
        y: start.1 as isize,
    };
    let mut pos = start;
    let mut pipe = treat;
    let mut prev: Option<Point> = None;

    let (max_x, max_y) = (lines.len(), lines[0].len());

    let mut l = 0usize;

    while pos != start || l == 0 {
        l += 1;
        let next: Option<Point> = match pipe {
            '|' => match prev {
                None => Some(pos.below()),
                Some(_) => Some(Point {
                    x: (pos.x << 1) - prev.unwrap().x,
                    y: pos.y,
                }),
            },
            '-' => match prev {
                None => Some(pos.right()),
                Some(_) => Some(Point {
                    x: pos.x,
                    y: (pos.y << 1) - prev.unwrap().y,
                }),
            },
            'F' => match prev {
                None => Some(pos.right()),
                Some(_) => Some(Point {
                    x: (pos.x << 1) - prev.unwrap().x + 1, // below => prev.x = pos.x + 1, right => pos.x = prev.x
                    y: (pos.y << 1) - prev.unwrap().y + 1, // below => prev.y = pos.y, right => pos.y = prev.y + 1
                }),
            },
            '7' => match prev {
                None => Some(pos.below()),
                Some(_) => Some(Point {
                    x: (pos.x << 1) - prev.unwrap().x + 1, // below => prev.x = pos.x + 1, left => pos.x = prev.x
                    y: (pos.y << 1) - prev.unwrap().y - 1, // below => prev.y = pos.y, left => pos.y = prev.y - 1
                }),
            },
            'J' => match prev {
                None => Some(pos.above()),
                Some(_) => Some(Point {
                    x: (pos.x << 1) - prev.unwrap().x - 1, // above => prev.x = pos.x - 1, left => pos.x = prev.x
                    y: (pos.y << 1) - prev.unwrap().y - 1, // above => prev.y = pos.y, left => pos.y = prev.y - 1
                }),
            },
            'L' => match prev {
                None => Some(pos.right()),
                Some(_) => Some(Point {
                    x: (pos.x << 1) - prev.unwrap().x - 1, // above => prev.x = pos.x - 1, left => pos.x = prev.x
                    y: (pos.y << 1) - prev.unwrap().y + 1, // below => prev.y = pos.y, right => pos.y = prev.y + 1
                }),
            },
            _ => None,
        };
        if next.is_none() || !next.unwrap().is_valid(max_x, max_y) {
            return None;
        }

        (prev, pos) = (Some(pos), next.unwrap());
        pipe = lines[pos.x as usize][pos.y as usize];
        if pos == start { pipe = treat }
        // check that we can actually connect by matching pipe
        if prev.unwrap().x < pos.x { // pos below prev
            match pipe {
                '|' | 'L' | 'J' => {}
                _ => return None
            }
        } else if prev.unwrap().x > pos.x { // pos above prev
            match pipe {
                '|' | 'F' | '7' => {}
                _ => return None
            }
        } else if prev.unwrap().y < pos.y { // pos right to prev
            match pipe {
                '-' | 'J' | '7' => {}
                _ => return None
            }
        } else if prev.unwrap().y > pos.y { // pos left to prev
            match pipe {
                '-' | 'L' | 'F' => {}
                _ => return None
            }
        } else {
            return None;
        }
    }
    Some(l)
}
