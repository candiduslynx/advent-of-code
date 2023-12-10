use std::collections::HashSet;

use lib::point::Point;

pub(crate) fn get_loop(
    lines: &Vec<Vec<char>>,
    start: &(usize, usize),
    treat: char,
) -> Option<HashSet<Point>> {
    let start = Point {
        x: start.0 as isize,
        y: start.1 as isize,
    };
    let mut pos = start;
    let mut pipe = treat;
    let mut prev: Option<Point> = None;
    let (max_x, max_y) = (lines.len(), lines[0].len());
    let mut result: HashSet<Point> = HashSet::new();

    while pos != start || result.len() == 0 {
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

        result.insert(pos);
        (prev, pos) = (Some(pos), next.unwrap());
        pipe = lines[pos.x as usize][pos.y as usize];
        if pos == start {
            pipe = treat
        }
        // check that we can actually connect by matching pipe
        if prev.unwrap().x < pos.x {
            // pos below prev
            match pipe {
                '|' | 'L' | 'J' => {}
                _ => return None,
            }
        } else if prev.unwrap().x > pos.x {
            // pos above prev
            match pipe {
                '|' | 'F' | '7' => {}
                _ => return None,
            }
        } else if prev.unwrap().y < pos.y {
            // pos right to prev
            match pipe {
                '-' | 'J' | '7' => {}
                _ => return None,
            }
        } else if prev.unwrap().y > pos.y {
            // pos left to prev
            match pipe {
                '-' | 'L' | 'F' => {}
                _ => return None,
            }
        } else {
            return None;
        }
    }
    Some(result)
}
